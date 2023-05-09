use tfhe::FheUint8;
use tfhe::prelude::*;

const ARRAY_SIZE: usize = 256;

pub struct EncryptedMap {
    data: Vec<Option<(FheUint8, FheUint8)>>,}

impl EncryptedMap {
    /// Initialisiert alle 256 Adressen der Map mit `0` und gibt die initialiserte Map zurück.
    pub fn new() -> Self {
        let mut map: Vec<Option<(FheUint8, FheUint8)>> = Vec::with_capacity(ARRAY_SIZE);
        for i in 0..ARRAY_SIZE {
            map.push(Some((
                FheUint8::try_encrypt_trivial(i as u8).unwrap(),
                FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
            )));
        }
        EncryptedMap {
            data: map,
        }
    }

    /// Schreibt einen Wert in die Map und liest sowie schreibt dabei jede Zeile der Map einmal, damit
    /// kein Rückschluss auf die veränderte Zeile gezogen werden kann.
    pub fn insert(&mut self, key: FheUint8, value: FheUint8) {
        let lsb_mask: FheUint8 = FheUint8::try_encrypt_trivial(1u8).unwrap();

        for i in 0..ARRAY_SIZE {
            match &self.data[i] {
                None => {
                    self.data[i] = Some((key, value));
                    break;
                }
                Some((existing_key, old_value)) => {
                    let equal = existing_key.eq(&key);
                    let not_equal: FheUint8 = &equal ^ &lsb_mask;

                    self.data[i] = Some((
                        key,
                        value * equal + old_value * not_equal
                    ));
                    break;
                }
                _ => continue,
            }
        }
    }

    /// Liest einen Wert aus der Map, in dem jede Zeile einmal gelesen wird.
    /// Der "unsichtbare" Zugriff ist durch die arithmetische Logik anstelle von
    /// Verzweigungen gelöst.
    /// Sollte ein Key übergeben werden, der nicht existiert, so wird eine 0 zurückgegeben.
    pub fn find_value(&self, key: &FheUint8) -> Option<FheUint8> {
        let mut result: FheUint8 = FheUint8::try_encrypt_trivial(0 as u8).unwrap();
        for i in 0..ARRAY_SIZE {
            match &self.data[i] {
                Some((existing_key, value)) => {
                    result = result + (value * key.eq(existing_key));
                }
                _ => continue,
            }
        }
        Some(result)
    }
}
