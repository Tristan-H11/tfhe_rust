use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write};

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};

use crate::serverside::control_unit::ControlUnit;

pub static RAM_SIZE: usize = 4;

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
    println!("[Server] ServerKey eingelesen und gesetzt.");

    // Daten einlesen
    let mut configuration_data = Vec::new();
    let mut file = File::open("config_data.bin")?;
    file.read_to_end(&mut configuration_data)?;

    let mut serialized_configuration_data: Vec<FheUint8> = bincode::deserialize(&configuration_data)?;

    // ALU konstruieren
    let alu_add: FheUint8 = serialized_configuration_data[0].clone();
    let alu_or: FheUint8 = serialized_configuration_data[1].clone();
    let alu_and: FheUint8 = serialized_configuration_data[2].clone();
    let alu_xor: FheUint8 = serialized_configuration_data[3].clone();
    let load: FheUint8 = serialized_configuration_data[4].clone();
    let save: FheUint8 = serialized_configuration_data[5].clone();
    let zero_initializer: FheUint8 = serialized_configuration_data[6].clone();
    let pc_init_value: FheUint8 = serialized_configuration_data[7].clone();
    println!("[Server] Config eingelesen");

    // Daten einlesen
    let mut deserialized_program = Vec::new();
    let mut file = File::open("program_data.bin")?;
    file.read_to_end(&mut deserialized_program)?;

    let mut program_data: Vec<(FheUint8, FheUint8)> = bincode::deserialize(&deserialized_program)?;
    println!("[Server] Programm eingelesen.");

    // Ram mit nullen auffüllen, bevor er übergeben wird.
    while program_data.len() < RAM_SIZE {
        program_data.push(
            (
                zero_initializer.clone(),
                zero_initializer.clone()
            )
        )
    }

    let mut control_unit = ControlUnit::new(
        alu_add,
        alu_or,
        alu_and,
        alu_xor,
        load,
        save,
        zero_initializer,
        pc_init_value,
        program_data,
        RAM_SIZE
    );
    println!("[Server] CU erstellt.");

    control_unit.start(4);

    let serialized_result = bincode::serialize(&control_unit.get_ram())?;
    let mut file = File::create("calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;
    println!("[Server] Ergebnis serialisiert.");
    Ok(())
}
