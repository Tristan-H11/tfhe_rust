//------ Konfiguration der Maschinensprache
// Dies ist relevant, damit im Server bekannt ist, mit welchen OpCodes der ausführende
// Befehl verglichen werden muss. Somit müssen die OpCodes nicht trivial verschlüsselt werden.

// Setzen der OpCodes
pub static ALU_ADD: u8  = 0b0000_0001; // Addition
pub static ALU_OR: u8   = 0b0000_0010; // Bitwise Or
pub static ALU_AND: u8  = 0b0000_0011; // Bitwise And
pub static ALU_XOR: u8  = 0b0000_0100; // Bitwise Xor
pub static ALU_SUB: u8  = 0b0000_0101; // Subtraction
pub static ALU_MUL: u8  = 0b0000_0110; // Multiplication

pub static ALU_ADD_R: u8  = 0b0000_0111; // Addition
pub static ALU_OR_R: u8   = 0b0000_1000; // Bitwise Or
pub static ALU_AND_R: u8  = 0b0000_1001; // Bitwise And
pub static ALU_XOR_R: u8  = 0b0000_1010; // Bitwise Xor
pub static ALU_SUB_R: u8  = 0b0000_1011; // Subtraction
pub static ALU_MUL_R: u8  = 0b0000_1100; // Multiplication

pub static LOAD: u8     = 0b0000_1101; // Konstante in Accu laden
pub static LOAD_R: u8   = 0b0000_1110; // Wert von RAM Adr in Accu laden
pub static STORE: u8     = 0b0000_1111; // Accu an RAM Adr speichern

pub static JNZ: u8  = 0b0001_0000; // Jump if not zero

// Damit der PC und die Flags etc im Server nicht trivial verschlüsselt werden muss
pub static ZERO_INITIALIZER: u8 = 0;
pub static PC_INIT_VALUE: u8 = 0;
