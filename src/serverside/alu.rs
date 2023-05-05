use std::error::Error;
use tfhe::FheUint8;
use tfhe::prelude::*;

/// Darstellung der ALU über vorgegebene Operationen, die mit selbst gewählten OpCodes
/// angesteuert werden können.
/// Aktuell existieren:
/// - Addition
/// - Binäres Und
/// - Binäres Oder
/// - Binäres XOr
pub struct Alu {
    pub(crate) opcode_add: FheUint8,
    pub(crate) opcode_and: FheUint8,
    pub(crate) opcode_or: FheUint8,
    pub(crate) opcode_xor: FheUint8,
}

impl Alu {
    /// Berechnet Anhang des übergebenen OpCodes das Ergebnis der beiden Operanden.
    /// Die Berechnung verfolgt ohne Verzweigung über die folgende Logik:
    /// `result = (add_result * op_code.eq(opcode_add)) + (and_result * op_code.eq(opcode_and)) + (or_result * op_code.eq(opcode_or)) + (xor_result * op_code.eq(opcode_xor))`
    /// 
    /// Soweit alle OpCodes richtig gesetzt sind und ein zulässiger op_code übergeben wird, wird immer ein Ergebnis berechnet.
    /// Sollten OpCodes falsch gesetzt sein, kann fälschlicherweise `0` berechnet werden.
    pub fn calculate(&self, op_code: FheUint8, a: FheUint8, b: FheUint8) -> Result<FheUint8, Box<dyn Error>> {
        // Addition
        let addition = (&a + &b) * op_code.eq(&self.opcode_add);
        let result = addition;

        // AND
        let and = (&a & &b) * op_code.eq(&self.opcode_and);
        let result = result + and;

        // OR
        let or = (&a | &b) * op_code.eq(&self.opcode_or);
        let result = result + or;

        // XOR
        let xor = (&a ^ &b) * op_code.eq(&self.opcode_xor);
        let result = result + xor;

        Ok(result)
    }
}
