use std::error::Error;
use std::fs::File;
use tfhe::{ConfigBuilder, FheUint8, generate_keys};
use tfhe::prelude::*;
use bincode;
use std::io::{Write};

///
/// Hier werden die Client-Berechnungen wie Schüsselerstellung, das Festlegen der opcodes etc vorgenommen.
///
pub fn start() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let (client_key, server_key) = generate_keys(config);

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
    let op_code = FheUint8::encrypt(3, &client_key);

    // Operanden a und b
    let a = FheUint8::encrypt(2, &client_key);
    let b = FheUint8::encrypt(3, &client_key);

    let mut serialized_server_key = Vec::new();
    bincode::serialize_into(&mut serialized_server_key, &server_key)?;

    let mut serialized_private_key = Vec::new();
    bincode::serialize_into(&mut serialized_private_key, &client_key)?;

    let mut serialized_data = Vec::new();
    bincode::serialize_into(&mut serialized_data, &opcode_add)?;
    bincode::serialize_into(&mut serialized_data, &opcode_and)?;
    bincode::serialize_into(&mut serialized_data, &opcode_or)?;
    bincode::serialize_into(&mut serialized_data, &opcode_xor)?;
    bincode::serialize_into(&mut serialized_data, &op_code)?;
    bincode::serialize_into(&mut serialized_data, &a)?;
    bincode::serialize_into(&mut serialized_data, &b)?;

//////// ServerKey speichern
    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\server_key.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_server_key.as_slice()).expect("ServerKey konnte nicht geschrieben werden!");

//////// ClientKey speichern
    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\private_key.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_private_key.as_slice()).expect("PrivateKey konnte nicht geschrieben werden!");

//////// Daten speichern
    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\data.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_data.as_slice()).expect("Konnte Daten nicht in die Datei schreiben!");

    /*
    Datenformat: { opcode_add , opcode_and , opcode_or , opcode_xor , op_code , a , b }
     */
    Ok(())
}
