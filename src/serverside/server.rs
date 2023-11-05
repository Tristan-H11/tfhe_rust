use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

use bincode;
use tfhe::prelude::FheTryTrivialEncrypt;
use tfhe::{set_server_key, FheUint8, ServerKey};

use crate::serverside::control_unit::ControlUnit;
use crate::serverside::opcode_container::OpcodeContainer;

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
    let start_time = Instant::now();
    // Server Key einlesen
    let mut serialized_server_key = Vec::new();
    let mut file = File::open("server_key.bin")?;
    file.read_to_end(&mut serialized_server_key)?;
    let server_key: ServerKey = bincode::deserialize(&serialized_server_key)?;

    let cloned_key = server_key.clone();
    set_server_key(server_key);

    // There is a race condition when using set_server_key inside of rayon-threads.
    // This race condition is introduced by the way rayon uses work stealing
    // So this workaround casts the key onto the global rayon-threadpool and thus, it can't race anymore.
    rayon::broadcast(|_| set_server_key(cloned_key.clone()));
    println!(
        "[Server, {}ms] ServerKey eingelesen und gesetzt.",
        start_time.elapsed().as_millis()
    );

    // Daten einlesen
    let opcodes: OpcodeContainer = OpcodeContainer::new();

    let zero_initializer: FheUint8 = FheUint8::try_encrypt_trivial(0u8).unwrap();
    let pc_init_value: FheUint8 = zero_initializer.clone();

    let start_time = Instant::now();
    // Daten einlesen
    let mut deserialized_program = Vec::new();
    let mut file = File::open("program_data.bin")?;
    file.read_to_end(&mut deserialized_program)?;

    let mut program_data: Vec<(FheUint8, FheUint8)> = bincode::deserialize(&deserialized_program)?;
    println!(
        "[Server, {}ms] Programm eingelesen.",
        start_time.elapsed().as_millis()
    );

    let start_time = Instant::now();
    // Die ram_size wird nun abhängig von dem übergebenen Programm bestimmt.
    let ram_size: usize = program_data.len();

    // Ram mit nullen auffüllen, bevor er übergeben wird.
    while program_data.len() < ram_size {
        program_data.push((zero_initializer.clone(), zero_initializer.clone()))
    }

    let mut control_unit = ControlUnit::new(
        opcodes,
        zero_initializer,
        pc_init_value,
        program_data,
        ram_size.clone(),
        cloned_key,
    );
    println!(
        "[Server, {}ms] CU erstellt.",
        start_time.elapsed().as_millis()
    );

    control_unit.start(ram_size as u8);

    let start_time = Instant::now();
    let serialized_result = bincode::serialize(&control_unit.get_ram())?;
    let mut file = File::create("calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;
    println!(
        "[Server, {}ms] Ergebnis serialisiert.",
        start_time.elapsed().as_millis()
    );
    Ok(())
}
