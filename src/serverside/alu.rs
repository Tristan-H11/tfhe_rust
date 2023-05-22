use std::time::Instant;
use tfhe::FheUint8;
use tfhe::prelude::*;
use crate::serverside::opcode_container_alu::OpcodeContainerAlu;

/// Darstellung der ALU über vorgegebene Operationen, die mit selbst gewählten OpCodes
/// angesteuert werden können.
/// Aktuell existieren:
/// - Addition
/// - Binäres Und
/// - Binäres Oder
/// - Binäres XOr
pub struct Alu {
    pub(crate) opcodes: OpcodeContainerAlu,
    pub(crate) zero_flag: FheUint8,
    pub(crate) overflow_flag: FheUint8,
    pub(crate) carry_flag: FheUint8
}

impl Alu {
    /// Berechnet Anhang des übergebenen OpCodes das Ergebnis der beiden Operanden.
    /// Die Berechnung verfolgt ohne Verzweigung über die folgende Logik:
    /// `result = (add_result * op_code.eq(opcode_add)) + (and_result * op_code.eq(opcode_and)) + (or_result * op_code.eq(opcode_or)) + (xor_result * op_code.eq(opcode_xor))`
    ///
    /// Soweit alle OpCodes richtig gesetzt sind und ein zulässiger op_code übergeben wird, wird immer ein Ergebnis berechnet.
    /// Sollten OpCodes falsch gesetzt sein, kann fälschlicherweise `0` berechnet werden.
    pub fn calculate(&mut self, op_code: &FheUint8, operand: &FheUint8, accu: &FheUint8, is_alu_command: &FheUint8) -> FheUint8 {
        let start_time = Instant::now();
        // Addition
        let is_addition: FheUint8 = self.opcodes.is_add(&op_code);
        let addition = (operand + accu) * is_addition;
        let result = addition;

        // AND
        let is_and: FheUint8 = self.opcodes.is_and(&op_code);
        let and = (operand & accu) * is_and;
        let result = result + and;

        // OR
        let is_or: FheUint8 = self.opcodes.is_or(&op_code);
        let or = (operand | accu) * is_or;
        let result = result + or;

        // XOR
        let is_xor: FheUint8 = self.opcodes.is_xor(&op_code);
        let xor = (operand ^ accu) * is_xor;
        let result = result + xor;

        let is_sub: FheUint8 = self.opcodes.is_sub(&op_code);
        let sub = (accu - operand) * is_sub;
        let result = result + sub;

        let is_mul: FheUint8 = self.opcodes.is_mul(&op_code);
        let mul = (operand * accu) * is_mul;
        let result = result + mul;

        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        // Zero-Flag
        self.zero_flag = result.eq(&FheUint8::try_encrypt_trivial(0u8).unwrap());
        let new_overflow_flag: FheUint8 = self.calculate_overflow(operand, accu, &result);
        self.overflow_flag = new_overflow_flag * is_alu_command + &self.overflow_flag * (&one - is_alu_command);

        let new_carry_flag: FheUint8 = self.calculate_carry(operand, accu);
        self.carry_flag = new_carry_flag * is_alu_command + &self.carry_flag * (&one - is_alu_command);

        println!("[ALU, {}ms] Berechnung und Flags abgeschlossen.", start_time.elapsed().as_millis());
        result
    }

    // TODO
    //  Die beiden Funktionen hier drunter sind noch ungeprüft und ich bin nicht ganz sicher, ob die korrekt sind!!!!
    //  Und es ist noch unklar, wann welche Flags gesetzt werden sollen.

    /// Wenn die beiden MSB's ver-xort werden und dieses Ergebnis ungleich dem Ergebnis MSB ist,
    /// dann gab es einen Carry an vorletzter Stelle, also einen Overflow. <br>
    /// Overflow = (A_msb ^ B_msb) ^ Result_msb
    fn calculate_overflow(&mut self, a: &FheUint8, b: &FheUint8, result: &FheUint8) -> FheUint8 {
        let negate_mask: &FheUint8 = &FheUint8::try_encrypt_trivial(0b0000_0001 as u8).unwrap();
        let msb_mask: &FheUint8 = &FheUint8::try_encrypt_trivial(0b1000_0000 as u8).unwrap();
        let masked_a: FheUint8 = a & msb_mask;
        let masked_b: FheUint8 = b & msb_mask;
        let masked_result: FheUint8 = result & msb_mask;

        // Ab hier steht der Wert, ob sie gleich oder ungleich sind im LSB
        let equal: FheUint8 = (masked_a ^ masked_b).eq(masked_result);
        // Overflow setzen, wenn sie UNGLEICH sind. Also !equal
        &equal ^ negate_mask
    }

    /// Ein Carry im letzten Bit gibt es, wenn die MSB der beiden Operanden ungleich sind UND es einen overflow gab
    /// ODER wenn die beiden MSB ver-undet 1 ergeben.
    /// Carry = (((A_msb ^ B_msb).eq(msb_mask) & self.overflow_flag) | (A_msb.eq(B_msb)
    fn calculate_carry(&mut self, a: &FheUint8, b: &FheUint8) -> FheUint8 {
        let msb_mask: &FheUint8 = &FheUint8::try_encrypt_trivial(0b1000_0000 as u8).unwrap();
        let masked_a: FheUint8 = a & msb_mask;
        let masked_b: FheUint8 = b & msb_mask;

        let a_xor_b_masked: FheUint8 = &masked_a ^ &masked_b;
        let not_equal: FheUint8 = a_xor_b_masked.eq(msb_mask);

        let left_side: FheUint8 = not_equal & &self.overflow_flag;
        let right_side: FheUint8 = masked_a.eq(&masked_b);

        left_side | right_side
    }
}
