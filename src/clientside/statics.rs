//------ Konfiguration der Maschinensprache
// Dies ist relevant, damit im Server bekannt ist, mit welchen OpCodes der ausführende
// Befehl verglichen werden muss. Somit müssen die OpCodes nicht trivial verschlüsselt werden.

// Setzen der OpCodes
pub static ALU_ADD: u8  = 0b0000_0010;
pub static ALU_OR: u8   = 0b0000_0100;
pub static ALU_AND: u8  = 0b0000_0110;
pub static ALU_XOR: u8  = 0b0000_1000;

pub static LOAD: u8     = 0b0000_0001;
pub static SAVE: u8     = 0b0000_0011;

// Damit der PC und die Flags etc im Server nicht trivial verschlüsselt werden muss
pub static ZERO_INITIALIZER: u8 = 0;
pub static PC_INIT_VALUE: u8 = 0;
