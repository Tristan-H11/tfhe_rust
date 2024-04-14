use std::time::Instant;

use rayon::prelude::*;
use tfhe::prelude::*;
use tfhe::{FheBool, FheUint8};
use crate::encrypt_trivial;

/// Darstellung des RAMs über einen Vector
/// Der Vector enthält in jeder Zelle ein Tupel (u8, u8).
/// Das erste Tupel enthält den Befehl, das zweite den Opernanden.
pub struct MemoryUint8 {
    data: Vec<(FheUint8, FheUint8)>,
}
impl MemoryUint8 {
    /// Erstellt den RAM und Accu mit den übergebenen Daten. Der Vektor darf maximal 8 bit Adressbreite haben und muss
    /// jede unbeschriebene Zelle mit 8 gefüllt haben. (Also exakt 256 Elemente lang sein)
    pub fn new(data: Vec<(FheUint8, FheUint8)>, size: usize) -> MemoryUint8 {
        println!("[RAM] new() gestartet.");
        assert_eq!(data.len(), size);
        MemoryUint8 { data }
    }

    pub fn get_data(&self) -> Vec<(FheUint8, FheUint8)> {
        println!("[RAM] get_data() aufgerufen");
        self.data.clone()
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    pub fn read_from_ram(&self, address: &FheUint8) -> (FheUint8, FheUint8) {
        let start_time = Instant::now();

        let mut result: (FheUint8, FheUint8) = (
            encrypt_trivial!(0u8),
            encrypt_trivial!(0u8),
        );

        result = self
            .data
            .par_iter()
            .enumerate()
            .map(|(current_index, (first, second))| {

                let encrypted_index: FheUint8 =
                    FheUint8::try_encrypt_trivial(current_index as u8).unwrap();
                let condition = &address.eq(&encrypted_index);

                let first_value = condition.if_then_else(first, &encrypt_trivial!(0u8));
                let second_value = condition.if_then_else(second, &encrypt_trivial!(0u8));

                let result: (FheUint8, FheUint8) = (
                    result.0.clone() + first_value,
                    result.1.clone() + second_value,
                );
                result
            })
            .reduce_with(|a: (FheUint8, FheUint8), b: (FheUint8, FheUint8)| (a.0 + b.0, a.1 + b.1))
            .unwrap_or(result);

        println!(
            "[RAM, {}ms] Lesen des RAMs beendet.",
            start_time.elapsed().as_millis()
        );
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: &FheUint8, new_value: &FheUint8, is_write: &FheBool) {
        let start_time = Instant::now();

        self.data
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, field)| {
                let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(index as u8).unwrap();
                let condition: FheBool = address.eq(&encrypted_index) & is_write;

                field.1 = condition.if_then_else(&new_value, &field.1);
            });

        println!(
            "[RAM, {}ms] Schreiben des RAMs beendet.",
            start_time.elapsed().as_millis()
        );
    }
}
