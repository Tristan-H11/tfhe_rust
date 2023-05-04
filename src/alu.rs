use std::error::Error;
use tfhe::FheUint8;
use tfhe::prelude::*;
use bincode;
use std::io::{Cursor};

pub fn start(data: &[u8]) -> Result<FheUint8, Box<dyn Error>> {
    let mut serialized_data = Cursor::new(data);

    let opcode_add: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_and: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_or: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_xor: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    let op_code: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let a: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let b: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    // Addition
    let addition = (&a + &b) * op_code.eq(opcode_add);
    let result = addition;

    // AND
    let and = (&a & &b) * op_code.eq(opcode_and);
    let result = result + and;

    // OR
    let or = (&a | &b) * op_code.eq(opcode_or);
    let result = result + or;

    // XOR
    let xor = (&a ^ &b) * op_code.eq(opcode_xor);
    let result = result + xor;

    Ok(result)
}
