use std::error::Error;
use std::fs::File;
use tfhe::{ConfigBuilder, FheUint8, generate_keys, GenericInteger};
use tfhe::prelude::*;
use bincode;
use std::io::{Write};
use crate::clientside::statics::*;

/// Client-Main-Funktion.
/// Hier werden:
/// - die Schlüssel erstellt,
/// - die Schlüssel serialisiert,
/// - die Maschinensprache konfiguriert,
/// - die OpCodes der Maschinensprache verschlüsselt und serialisiert,
/// - die Operanden und der OpCode für die Alu verschlüsselt und serialisiert. (Das wird sich demnächst ändern, wenn die CU gebaut wird)
pub fn start() -> Result<(), Box<dyn Error>> {

    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();
    let (client_key, server_key) = generate_keys(config);

    // ServerKey speichern
    let mut serialized_server_key = Vec::new();
    bincode::serialize_into(&mut serialized_server_key, &server_key)?;

    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\server_key.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_server_key.as_slice()).expect("ServerKey konnte nicht geschrieben werden!");


    // ClientKey speichern
    let mut serialized_private_key = Vec::new();
    bincode::serialize_into(&mut serialized_private_key, &client_key)?;

    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\private_key.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_private_key.as_slice()).expect("PrivateKey konnte nicht geschrieben werden!");


    // Daten speichern
    let data: Vec<u8> = vec![
        ALU_ADD,
        ALU_AND,
        ALU_OR,
        ALU_XOR,
        RAM_READ,
        RAM_WRITE,
        OP_CODE,
        OP_A,
        OP_B
    ];

    // Alle Werte im Vector verschlüsseln
    let encrypted_data: Vec<FheUint8> = data.iter()
        .map(|&x : &u8| FheUint8::encrypt(x, &client_key))
        .collect();

    let mut serialized_data = Vec::new();
    for encrypted_value in encrypted_data {
        bincode::serialize_into(&mut serialized_data, &encrypted_value)?;
    }

    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\data.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_data.as_slice()).expect("Konnte Daten nicht in die Datei schreiben!");

    Ok(())
}
