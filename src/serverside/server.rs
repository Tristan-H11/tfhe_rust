use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write};

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};
use tfhe::prelude::FheTryTrivialEncrypt;

use crate::serverside::alu::Alu;
use crate::serverside::memory_uint8::MemoryUint8;

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
    let ALU_ADD: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_OR: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_AND: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_XOR: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let LOAD: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let SAVE: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ZERO_INITIALIZER: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let PC_INIT_VALUE: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;


    println!("Daten eingelesen.");


    let mut alu = Alu {
        opcode_add: ALU_ADD,
        opcode_and: ALU_AND,
        opcode_or: ALU_OR,
        opcode_xor: ALU_XOR,
        zero_flag: ZERO_INITIALIZER.clone(),
        overflow_flag: ZERO_INITIALIZER.clone(),
        carry_flag: ZERO_INITIALIZER.clone(),
    };


    let op_code: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let a: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let b: FheUint8 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    println!("Alu erstellt.");

    // Der Datenspeicher ist vorerst nur 8 Zeilen groß
    let mut memory = MemoryUint8::new(8, ZERO_INITIALIZER.clone());

    println!("Operanden in den RAM geschrieben.");

    /*
    let deserialized_values: Vec<myType> = bincode::deserialize(&file_content)?;
    liest die gesamte Datei (oder den Rest ein) und teilt es in den Array auf
     */

    // Ergebnis berechnen
    let result = alu.calculate(
        op_code,
        memory.read_from_ram(FheUint8::try_encrypt_trivial(0 as u8).unwrap()),
        memory.read_from_ram(FheUint8::try_encrypt_trivial(1 as u8).unwrap()),
    )?;


    memory.write_to_ram(
        FheUint8::try_encrypt_trivial(2 as u8).unwrap(),
        result.clone(),
    );
    println!("Alu Ergebnis in den RAM geschrieben.");

    // TODO: Hier muss irgendwie ein Vector existieren, dessen Werte per OUT-Befehl gespeichert wurden
    //  Dieser Vektor wird dann hier ausgelesen und zurück serialisiert, nach dem der END-Befehl kam
    // Ergebnis serialisiert abspeichern
    let serialized_result = bincode::serialize(
        &memory.read_from_ram(FheUint8::try_encrypt_trivial(2 as u8).unwrap())
    )?;
    let mut file = File::create("calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;
    println!("Ergebnis serialisiert.");
    Ok(())
}
