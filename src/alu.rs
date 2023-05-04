use tfhe::{ConfigBuilder, FheUint8, generate_keys, ServerKey, set_server_key};
use tfhe::prelude::*;

pub fn start() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

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
    let clear_op_code = 2u8;
    let op_code = FheUint8::encrypt(clear_op_code, &client_key);

    let clear_a = 2;
    let clear_b = 3;

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    let mut result = FheUint8::encrypt(0, &client_key);

///////// Serverside /////////

    // Addition
    let addition = (&a + &b) * op_code.eq(opcode_add);
    let result = result + addition;

    // AND
    let and = (&a & &b) * op_code.eq(opcode_and);
    let result = result + and;

    // OR
    let or = (&a | &b) * op_code.eq(opcode_or);
    let result = result + or;

    // XOR
    let xor = (&a ^ &b) * op_code.eq(opcode_xor);
    let result = result + xor;


/////////// Clientside /////////
    let dec_result: u8 = result.decrypt(&client_key);

    assert_eq!(dec_result, 3);
    println!("Decrypted Result: {}; Clear Result: {}", dec_result, 3);
}
