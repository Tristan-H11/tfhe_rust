use std::error::Error;
use std::fs::File;
use std::io::Write;

use bincode;
use tfhe::{ConfigBuilder, FheUint16, generate_keys};
use tfhe::prelude::*;

use crate::clientside::statics::*;

/// Client-Main-Funktion.
/// Hier werden:
/// - die Schlüssel erstellt,
/// - die Schlüssel serialisiert,
/// - die Maschinensprache konfiguriert,
/// - die OpCodes der Maschinensprache verschlüsselt und serialisiert,
/// - die Operanden und der OpCode für die Alu verschlüsselt und serialisiert. (Das wird sich demnächst ändern, wenn die CU gebaut wird)
pub fn start() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint16()
        .build();
    let (client_key, server_key) = generate_keys(config);

    // ServerKey speichern
    let mut serialized_server_key = Vec::new();
    bincode::serialize_into(&mut serialized_server_key, &server_key)?;

    let mut file = File::create("server_key.bin")?;
    file.write_all(serialized_server_key.as_slice())?;


    // ClientKey speichern
    let mut serialized_private_key = Vec::new();
    bincode::serialize_into(&mut serialized_private_key, &client_key)?;

    let mut file = File::create("private_key.bin")?;
    file.write_all(serialized_private_key.as_slice())?;


    // Daten speichern
    let configuration_data: Vec<u16> = vec![
        ALU_ADD_REGRAM,
        ALU_ADD_REGREG,
        ALU_AND_REGRAM,
        ALU_AND_REGREG,
        ALU_OR_REGRAM,
        ALU_OR_REGREG,
        ALU_XOR_REGRAM,
        ALU_XOR_REGREG,
        MOV_RAMREG,
        MOV_REGRAM,
        LOAD_CONST_REG,
        SWAP_REGREG,
        OUT_RAM,
        JMP,
        JMPC,
        JMPO,
        JMPZ,
        JMPR,
        END,
        REG1_ADR,
        REG2_ADR,
        REG3_ADR,
        REG4_ADR,
        ZERO_INITIALIZER
    ];

    // Die 16 Bit Befehle,die ausgeführt werden sollen
    let program_data: Vec<u16> = vec![
        LOAD_2_TO_REG1,
        LOAD_1_TO_REG2,
        ADD_REG1_REG2,
        OUT_REG1
    ];

    // Alle Werte im Vector verschlüsseln und serialiseren
    let encrypted_configuration_data: Vec<FheUint16> = configuration_data.iter()
        .map(|&x: &u16| FheUint16::encrypt(x, &client_key))
        .collect();

    let mut serialized_configuration_data = Vec::new();
    for encrypted_value in encrypted_configuration_data {
        bincode::serialize_into(&mut serialized_configuration_data, &encrypted_value)?;
    }

    let mut file = File::create("config_data.bin")?;
    file.write_all(serialized_configuration_data.as_slice())?;


    let encrypted_program_data: Vec<FheUint16> = program_data.iter()
        .map(|&x: &u16| FheUint16::encrypt(x, &client_key))
        .collect();

    let mut serialized_program_data = Vec::new();
    for encrypted_value in encrypted_program_data {
        bincode::serialize_into(&mut serialized_program_data, &encrypted_value)?;
    }

    let mut file = File::create("program_data.bin")?;
    file.write_all(serialized_program_data.as_slice())?;

    Ok(())
}
