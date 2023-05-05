use std::error::Error;

use tfhe::FheUint8;
use tfhe::prelude::*;

/// Darstellung des RAMs über ein 256 Felder großes Array
struct Memory {
    pub(crate) memory: [FheUint8; 256],
}

impl Memory {
    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: FheUint8) -> Result<FheUint8, Box<dyn Error>> {
        let mut result: FheUint8 = FheUint8::try_encrypt_trivial(0u8).unwrap();
        for (i, value) in self.memory.iter().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            result = result + (value * address.eq(&encrypted_index));
        }
        Ok(result)
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: FheUint8, value: FheUint8) {
        let max_value: FheUint8 = FheUint8::try_encrypt_trivial(255u8).unwrap();

        for (i, field) in self.memory.iter_mut().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            let condition: FheUint8 = address.eq(&encrypted_index);
            // X ^ 1 = !X, deshalb wird hier Condition mit 1111_1111 xOr't
            let not_condition: FheUint8 = &condition ^ &max_value;

            // m_x = (indexEqual AND newValue) OR (!indexEqual AND m_x)
            *field = (condition * value.clone()) + (not_condition * field.clone());
        }
    }
}
