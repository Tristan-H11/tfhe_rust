use tfhe::{ConfigBuilder, FheUint8, generate_keys, ServerKey, set_server_key};
use tfhe::prelude::*;

pub fn start() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let (client_key, server_key) = generate_keys(config);

    let mut memory: Vec<FheUint8> = Vec::new();

    for i in 0..8 {
        memory.append(&mut vec![FheUint8::encrypt(i * 2, &client_key)]);
        println!("Memory {} geschrieben!", i);
    }
    /*
        Memory = {0,2,4,6,8,10,12,14}
     */

    let target_index = FheUint8::encrypt(4, &client_key);

    // Serverside
    set_server_key(server_key);

    let mut result = FheUint8::encrypt(0, &client_key);

    for (i, value) in memory.iter().enumerate() {
        let index = FheUint8::encrypt(i as u8, &client_key);
        let equal = index.eq(&target_index);

        result = result + (value * equal);
    }


    // Clientside
    let result_decrypted: u8 = result.decrypt(&client_key);
    println!("Das Ergebnis ist {}", result_decrypted);
}
