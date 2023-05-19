use std::error::Error;
use std::fs::File;
use std::io::Write;

use bincode;
use tfhe::{ConfigBuilder, FheUint8, generate_keys};
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
        .enable_default_uint8()
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
    let configuration_data: Vec<u8> = vec![
        ALU_ADD,
        ALU_OR,
        ALU_AND,
        ALU_XOR,
        ALU_SUB,
        ALU_MUL,
        ALU_ADD_R,
        ALU_OR_R,
        ALU_AND_R,
        ALU_XOR_R,
        ALU_SUB_R,
        ALU_MUL_R,
        LOAD,
        LOAD_R,
        SAVE,
        JNZ,
        ZERO_INITIALIZER,
        PC_INIT_VALUE,
    ];

    // Die Befehle, die ausgeführt werden sollen
    let program_data: Vec<(u8, u8)> = vec![
        (LOAD, 2),      // Speicher für den Counter allocaten <-1
        (LOAD, 3),      // Initialwert des Ergebnisses <-6
        // Multiplikation
        (LOAD_R, 1),
        (ALU_MUL_R, 0), // Multiplizieren
        (SAVE, 1),      // Ergebnis zwischenspeichern
        // Counter-Dekrement
        (LOAD_R, 0),    // Counter laden
        (ALU_SUB, 1),   // Counter dekrementieren
        (SAVE, 0),      // Counter zwischenspeichern
        // Jump
        (JNZ, 2),       // Von vorn, wenn Accu != 0
    ];

    // Alle Werte im Vector verschlüsseln und serialiseren
    let encrypted_configuration_data: Vec<FheUint8> = configuration_data.iter()
        .map(|&x: &u8| FheUint8::encrypt(x, &client_key))
        .collect();

    let mut serialized_configuration_data = Vec::new();
    bincode::serialize_into(&mut serialized_configuration_data, &encrypted_configuration_data)?;

    let mut file = File::create("config_data.bin")?;
    file.write_all(serialized_configuration_data.as_slice())?;


    let encrypted_program_data: Vec<(FheUint8, FheUint8)> = program_data.iter()
        .map(|&(x, y): &(u8, u8)|
            (
                FheUint8::encrypt(x, &client_key),
                FheUint8::encrypt(y, &client_key)
            )
        )
        .collect();

    let mut serialized_program_data = Vec::new();
    bincode::serialize_into(&mut serialized_program_data, &encrypted_program_data)?;

    let mut file = File::create("program_data.bin")?;
    file.write_all(serialized_program_data.as_slice())?;

    Ok(())
}
