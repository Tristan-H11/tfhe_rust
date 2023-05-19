//------ Konfiguration der Maschinensprache
// Dies ist relevant, damit im Server bekannt ist, mit welchen OpCodes der ausführende
// Befehl verglichen werden muss. Somit müssen die OpCodes nicht trivial verschlüsselt werden.

// Setzen der OpCodes
/*
ALU Befehle enden auf 0.
Nicht-ALU Befehl enden auf 1.
 */
pub static ALU_ADD: u8  = 1; // Addition
pub static ALU_OR: u8   = 2; // Bitwise Or
pub static ALU_AND: u8  = 3; // Bitwise And
pub static ALU_XOR: u8  = 4; // Bitwise Xor
pub static ALU_SUB: u8  = 5; // Subtraction
pub static ALU_MUL: u8  = 6; // Multiplication

pub static ALU_ADD_R: u8  = 7; // Addition
pub static ALU_OR_R: u8   = 8; // Bitwise Or
pub static ALU_AND_R: u8  = 9; // Bitwise And
pub static ALU_XOR_R: u8  = 10; // Bitwise Xor
pub static ALU_SUB_R: u8  = 11; // Subtraction
pub static ALU_MUL_R: u8  = 12; // Multiplication

pub static LOAD: u8     = 13; // Konstante in Accu laden
pub static LOAD_R: u8   = 14; // Wert von RAM Adr in Accu laden
pub static SAVE: u8     = 15; // Accu an RAM Adr speichern

pub static JNZ: u8  = 16; // Jump if not zero

// Damit der PC und die Flags etc im Server nicht trivial verschlüsselt werden muss
pub static ZERO_INITIALIZER: u8 = 0;
pub static PC_INIT_VALUE: u8 = 0;
