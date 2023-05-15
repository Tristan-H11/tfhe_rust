use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};

use crate::serverside::control_unit;

/// Server-Main-Funktion.
/// Hier werden:
/// - Der ServerKey eingelesen und gesetzt,
/// - die Daten vom Client eingelesen und deserialisiert,
/// - der OpCode und die Operanden von der ALU berechnet und
/// - das Ergebnis wieder abgespeichert.
///
/// Später sollen die Eingaben des Nutzers in einer Struktur gespeichert werden, die einen
/// Program-RAM und einen Program-Counter simulieren, damit "richtige" Programmabläufe möglich
/// werden.
pub fn start() -> Result<(), Box<dyn Error>> {

    // Server Key einlesen
    let mut serialized_server_key = Vec::new();
    let mut file = File::open("server_key.bin")?;
    file.read_to_end(&mut serialized_server_key)?;
    let server_key: ServerKey = bincode::deserialize(&serialized_server_key)?;

    set_server_key(server_key);
    println!("ServerKey eingelesen und gesetzt.");

    // Daten einlesen
    let mut configuration_data = Vec::new();
    let mut file = File::open("config_data.bin")?;
    file.read_to_end(&mut configuration_data)?;

    let mut serialized_configuration_data = Cursor::new(configuration_data);

    // ALU konstruieren
    let alu_add: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let alu_or: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let alu_and: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let alu_xor: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let load: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let save: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let zero_initializer: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let pc_init_value: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    println!("Config eingelesen");

    // Daten einlesen
    let mut deserialized_program = Vec::new();
    let mut file = File::open("program_data.bin")?;
    file.read_to_end(&mut deserialized_program)?;

    let program_data: Vec<(FheUint8, FheUint8)> = bincode::deserialize(&deserialized_program)?;
    println!("Programm eingelesen.");

    let mut control_unit = ControlUnit::new(
        alu_add,
        alu_or,
        alu_and,
        alu_xor,
        load,
        save,
        zero_initializer,
        pc_init_value,
        program_data
    );
    println!("CU erstellt.");

    control_unit.start(10);

    // TODO: Den gesamten RAM zurückgeben und auslesen I guess?
    // Ergebnis serialisiert abspeichern
    // let serialized_result = bincode::serialize(
    //     FheUint8::try_encrypt_trivial(2 as u8).unwrap()
    // )?;
    // let mut file = File::create("calculated_result.bin")?;
    // file.write_all(serialized_result.as_slice())?;
    println!("Ergebnis serialisiert.");
    Ok(())
}
