use std::error::Error;
use std::fs::File;
use std::io::Read;

use bincode;
use tfhe::{ClientKey, FheUint16, FheUint8};
use tfhe::prelude::*;

/// Verify-Main-Funktion.
/// Hier wird das Ergebnis ausgelesen, entschlüsselt und zur Überprüfung ausgegeben.
pub fn start() -> Result<(), Box<dyn Error>> {
    // Ergebnis einlesen und deserialisieren
    let mut calculated_result = Vec::new();
    let mut file = File::open("calculated_result.bin")?;
    file.read_to_end(&mut calculated_result)?;

    let deserialized_result: Vec<(FheUint8, FheUint8)> = bincode::deserialize(&calculated_result)?;

    // PrivateKey einlesen
    let mut serialized_private_key = Vec::new();
    let mut file = File::open("private_key.bin")?;
    file.read_to_end(&mut serialized_private_key)?;
    let private_key: ClientKey = bincode::deserialize(&serialized_private_key)?;


    let result_ram: Vec<(u8, u8)> = deserialized_result.iter().map(
        |(x, y): &(FheUint8, FheUint8)| (x.decrypt(&private_key), y.decrypt(&private_key))
    ).collect();

    for (i, x) in result_ram.iter().enumerate() {
        println!("Zeile {}: OpCode {} --- Wert {}", i, x.0, x.1);
    }

    Ok(())
}
