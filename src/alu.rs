use std::error::Error;
use tfhe::{ConfigBuilder, FheUint8, generate_keys, ServerKey, set_server_key};
use tfhe::prelude::*;
use bincode;
use std::io::Cursor;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let (client_key, server_keys) = generate_keys(config);

    /* Op-Code:
    00 (0) = Add
    01 (1) = AND
    10 (2) = OR
    11 (3) = XOR
     */
    let opcode_add = FheUint8::encrypt(0, &client_key);
    let opcode_and = FheUint8::encrypt(1, &client_key);
    let opcode_or = FheUint8::encrypt(2, &client_key);
    let opcode_xor = FheUint8::encrypt(3, &client_key);
    // Gewünschter OpCode
    let op_code = FheUint8::encrypt(1, &client_key);

    // Operanden a und b
    let a = FheUint8::encrypt(2, &client_key);
    let b = FheUint8::encrypt(3, &client_key);

    let mut serialized_data = Vec::new();
    bincode::serialize_into(&mut serialized_data, &server_keys)?;
    bincode::serialize_into(&mut serialized_data, &opcode_add)?;
    bincode::serialize_into(&mut serialized_data, &opcode_and)?;
    bincode::serialize_into(&mut serialized_data, &opcode_or)?;
    bincode::serialize_into(&mut serialized_data, &opcode_xor)?;
    bincode::serialize_into(&mut serialized_data, &op_code)?;
    bincode::serialize_into(&mut serialized_data, &a)?;
    bincode::serialize_into(&mut serialized_data, &b)?;

    /*
    Hier werden die serialisierten Daten an den Server geben und dort wird gerechnet.
    Das ist analog zur Dateischnittstelle aus dem C-Projekt.
     */
    let calculated_result = server_function(&serialized_data)?;
    let deserialized_result: FheUint8 = bincode::deserialize(&calculated_result)?;

    let dec_result: u8 = deserialized_result.decrypt(&client_key);

    println!("Decrypted Result: {}", dec_result);
    Ok(())
}

fn server_function(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let mut serialized_data = Cursor::new(data);
    let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data)?;

    set_server_key(server_key);

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

    let serialized_result = bincode::serialize(&result)?;
    Ok(serialized_result)
}
