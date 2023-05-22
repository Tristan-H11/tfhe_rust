use std::cmp::{max};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use tfhe::{FheUint8, ServerKey, set_server_key};
use tfhe::prelude::*;

/// Darstellung des RAMs über einen Vector
/// Der Vector enthält in jeder Zelle ein Tupel (u8, u8).
/// Das erste Tupel enthält den Befehl, das zweite den Opernanden.
pub struct MemoryUint8 {
    data: Vec<(FheUint8, FheUint8)>,
    accu: FheUint8,
    key: ServerKey,
}

impl MemoryUint8 {
    /// Erstellt den RAM und Accu mit den übergebenen Daten. Der Vektor darf maximal 8 bit Adressbreite haben und muss
    /// jede unbeschriebene Zelle mit 8 gefüllt haben. (Also exakt 256 Elemente lang sein)
    pub fn new(zero_initializer: FheUint8, data: Vec<(FheUint8, FheUint8)>, size: usize, key: ServerKey) -> MemoryUint8 {
        println!("[RAM] new() gestartet.");
        assert_eq!(data.len(), size);
        MemoryUint8 {
            data,
            accu: zero_initializer.clone(),
            key,
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

        let mut threads: Vec<JoinHandle<(FheUint8, FheUint8)>> = vec![];
        let thread_count: usize = 4;
        let chunk_size = max(thread_count, self.data.len() / thread_count);

        let chunks = self.data.chunks(chunk_size);

        for (chunk_number, chunk) in chunks.enumerate() {
            let chunk: Vec<(FheUint8, FheUint8)> = chunk.to_vec(); // Kopieren des Chunks, damit es threadsicher ist
            let mut result = result.clone();
            let address = address.clone();
            let key = self.key.clone();
            let chunk_number = chunk_number.clone();
            let chunk_size = chunk_size.clone();
            threads.push(
                thread::spawn(move || {
                    set_server_key(key);
                    for (chunk_index, (first, second)) in chunk.iter().enumerate() {
                        let current_index: u8 = (chunk_number * chunk_size + chunk_index) as u8;
                        let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(current_index).unwrap();

                        // Adresse vergleichen
                        let condition: FheUint8 = address.eq(&encrypted_index);
                        // Opcode setzen
                        result.0 += first * &condition;
                        // Operand setzen
                        result.1 += second * &condition;
                    }
                    result
                })
            );
        }

        for thread in threads {
            let thread_result = thread.join().unwrap();
            result.0 += thread_result.0;
            result.1 += thread_result.1;
        }

        println!("[RAM, {}ms] Lesen des RAMs beendet.", start_time.elapsed().as_millis());
        result
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: &FheUint8, new_value: &FheUint8, is_write: &FheUint8) {
        let start_time = Instant::now();
        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();

        for (i, field) in self.data.iter_mut().enumerate() {
            let encrypted_index: FheUint8 = FheUint8::try_encrypt_trivial(i as u8).unwrap();

            let condition: FheUint8 = address.eq(&encrypted_index) * is_write;
            let not_condition: FheUint8 = &one - &condition;

            // m_x = (indexEqual AND isWrite AND new_value) OR (!indexEqual OR !isWrite AND m_x)
            field.1 = (condition * new_value.clone()) + (not_condition * field.1.clone());
        }
        println!("[RAM, {}ms] Schreiben des RAMs beendet.", start_time.elapsed().as_millis());
    }
}
