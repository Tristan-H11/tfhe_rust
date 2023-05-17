use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};

use crate::serverside::control_unit::ControlUnit;

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

    let serialized_configuration_data: Vec<FheUint8> = bincode::deserialize(&configuration_data)?;

    let alu_add: FheUint8 = serialized_configuration_data[0].clone();
    let alu_or: FheUint8 = serialized_configuration_data[1].clone();
    let alu_and: FheUint8 = serialized_configuration_data[2].clone();
    let alu_xor: FheUint8 = serialized_configuration_data[3].clone();
    let alu_sub: FheUint8 = serialized_configuration_data[4].clone();
    let alu_mul: FheUint8 = serialized_configuration_data[5].clone();
    let alu_add_r: FheUint8 = serialized_configuration_data[6].clone();
    let alu_or_r: FheUint8 = serialized_configuration_data[7].clone();
    let alu_and_r: FheUint8 = serialized_configuration_data[8].clone();
    let alu_xor_r: FheUint8 = serialized_configuration_data[9].clone();
    let alu_sub_r: FheUint8 = serialized_configuration_data[10].clone();
    let alu_mul_r: FheUint8 = serialized_configuration_data[11].clone();
    let load: FheUint8 = serialized_configuration_data[12].clone();
    let load_r: FheUint8 = serialized_configuration_data[13].clone();
    let save: FheUint8 = serialized_configuration_data[14].clone();
    let zero_initializer: FheUint8 = serialized_configuration_data[15].clone();
    let pc_init_value: FheUint8 = serialized_configuration_data[16].clone();
    println!("[Server] Config eingelesen");

    // Daten einlesen
    let mut deserialized_program = Vec::new();
    let mut file = File::open("program_data.bin")?;
    file.read_to_end(&mut deserialized_program)?;

    let mut program_data: Vec<(FheUint8, FheUint8)> = bincode::deserialize(&deserialized_program)?;

    // Die RAM_SIZE wird nun abhängig von dem übergebenn Programm bestimmt.
    // Damit ist sichergestellt, dass die CPU nur so viele Zyklen durchläuft, wie das Programm lang ist.
    // Ohne Sprünge ist das noch möglich.
    let ram_size: usize = program_data.len();

    println!("[Server] Programm eingelesen.");

    // Ram mit nullen auffüllen, bevor er übergeben wird.
    while program_data.len() < ram_size {
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
        alu_sub,
        alu_mul,
        alu_add_r,
        alu_or_r,
        alu_and_r,
        alu_xor_r,
        alu_sub_r,
        alu_mul_r,
        load,
        load_r,
        save,
        zero_initializer,
        pc_init_value,
        program_data,
        ram_size.clone()
    );
    println!("[Server] CU erstellt.");

    control_unit.start(ram_size as u8);

    let serialized_result = bincode::serialize(&control_unit.get_ram())?;
    let mut file = File::create("calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;
    println!("[Server] Ergebnis serialisiert.");
    Ok(())
}
