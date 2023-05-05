use std::error::Error;
use std::fs::File;
use tfhe::{ClientKey, FheUint8};
use tfhe::prelude::*;
use bincode;
use std::io::{Read};

/// Verify-Main-Funktion.
/// Hier wird das Ergebnis ausgelesen, entschlüsselt und zur Überprüfung ausgegeben.
pub fn start() -> Result<(), Box<dyn Error>> {
    // Ergebnis einlesen und deserialisieren
    let mut calculated_result = Vec::new();
    let mut file = File::open("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\calculated_result.bin")
        .expect("Konnte datei nicht öffnen!");
    file.read_to_end(&mut calculated_result).expect("konnte datei nicht auslesen");
    let deserialized_result: FheUint8 = bincode::deserialize(&calculated_result)?;

    // PrivateKey einlesen
    let mut serialized_private_key = Vec::new();
    let mut file = File::open("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\private_key.bin")
        .expect("Konnte datei nicht öffnen!");
    file.read_to_end(&mut serialized_private_key).expect("konnte datei nicht auslesen");
    let private_key: ClientKey = bincode::deserialize(&serialized_private_key)?;

    let dec_result: u8 = deserialized_result.decrypt(&private_key);
    println!("Decrypted Result: {}", dec_result);
    Ok(())
}
