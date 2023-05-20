use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};

use crate::serverside::control_unit::ControlUnit;
use crate::serverside::opcode_container::OpcodeContainer;
use crate::serverside::opcode_container_alu::OpcodeContainerAlu;

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

    let opcodes_alu: OpcodeContainerAlu = OpcodeContainerAlu {
        add: serialized_configuration_data[0].clone(),
        or: serialized_configuration_data[1].clone(),
        and: serialized_configuration_data[2].clone(),
        xor: serialized_configuration_data[3].clone(),
        sub: serialized_configuration_data[4].clone(),
        mul: serialized_configuration_data[5].clone(),
        add_r: serialized_configuration_data[6].clone(),
        or_r: serialized_configuration_data[7].clone(),
        and_r: serialized_configuration_data[8].clone(),
        xor_r: serialized_configuration_data[9].clone(),
        sub_r: serialized_configuration_data[10].clone(),
        mul_r: serialized_configuration_data[11].clone(),
    };

    let opcodes: OpcodeContainer = OpcodeContainer {
        opcodes_alu,
        load: serialized_configuration_data[12].clone(),
        load_r: serialized_configuration_data[13].clone(),
        save: serialized_configuration_data[14].clone(),
        jnz: serialized_configuration_data[15].clone()
    };

    let zero_initializer: FheUint8 = serialized_configuration_data[16].clone();
    let pc_init_value: FheUint8 = serialized_configuration_data[17].clone();
    println!("[Server] Config eingelesen");

    // Daten einlesen
    let mut deserialized_program = Vec::new();
    let mut file = File::open("program_data.bin")?;
    file.read_to_end(&mut deserialized_program)?;

    let mut program_data: Vec<(FheUint8, FheUint8)> = bincode::deserialize(&deserialized_program)?;

    // Die ram_size wird nun abhängig von dem übergebenen Programm bestimmt.
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
        opcodes,
        zero_initializer,
        pc_init_value,
        program_data,
        ram_size.clone(),
    );
    println!("[Server] CU erstellt.");

    control_unit.start(25);

    let serialized_result = bincode::serialize(&control_unit.get_ram())?;
    let mut file = File::create("calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;
    println!("[Server] Ergebnis serialisiert.");
    Ok(())
}
