use tfhe::FheUint8;

use crate::util::encrypted_map::EncryptedMap;

/// Darstellung des RAMs über ein 256 Felder großes Array
pub struct Memory {
    memory: EncryptedMap,
}

impl Memory {
    /// Initialisiert alle 256 Adressen des RAMs mit `0` und gibt den initialiserten RAM zurück.
    pub fn new() -> Self {
        let mut memory: EncryptedMap = EncryptedMap::new();
        Memory {
            memory
        }
    }

    /// Liest einen Wert aus dem RAM, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    /// Sollte eine Adresse übergeben werden, die nicht existiert, so wird eine 0 zurückgegeben.
    pub fn read_from_ram(&self, address: FheUint8) -> FheUint8 {
        self.memory.find_value(&address).unwrap()
    }

    /// Schreibt einen Wert in den RAM und liest sowie schreibt dabei jede Zeile des RAMs einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn write_to_ram(&mut self, address: FheUint8, value: FheUint8) {
        self.memory.insert(address, value);
    }
}
