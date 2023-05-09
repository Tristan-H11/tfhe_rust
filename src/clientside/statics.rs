//------ Konfiguration der Maschinensprache
// Setzen der ALU OpCodes
pub static ALU_ADD: u8 = 0b0000_0000;
pub static ALU_AND: u8 = 0b0000_0001;
pub static ALU_OR: u8  = 0b0000_0010;
pub static ALU_XOR: u8 = 0b0000_0011;

// Adressen der Register im RAM, damit im Server nicht trivial verschlüsselt werden muss.
pub static REG1_ADR: u8 = 0;
pub static REG2_ADR: u8 = 1;
pub static REG3_ADR: u8 = 2;
pub static REG4_ADR: u8 = 3;

//TODO Die Befehle aus Architektur.md hier umsetzen. Relevant sind nur die unteren 6 bit.
// Gespeichert werden müssen die trotzdem als 16 bit
//----- Konfiguration des Nutzerprogramms
// Setzen des gewünschten Befehls
pub static OP_CODE: u8 = 0b0000_000; //XOR
pub static OP_A: u8 = 0b0000_0011;
pub static OP_B: u8 = 0b0000_0001;
