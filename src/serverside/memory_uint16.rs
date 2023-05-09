use tfhe::FheUint16;
use tfhe::prelude::*;

/// Darstellung des RAMs über ein Array
/// Diese 16 Bit Variante soll als speicher für die Instruktionen genutzt werden
pub struct MemoryUint16 {
    data: Vec<FheUint16>,
}

impl MemoryUint16 {
    pub fn new(capacity: usize) -> MemoryUint16 {
        println!("RAM erstellen gestartet.");
        let mut data: Vec<FheUint16> = Vec::with_capacity(capacity);
        for i in 0..capacity {
            data.push(
                FheUint16::try_encrypt_trivial(0 as u16).unwrap(),
            );
        }
        MemoryUint16 {
            data
        }
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: FheUint16) -> FheUint16 {
        println!("Lesen des RAMs gestartet");
        let mut result: FheUint16 = FheUint16::try_encrypt_trivial(0 as u16).unwrap();
        for (i, value) in self.data.iter().enumerate() {
            let encrypted_index: FheUint16 = FheUint16::try_encrypt_trivial(i as u16).unwrap();

            result = result + (value * address.eq(&encrypted_index));
        }
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: FheUint16, value: FheUint16) {
        println!("Schreiben des RAMs gestartet");
        let lsb_mask: FheUint16 = FheUint16::try_encrypt_trivial(1 as u16).unwrap();

        for (i, field) in self.data.iter_mut().enumerate() {
            let encrypted_index: FheUint16 = FheUint16::try_encrypt_trivial(i as u16).unwrap();

            let condition: FheUint16 = address.eq(&encrypted_index);
            let not_condition: FheUint16 = &condition ^ &lsb_mask;

            // m_x = (indexEqual AND newValue) OR (!indexEqual AND m_x)
            *field = (condition * value.clone()) + (not_condition * field.clone());
        }
    }
}
