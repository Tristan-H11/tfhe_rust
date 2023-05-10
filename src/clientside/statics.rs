//------ Konfiguration der Maschinensprache
// Dies ist relevant, damit im Server bekannt ist, mit welchen OpCodes der ausführende
// Befehl verglichen werden muss. Somit müssen die OpCodes nicht trivial verschlüsselt werden.

// Setzen der ALU OpCodes
pub static ALU_ADD_REGRAM: u16 = 0b0000_0000;
pub static ALU_ADD_REGREG: u16 = 0b0000_0001;

pub static ALU_AND_REGRAM: u16 = 0b0000_0010;
pub static ALU_AND_REGREG: u16 = 0b0000_0011;

pub static ALU_OR_REGRAM: u16 = 0b0000_0100;
pub static ALU_OR_REGREG: u16 = 0b0000_0101;

pub static ALU_XOR_REGRAM: u16 = 0b0000_0110;
pub static ALU_XOR_REGREG: u16 = 0b0000_0111;

// Setzen der Transportbefehle
pub static MOV_RAMREG: u16 = 0b0001_0000;
pub static MOV_REGRAM: u16 = 0b0001_0001;
pub static LOAD_CONST_REG: u16 = 0b0001_0010;
pub static SWAP_REGREG: u16 = 0b0001_0011;
pub static OUT_RAM: u16 = 0b0001_0100;

// Setzen der Programmflussbefehle
pub static JMP: u16 = 0b0010_0000;
pub static JMPC: u16 = 0b0010_0001;
pub static JMPO: u16 = 0b0010_0010;
pub static JMPZ: u16 = 0b0010_0011;
pub static JMPR: u16 = 0b0010_0100;
pub static END: u16 = 0b0011_0000;

// Adressen der Register im RAM, damit im Server nicht trivial verschlüsselt werden muss.
pub static REG1_ADR: u16 = 0;
pub static REG2_ADR: u16 = 1;
pub static REG3_ADR: u16 = 2;
pub static REG4_ADR: u16 = 3;

// Damit der PC und die Flags etc im Server nicht trivial verschlüsselt werden muss
pub static ZERO_INITIALIZER: u16 = 0;


//----- Konfiguration des Nutzerprogramms
pub static LOAD_2_TO_REG1: u16 = ((0b0000_0010 << 8) | (REG1_ADR << 6) | LOAD_CONST_REG) as u16;
pub static LOAD_1_TO_REG2: u16 = ((0b0000_0001 << 8) | (REG2_ADR << 6) | LOAD_CONST_REG) as u16;
pub static ADD_REG1_REG2: u16 = ((REG1_ADR << 8) | (REG2_ADR << 6) | ALU_ADD_REGREG) as u16;
pub static OUT_REG1: u16 = ((REG1_ADR << 8) | (OUT_RAM)) as u16;
