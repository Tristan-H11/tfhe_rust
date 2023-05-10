use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write};

use bincode;
use tfhe::{FheUint16, ServerKey, set_server_key};
use tfhe::prelude::FheTryTrivialEncrypt;

use crate::serverside::alu::Alu;
use crate::serverside::memory_uint16::MemoryUint16;

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
    let ALU_ADD_REGRAM: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_ADD_REGREG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_AND_REGRAM: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_AND_REGREG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_OR_REGRAM: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_OR_REGREG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_XOR_REGRAM: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ALU_XOR_REGREG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let MOV_RAMREG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let MOV_REGRAM: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let LOAD_CONST_REG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let SWAP_REGREG: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let OUT_RAM: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let JMP: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let JMPC: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let JMPO: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let JMPZ: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let JMPR: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let END: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let REG1_ADR: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let REG2_ADR: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let REG3_ADR: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let REG4_ADR: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let ZERO_INITIALIZER: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;

    println!("Daten eingelesen.");

    // TODO: Neue Config-data auslesen. Die paar auslesungen über dem Todo hier passen nicht mehr
    //  Dann muss der Programmcode ausgelesen werden. Das ist ein 16 Bit Vector, der gespeichert wird
    //  dann wird durch den Vector durchgegangen, ne Maske von 0b11_1111 aufgelegt und der Befehl
    //  gematched. Der gematchte Befehl führt dann Sachen aus und dann wird der PC erhöht und
    //  der nächste Befehl aus dem Vector ausgelesen.


    let mut alu = Alu {
        opcode_add,
        opcode_and,
        opcode_or,
        opcode_xor,
        zero_flag: ZERO_INITIALIZER.clone(),
        overflow_flag: ZERO_INITIALIZER.clone(),
        carry_flag: ZERO_INITIALIZER.clone(),
    };


    let op_code: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let a: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    let b: FheUint16 = bincode::deserialize_from(&mut serialized_configuration_data)?;
    println!("Alu erstellt.");

    // Der Datenspeicher ist vorerst nur 8 Zeilen groß
    let mut memory = MemoryUint16::new(8);
    memory.write_to_ram(
        FheUint16::try_encrypt_trivial(0 as u16).unwrap(),
        a.clone(),
    );
    memory.write_to_ram(
        FheUint16::try_encrypt_trivial(1 as u16).unwrap(),
        b.clone(),
    );
    println!("Operanden in den RAM geschrieben.");

    /*
    let deserialized_values: Vec<myType> = bincode::deserialize(&file_content)?;
    liest die gesamte Datei (oder den Rest ein) und teilt es in den Array auf
     */

    // Ergebnis berechnen
    let result = alu.calculate(
        op_code,
        memory.read_from_ram(FheUint16::try_encrypt_trivial(0 as u16).unwrap()),
        memory.read_from_ram(FheUint16::try_encrypt_trivial(1 as u16).unwrap()),
    )?;


    memory.write_to_ram(
        FheUint16::try_encrypt_trivial(2 as u16).unwrap(),
        result.clone(),
    );
    println!("Alu Ergebnis in den RAM geschrieben.");

    // TODO: Hier muss irgendwie ein Vector existieren, dessen Werte per OUT-Befehl gespeichert wurden
    //  Dieser Vektor wird dann hier ausgelesen und zurück serialisiert, nach dem der END-Befehl kam
    // Ergebnis serialisiert abspeichern
    let serialized_result = bincode::serialize(
        &memory.read_from_ram(FheUint16::try_encrypt_trivial(2 as u16).unwrap())
    )?;
    let mut file = File::create("calculated_result.bin")?;
    file.write_all(serialized_result.as_slice())?;
    println!("Ergebnis serialisiert.");
    Ok(())
}
