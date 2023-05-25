use std::time::Instant;

use rayon::prelude::*;
use tfhe::{FheUint8, set_server_key};
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
    pub fn write_accu(&mut self, new_value: &FheUint8, is_write_accu: &FheUint8) {
        let start_time = Instant::now();
        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        self.accu = new_value * is_write_accu + self.get_accu() * (one - is_write_accu);
        println!("[RAM, {}ms] Schreiben des Akkumulators beendet.", start_time.elapsed().as_millis());
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: &FheUint8) -> (FheUint8, FheUint8) {
        let start_time = Instant::now();

        let mut result: (FheUint8, FheUint8) =
            (
                FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
                FheUint8::try_encrypt_trivial(0 as u8).unwrap()
            );

        result = self.data.par_iter()
            .enumerate()
            .map(|(current_index, (first, second))| {
                let address = address.clone();

                let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(current_index as u8).unwrap();
                let condition: &FheUint8 = &address.eq(&encrypted_index);

                let result: (FheUint8, FheUint8) = (
                    result.0.clone() + first * condition,
                    result.1.clone() + second * condition,
                );
                result
            })
            .reduce_with(|a: (FheUint8, FheUint8), b: (FheUint8, FheUint8)| {
                (a.0 + b.0, a.1 + b.1)
            }).unwrap_or(result);

        println!("[RAM, {}ms] Lesen des RAMs beendet.", start_time.elapsed().as_millis());
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: &FheUint8, new_value: &FheUint8, is_write: &FheUint8) {
        let start_time = Instant::now();
        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        self.data.par_iter_mut().enumerate().for_each(|(index, field)| {
            let new_value = new_value.clone();
            let is_write = is_write.clone();
            let one = one.clone();
            let address = address.clone();

            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(index as u8).unwrap();
            let condition: FheUint8 = address.eq(&encrypted_index) * &is_write;
            let not_condition: FheUint8 = &one - &condition;

            field.1 = (condition * new_value.clone()) + (not_condition * field.1.clone());
        });

        println!("[RAM, {}ms] Schreiben des RAMs beendet.", start_time.elapsed().as_millis());
    }
}
