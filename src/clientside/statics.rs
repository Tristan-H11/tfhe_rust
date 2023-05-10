//------ Konfiguration der Maschinensprache
// Dies ist relevant, damit im Server bekannt ist, mit welchen OpCodes der ausführende
// Befehl verglichen werden muss. Somit müssen die OpCodes nicht trivial verschlüsselt werden.

// Setzen der ALU OpCodes
pub static ALU_ADD_REGRAM: u8 = 0b0000_0000;
pub static ALU_ADD_REGREG: u8 = 0b0000_0001;

pub static ALU_AND_REGRAM: u8 = 0b0000_0010;
pub static ALU_AND_REGREG: u8 = 0b0000_0011;

pub static ALU_OR_REGRAM: u8 = 0b0000_0100;
pub static ALU_OR_REGREG: u8 = 0b0000_0101;

pub static ALU_XOR_REGRAM: u8 = 0b0000_0110;
pub static ALU_XOR_REGREG: u8 = 0b0000_0111;

// Setzen der Transportbefehle
pub static MOV_RAMREG: u8 = 0b0001_0000;
pub static MOV_REGRAM: u8 = 0b0001_0001;
pub static LOAD_CONST_REG: u8 = 0b0001_0010;
pub static SWAP_REGREG: u8 = 0b0001_0011;
pub static OUT_RAM: u8 = 0b0001_0100;

// Setzen der Programmflussbefehle
pub static JMP: u8 = 0b0010_0000;
pub static JMPC: u8 = 0b0010_0001;
pub static JMPO: u8 = 0b0010_0010;
pub static JMPZ: u8 = 0b0010_0011;
pub static JMPR: u8 = 0b0010_0100;
pub static END: u8 = 0b0011_0000;

// Adressen der Register im RAM, damit im Server nicht trivial verschlüsselt werden muss.
pub static REG1_ADR: u8 = 0;
pub static REG2_ADR: u8 = 1;
pub static REG3_ADR: u8 = 2;
pub static REG4_ADR: u8 = 3;

// Damit der PC und die Flags etc im Server nicht trivial verschlüsselt werden muss
pub static ZERO_INITIALIZER: u8 = 0;

//----- Konfiguration des Nutzerprogramms
// Setzen des gewünschten Befehls
pub static OP_CODE: u8 = 0b0000_000; //XOR
pub static OP_A: u8 = 0b0000_0011;
pub static OP_B: u8 = 0b0000_0001;
