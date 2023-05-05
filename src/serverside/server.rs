use std::error::Error;
use crate::serverside::alu;

use std::fs::File;
use tfhe::{FheUint8, ServerKey, set_server_key};

use bincode;
use std::io::{Cursor, Read, Write};
use tfhe::prelude::FheTryTrivialEncrypt;
use crate::serverside::alu::Alu;

///
/// Hier werden die Berechnungen auf den verschlüsselten Daten getätigt.
/// Am Ende soll das hier die gesamte CPU-Simulation sein, auf der am Ende alles ausgelesen wird
///

pub fn start() -> Result<(), Box<dyn Error>> {
//////// Server Key einlesen
    let mut serialized_server_key = Vec::new();
    let mut file = File::open("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\server_key.bin")
        .expect("Konnte datei nicht öffnen!");
    file.read_to_end(&mut serialized_server_key).expect("konnte datei nicht auslesen");
    let server_key: ServerKey = bincode::deserialize(&serialized_server_key)?;

    set_server_key(server_key);

//////// Daten einlesen
    let mut data = Vec::new();
    let mut file = File::open("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\data.bin")
        .expect("Konnte datei nicht öffnen!");
    file.read_to_end(&mut data).expect("konnte datei nicht auslesen");


    let mut serialized_data = Cursor::new(data);

//////// ALU konstruieren
    let opcode_add: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_and: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_or: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_xor: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    let alu = Alu {
        opcode_add,
        opcode_and,
        opcode_or,
        opcode_xor
    };

//////// Memory-Access konstruieren
    let ram_read: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let ram_write: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    //TODO: Memory bauen und einbinden



    let op_code: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let a: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let b: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;


//////// Ergebnis berechnen
    let result = alu.calculate(op_code, a, b).expect("ALU-Berechnung fehlgeschlagen!");

//////// Ergebnis serialisiert abspeichern
    let serialized_result = bincode::serialize(&result)?;
    let mut file = File::create("C:\\Users\\tridd\\IdeaProjects\\tfhe_rust\\src\\calculated_result.bin").expect("Datei erstellen fehlgeschlagen!");
    file.write_all(serialized_result.as_slice()).expect("Result konnte nicht geschrieben werden!");
    Ok(())
}
