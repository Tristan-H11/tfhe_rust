use tfhe::{FheUint16, FheUint8};
use tfhe::prelude::*;

/// Darstellung des RAMs über einen Vector
/// Der Vector enthält in jeder Zelle ein Tupel (u8, u8).
/// Das erste Tupel enthält den Befehl, das zweite den Opernanden.
pub struct MemoryUint8 {
    data: Vec<(FheUint8, FheUint8)>,
    accu: FheUint8,
}

impl MemoryUint8 {
    /// Erstellt den RAM und Accu mit den übergebenen Daten. Der Vektor darf maximal 8 bit Adressbreite haben und muss
    /// jede unbeschriebene Zelle mit 8 gefüllt haben. (Also exakt 256 Elemente lang sein)
    pub fn new(zero_initializer: FheUint8, data: Vec<(FheUint8, FheUint8)>) -> MemoryUint8 {
        println!("RAM erstellen gestartet.");
        assert_eq!(data.len(), 256);
        MemoryUint8 {
            data,
            accu: zero_initializer.clone(),
        }
    }

    /// Liefert den Wert des Akkumulators zurück.
    pub fn get_accu(&self) -> &FheUint8 {
        &self.accu
    }

    // Schreibt einen neuen Wert in den Akkumulator
    pub fn write_accu(&mut self, new_value: FheUint8) {
        self.accu = new_value;
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: &FheUint8) -> (FheUint8, FheUint8) {
        println!("Lesen des RAMs gestartet");
        let mut result: (FheUint8, FheUint8) =
            (
                FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
                FheUint8::try_encrypt_trivial(0 as u8).unwrap()
            );

        for (i, &mut value) in self.data.iter().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            // OpCode auslesen
            result.0 = result.0 + (&value.0 * address.eq(&encrypted_index));
            // Operanden auslesen
            result.1 = result.1 + (&value.1 * address.eq(&encrypted_index));
        }
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: FheUint8, value: FheUint8, is_write: FheUint8) {
        println!("Schreiben des RAMs gestartet");
        let lsb_mask: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        for (i, &mut field) in self.data.iter_mut().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            let condition: FheUint8 = address.eq(&encrypted_index);
            let not_condition: FheUint8 = &condition ^ &lsb_mask;

            // m_x = (indexEqual AND newValue AND isWrite) OR (!indexEqual AND m_x)
            *field.1 = (condition * value.clone() * &is_write) + (not_condition * field.clone());
        }
    }
}
