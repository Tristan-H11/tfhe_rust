use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write};

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};
use tfhe::prelude::FheTryTrivialEncrypt;

use crate::serverside::alu::Alu;
use crate::serverside::memory::Memory;

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
    let mut file = File::open("C:/data/server_key.bin")?;
    file.read_to_end(&mut serialized_server_key)?;
    let server_key: ServerKey = bincode::deserialize(&serialized_server_key)?;

    set_server_key(server_key);

    // Daten einlesen
    let mut data = Vec::new();
    let mut file = File::open("C:/data/data.bin")?;
    file.read_to_end(&mut data)?;


    let mut serialized_data = Cursor::new(data);

    // ALU konstruieren
    let opcode_add: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_and: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_or: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let opcode_xor: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    let zero_flag_initializer: FheUint8 = FheUint8::try_encrypt_trivial(0u8).unwrap();

    let mut alu = Alu {
        opcode_add,
        opcode_and,
        opcode_or,
        opcode_xor,
        zero_flag: zero_flag_initializer.clone(),
        overflow_flag: zero_flag_initializer.clone(),
        carry_flag: zero_flag_initializer.clone(),
    };

    // Memory-Access konstruieren
    let ram_read: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let ram_write: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;


    let op_code: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let a: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    let b: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    // TODO: Memory bauen und einbinden
    let mut memory = Memory::new();
    memory.write_to_ram(
        FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
        a.clone(),
    );
    memory.write_to_ram(
        FheUint8::try_encrypt_trivial(1 as u8).unwrap(),
        b.clone(),
    );

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


    // Ergebnis serialisiert abspeichern
    let serialized_result = bincode::serialize(
        &memory.read_from_ram(FheUint8::try_encrypt_trivial(2 as u8).unwrap())
    )?;
    let mut file = File::create("C:/data/calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;

    Ok(())
}
