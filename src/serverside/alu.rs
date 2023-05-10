use std::error::Error;
use tfhe::FheUint16;

use tfhe::prelude::*;

/// Darstellung der ALU über vorgegebene Operationen, die mit selbst gewählten OpCodes
/// angesteuert werden können.
/// Aktuell existieren:
/// - Addition
/// - Binäres Und
/// - Binäres Oder
/// - Binäres XOr
pub struct Alu {
    pub(crate) opcode_add: FheUint16,
    pub(crate) opcode_and: FheUint16,
    pub(crate) opcode_or: FheUint16,
    pub(crate) opcode_xor: FheUint16,
    pub(crate) zero_flag: FheUint16,
    pub(crate) overflow_flag: FheUint16,
    pub(crate) carry_flag: FheUint16
}

impl Alu {
    /// Berechnet Anhang des übergebenen OpCodes das Ergebnis der beiden Operanden.
    /// Die Berechnung verfolgt ohne Verzweigung über die folgende Logik:
    /// `result = (add_result * op_code.eq(opcode_add)) + (and_result * op_code.eq(opcode_and)) + (or_result * op_code.eq(opcode_or)) + (xor_result * op_code.eq(opcode_xor))`
    ///
    /// Soweit alle OpCodes richtig gesetzt sind und ein zulässiger op_code übergeben wird, wird immer ein Ergebnis berechnet.
    /// Sollten OpCodes falsch gesetzt sein, kann fälschlicherweise `0` berechnet werden.
    pub fn calculate(&mut self, op_code: FheUint16, a: FheUint16, b: FheUint16) -> Result<FheUint16, Box<dyn Error>> {
        // Addition
        let is_addition: FheUint16 = op_code.eq(&self.opcode_add);
        let addition = (&a + &b + &self.carry_flag) * is_addition;
        let result = addition;

        // AND
        let is_and: FheUint16 = op_code.eq(&self.opcode_and);
        let and = (&a & &b) * is_and;
        let result = result + and;

        // OR
        let is_or: FheUint16 = op_code.eq(&self.opcode_or);
        let or = (&a | &b) * is_or;
        let result = result + or;

        // XOR
        let is_xor: FheUint16 = op_code.eq(&self.opcode_xor);
        let xor = (&a ^ &b) * is_xor;
        let result = result + xor;

        // Zero-Flag
        self.zero_flag = result.eq(&FheUint16::try_encrypt_trivial(0u8).unwrap());
        self.set_overflow(a.clone(), b.clone(), result.clone());
        self.set_carry(a.clone(), b.clone());

        println!("Alu Berechnung abgeschlossen.");
        Ok(result)
    }

    // TODO
    //  Die beiden Funktionen hier drunter sind noch ungeprüft und ich bin nicht ganz sicher, ob die korrekt sind!!!!
    //  Und es ist noch unklar, wann welche Flags gesetzt werden sollen.

    /// Wenn die beiden MSB's ver-xort werden und dieses Ergebnis ungleich dem Ergebnis MSB ist,
    /// dann gab es einen Carry an vorletzter Stelle, also einen Overflow. <br>
    /// Overflow = (A_msb ^ B_msb) ^ Result_msb
    fn set_overflow(&mut self, a: FheUint16, b: FheUint16, result: FheUint16) {
        let negate_mask: FheUint16 = FheUint16::try_encrypt_trivial(0b0000_0001 as u16).unwrap();
        let msb_mask: FheUint16 = FheUint16::try_encrypt_trivial(0b1000_0000 as u16).unwrap();
        let masked_a: FheUint16 = &a & &msb_mask;
        let masked_b: FheUint16 = &b & &msb_mask;
        let masked_result: FheUint16 = &result & &msb_mask;

        // Ab hier steht der Wert, ob sie gleich oder ungleich sind im LSB
        let equal: FheUint16 = (masked_a ^ masked_b).eq(masked_result);
        // Overflow setzen, wenn sie UNGLEICH sind. Also !equal
        self.overflow_flag = &equal ^ &negate_mask;
    }

    /// Ein Carry im letzten Bit gibt es, wenn die MSB der beiden Operanden ungleich sind UND es einen overflow gab
    /// ODER wenn die beiden MSB ver-undet 1 ergeben.
    /// Carry = (((A_msb ^ B_msb).eq(msb_mask) & self.overflow_flag) | (A_msb.eq(B_msb)
    fn set_carry(&mut self, a: FheUint16, b: FheUint16) {
        let msb_mask: FheUint16 = FheUint16::try_encrypt_trivial(0b1000_0000 as u16).unwrap();
        let masked_a: FheUint16 = &a & &msb_mask;
        let masked_b: FheUint16 = &b & &msb_mask;

        let a_xor_b_masked: FheUint16 = &masked_a ^ &masked_b;
        let not_equal: FheUint16 = a_xor_b_masked.eq(msb_mask);

        let left_side: FheUint16 = not_equal & &self.overflow_flag;
        let right_side: FheUint16 = masked_a.eq(&masked_b);

        self.carry_flag = left_side | right_side;
    }
}
