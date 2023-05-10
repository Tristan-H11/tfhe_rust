use std::error::Error;
use std::fs::File;
use std::io::Read;

use bincode;
use tfhe::{ClientKey, FheUint16};
use tfhe::prelude::*;

/// Verify-Main-Funktion.
/// Hier wird das Ergebnis ausgelesen, entschlüsselt und zur Überprüfung ausgegeben.
pub fn start() -> Result<(), Box<dyn Error>> {
    // Ergebnis einlesen und deserialisieren
    let mut calculated_result = Vec::new();
    let mut file = File::open("calculated_result.bin")?;
    file.read_to_end(&mut calculated_result)?;

    // TODO: Das hier muss auf nen ganzen VEktor unbekannter Länge ergänzt werden,
    //  damit dynamisch viele Ergebnisse zurückgegeben werden können.
    let deserialized_result: FheUint16 = bincode::deserialize(&calculated_result)?;

    // PrivateKey einlesen
    let mut serialized_private_key = Vec::new();
    let mut file = File::open("private_key.bin")?;
    file.read_to_end(&mut serialized_private_key)?;
    let private_key: ClientKey = bincode::deserialize(&serialized_private_key)?;

    let dec_result: u16 = deserialized_result.decrypt(&private_key);
    println!("Decrypted Result: {}", dec_result);
    
    Ok(())
}
