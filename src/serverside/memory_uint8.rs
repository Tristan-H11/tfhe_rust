use tfhe::FheUint8;
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
    pub fn new(zero_initializer: FheUint8, data: Vec<(FheUint8, FheUint8)>, size: usize) -> MemoryUint8 {
        println!("[RAM] new() gestartet.");
        assert_eq!(data.len(), size);
        MemoryUint8 {
            data,
            accu: zero_initializer.clone(),
        }
    }

    pub fn get_data(&self) -> Vec<(FheUint8, FheUint8)> {
        println!("[RAM] get_data() aufgerufen");
        self.data.clone()
    }

    /// Liefert den Wert des Akkumulators zurück.
    pub fn get_accu(&self) -> &FheUint8 {
        println!("[RAM] get_accu() aufgerufen");
        &self.accu
    }

    // Schreibt einen neuen Wert in den Akkumulator
    pub fn write_accu(&mut self, new_value: FheUint8, is_write_accu: &FheUint8) {
        println!("[RAM] write_accu() aufgerufen");
        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        self.accu = new_value * is_write_accu + &self.accu.clone() * (one - is_write_accu);
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: &FheUint8) -> (FheUint8, FheUint8) {
        println!("[RAM] read_from_ram() aufgerufen");
        let mut result: (FheUint8, FheUint8) =
            (
                FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
                FheUint8::try_encrypt_trivial(0 as u8).unwrap()
            );

        for (i, value) in self.data.iter().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            // OpCode auslesen
            let condition: FheUint8 = address.eq(&encrypted_index);
            result.0 = result.0 + (&value.0 * &condition);
            // Operanden auslesen
            result.1 = result.1 + (&value.1 * &condition);
        }
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: &FheUint8, new_value: FheUint8, is_write: &FheUint8) {
        println!("[RAM] write_to_ram() aufgerufen");
        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        for (i, field) in self.data.iter_mut().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            let condition: FheUint8 = address.eq(&encrypted_index) * is_write;
            let not_condition: FheUint8 = &one - &condition;

            // m_x = (indexEqual AND isWrite AND new_value) OR (!indexEqual OR !isWrite AND m_x)
            field.1 = (condition * new_value.clone()) + (not_condition * field.1.clone());
        }
    }
}
