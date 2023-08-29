//------ Konfiguration der Maschinensprache
/**
MSB ist ein ALU JA NEIN
2MSB ist loadFromRAM JA NEIN
3MSB ist Programmfluss (branch, jump) JA NEIN
*/

// Setzen der OpCodes
pub static ALU_ADD: u8  = 0b1000_0001; // Addition
pub static ALU_OR: u8   = 0b1000_0010; // Bitwise Or
pub static ALU_AND: u8  = 0b1000_0011; // Bitwise And
pub static ALU_XOR: u8  = 0b1000_0100; // Bitwise Xor
pub static ALU_SUB: u8  = 0b1000_0101; // Subtraction
pub static ALU_MUL: u8  = 0b1000_0110; // Multiplication

pub static ALU_ADD_R: u8  = 0b1100_0001; // Addition
pub static ALU_OR_R: u8   = 0b1100_0010; // Bitwise Or
pub static ALU_AND_R: u8  = 0b1100_0011; // Bitwise And
pub static ALU_XOR_R: u8  = 0b1100_0100; // Bitwise Xor
pub static ALU_SUB_R: u8  = 0b1100_0101; // Subtraction
pub static ALU_MUL_R: u8  = 0b1100_0110; // Multiplication

pub static LOAD: u8     = 0b0000_0001; // Konstante in Accu laden
pub static LOAD_R: u8   = 0b0100_0001; // Wert von RAM Adr in Accu laden
pub static STORE: u8     = 0b0000_0010; // Accu an RAM Adr speichern

pub static JNZ: u8  = 0b0010_0001; // Jump if not zero

// Damit der PC und die Flags etc im Server nicht trivial verschlüsselt werden muss
pub static ZERO_INITIALIZER: u8 = 0;
pub static PC_INIT_VALUE: u8 = 0;
