use tfhe::FheUint8;
use tfhe::prelude::*;

/// Darstellung des RAMs über ein Array.
/// Diese 8 Bit Variante soll als Datenspeicher dienen und wird von den MOV Befehlen angesprochen
pub struct MemoryUint8 {
    data: Vec<FheUint8>,
}

impl MemoryUint8 {
    pub fn new(capacity: usize) -> MemoryUint8 {
        println!("RAM erstellen gestartet.");
        let mut data: Vec<FheUint8> = Vec::with_capacity(capacity);
        for i in 0..capacity {
            data.push(
                FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
            );
        }
        MemoryUint8 {
            data
        }
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: FheUint8) -> FheUint8 {
        println!("Lesen des RAMs gestartet");
        let mut result: FheUint8 = FheUint8::try_encrypt_trivial(0u8).unwrap();
        for (i, value) in self.data.iter().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            result = result + (value * address.eq(&encrypted_index));
        }
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: FheUint8, value: FheUint8) {
        println!("Schreiben des RAMs gestartet");
        let lsb_mask: FheUint8 = FheUint8::try_encrypt_trivial(1u8).unwrap();

        for (i, field) in self.data.iter_mut().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            let condition: FheUint8 = address.eq(&encrypted_index);
            let not_condition: FheUint8 = &condition ^ &lsb_mask;

            // m_x = (indexEqual AND newValue) OR (!indexEqual AND m_x)
            *field = (condition * value.clone()) + (not_condition * field.clone());
        }
    }
}
