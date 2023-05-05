use std::error::Error;
use tfhe::FheUint8;
use tfhe::prelude::*;

pub struct Alu {
    pub(crate) opcode_add: FheUint8,
    pub(crate) opcode_and: FheUint8,
    pub(crate) opcode_or: FheUint8,
    pub(crate) opcode_xor: FheUint8,
}

impl Alu {
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
