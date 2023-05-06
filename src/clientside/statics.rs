//------ Konfiguration der Maschinensprache
// Setzen der ALU OpCodes
pub static ALU_ADD: u8 = 0b0000_0000;
pub static ALU_AND: u8 = 0b0000_0001;
pub static ALU_OR: u8  = 0b0000_0010;
pub static ALU_XOR: u8 = 0b0000_0011;

// Setzen der RAM-Befehle
pub static RAM_READ: u8  = 0b0000_0100;
pub static RAM_WRITE: u8 = 0b0000_0101;

//TODO Die Befehle aus Architektur.md hier umsetzen. Relevant sind nur die unteren 6 bit.
// Gespeichert werden müssen die trotzdem als 16 bit
//----- Konfiguration des Nutzerprogramms
// Setzen des gewünschten Befehls
pub static OP_CODE: u8 = 0b0000_0011; //XOR
pub static OP_A: u8 = 0b1100_0011;
pub static OP_B: u8 = 0b1000_0001;
