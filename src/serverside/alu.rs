use std::time::Instant;

use tfhe::{FheBool, FheUint8};
use tfhe::prelude::*;

use crate::encrypt_trivial;
use crate::serverside::opcode_container_alu::OpcodeContainerAlu;

///
/// Darstellung der ALU.
///
/// # Flags
/// * Zero-Flag: Wird gesetzt, wenn das Ergebnis der Berechnung 0 ist.
/// * Overflow-Flag: Wird gesetzt, wenn das Ergebnis der Berechnung einen Überlauf verursacht hat.
/// * Carry-Flag: Wird gesetzt, wenn das Ergebnis der Berechnung einen Carry verursacht hat.
///
pub struct Alu {
    pub(crate) opcodes: OpcodeContainerAlu,
    pub(crate) zero_flag: FheBool,
    pub(crate) overflow_flag: FheBool,
    pub(crate) carry_flag: FheBool,
}

impl Alu {
    /// Berechnet Anhang des übergebenen OpCodes das Ergebnis der beiden Operanden.
    /// Die Berechnung verfolgt ohne Verzweigung über die folgende Logik (mit add, and, or sowie xor dargestellt):
    /// `result = (add_result * op_code.eq(opcode_add)) + (and_result * op_code.eq(opcode_and)) + (or_result * op_code.eq(opcode_or)) + (xor_result * op_code.eq(opcode_xor))`
    ///<br><br>
    /// Aktuell berechnet die Alu `ADD`, `AND`, `OR`, `XOR`, `SUB` und `MUL`.
    /// <br><br>
    /// Soweit alle OpCodes richtig gesetzt sind und ein zulässiger op_code übergeben wird, wird immer ein Ergebnis berechnet.
    /// Sollten OpCodes falsch gesetzt sein oder ein ungültiger Opcode übergeben werden, kann fälschlicherweise `0` berechnet werden.
    pub fn calculate(
        &mut self,
        op_code: &FheUint8,
        operand: &FheUint8,
        accu: &FheUint8,
        is_alu_command: &FheBool,
    ) -> FheUint8 {
        let start_time = Instant::now();

        let (add_and_result, (or_xor_result, mul_sub_result)): (FheUint8, (FheUint8, FheUint8)) =
            rayon::join(
                || {
                    let opcodes = self.opcodes.clone();
                    let op_code = op_code.clone();
                    let operand = operand.clone();
                    let accu = accu.clone();

                    let is_addition: FheUint8 = opcodes.is_add(&op_code);
                    let is_and: FheUint8 = opcodes.is_and(&op_code);
                    (&operand + &accu) * is_addition + (operand & accu) * is_and
                },
                // Hier muss ein bisschen geschummelt werden, weil ein Join nur zwei Rückgabetypen akzeptiert.
                // Deshalb ist es ein geschachteltes Join und der zweite Eintrag des Ergebnisses ist selber ein Tupel
                || {
                    rayon::join(
                        || {
                            let opcodes = self.opcodes.clone();
                            let op_code = op_code.clone();
                            let operand = operand.clone();
                            let accu = accu.clone();

                            let is_or: FheUint8 = opcodes.is_or(&op_code);
                            let is_xor: FheUint8 = opcodes.is_xor(&op_code);
                            (&operand | &accu) * is_or + (operand ^ accu) * is_xor
                        },
                        || {
                            let opcodes = self.opcodes.clone();
                            let op_code = op_code.clone();
                            let operand = operand.clone();
                            let accu = accu.clone();

                            let is_mul: FheUint8 = opcodes.is_mul(&op_code);
                            let is_sub: FheUint8 = opcodes.is_sub(&op_code);
                            (&operand * &accu) * is_mul + (accu - operand) * is_sub
                        },
                    )
                },
            );

        let result = add_and_result + or_xor_result + mul_sub_result;

        // Zero-Flag
        self.zero_flag = result.eq(&encrypt_trivial!(0u8));

        let new_overflow_flag: FheBool = self.calculate_overflow(operand, accu, &result);
        self.overflow_flag = is_alu_command.if_then_else(&new_overflow_flag, &self.overflow_flag);

        let new_carry_flag: FheBool = self.calculate_carry(operand, accu);
        self.carry_flag = is_alu_command.if_then_else(&new_carry_flag, &self.carry_flag);

        println!(
            "[ALU, {}ms] Berechnung und Flags abgeschlossen.",
            start_time.elapsed().as_millis()
        );
        result
    }

    ///
    /// Wenn die beiden MSB's ver-xort werden und dieses Ergebnis ungleich dem Ergebnis MSB ist,
    /// dann gab es einen Carry an vorletzter Stelle, also einen Overflow. <br>
    /// `Overflow = (A_msb ^ B_msb) ^ Result_msb`
    ///
    fn calculate_overflow(&mut self, a: &FheUint8, b: &FheUint8, result: &FheUint8) -> FheBool {
        let negate_mask: &FheUint8 = &encrypt_trivial!(0b0000_0001u8);
        let msb_mask: &FheUint8 = &encrypt_trivial!(0b1000_0000u8);
        let masked_a: FheUint8 = a & msb_mask;
        let masked_b: FheUint8 = b & msb_mask;
        let masked_result: FheUint8 = result & msb_mask;

        // Ab hier steht der Wert, ob sie gleich oder ungleich sind im LSB
        let equal: FheBool = (masked_a ^ masked_b).eq(masked_result);
        // Overflow setzen, wenn sie UNGLEICH sind. Also !equal
        !equal
    }

    ///
    /// Ein Carry im letzten Bit gibt es, wenn die MSB der beiden Operanden ungleich sind UND es einen overflow gab
    /// ODER wenn die beiden MSB ver-undet 1 ergeben. <br>
    /// `Carry = (((A_msb ^ B_msb).eq(msb_mask) & self.overflow_flag) | (A_msb.eq(B_msb)`
    ///
    fn calculate_carry(&mut self, a: &FheUint8, b: &FheUint8) -> FheBool {
        let msb_mask: &FheUint8 = &encrypt_trivial!(0b1000_0000u8);
        let masked_a: FheUint8 = a & msb_mask;
        let masked_b: FheUint8 = b & msb_mask;

        let a_xor_b_masked: FheUint8 = &masked_a ^ &masked_b;
        let not_equal: FheBool = a_xor_b_masked.eq(msb_mask);

        let left_side: FheBool = not_equal & &self.overflow_flag;
        let right_side: FheBool = masked_a.eq(&masked_b);

        left_side | right_side
    }
}
