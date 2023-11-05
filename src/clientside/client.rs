use std::error::Error;
use std::fs::File;
use std::io::Write;

use bincode;
use tfhe::prelude::*;
use tfhe::{generate_keys, ConfigBuilder, FheUint8};

use crate::clientside::statics::*;

///
/// Startet den Client.
/// Erzeugt die Schlüssel und speichert sie in Dateien.
/// Erzeugt die verschlüsselten Daten und speichert sie in Dateien.
///
pub fn start() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
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

    // Die Befehle, die ausgeführt werden sollen
    let program_data: Vec<(u8, u8)> = vec![
        (LOAD, 2), // Lade 1 in den Akkumulator (Akk = 1)
        (ALU_ADD, 3),
        (STORE, 0),
    ];

    let encrypted_program_data: Vec<(FheUint8, FheUint8)> = program_data
        .iter()
        .map(|&(x, y): &(u8, u8)| {
            (
                FheUint8::encrypt(x, &client_key),
                FheUint8::encrypt(y, &client_key),
            )
        })
        .collect();

    let mut serialized_program_data = Vec::new();
    bincode::serialize_into(&mut serialized_program_data, &encrypted_program_data)?;

    let mut file = File::create("program_data.bin")?;
    file.write_all(serialized_program_data.as_slice())?;

    Ok(())
}
