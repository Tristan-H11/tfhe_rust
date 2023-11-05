///
/// Makro zur Erzeugung eines FheUing8::try_encrypt_trivial Aufrufes.
///
/// # Arguments
/// * `value` - Der Wert, der verschlüsselt werden soll.
#[macro_export]
macro_rules! encrypt_trivial {
    ($value: expr) => {
        FheUint8::try_encrypt_trivial($value).unwrap()
    };
}
