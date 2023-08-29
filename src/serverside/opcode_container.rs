use tfhe::FheUint8;
use tfhe::prelude::*;
use crate::serverside::opcode_container_alu::OpcodeContainerAlu;

/// Datenstruktur zum Speichern aller Opcodes und ausführen einfacher inhaltlicher Abfragen.
#[derive(Clone)]
pub struct OpcodeContainer {
    pub(crate) opcodes_alu: OpcodeContainerAlu,
    pub(crate) load: FheUint8,
    pub(crate) load_r: FheUint8,
    pub(crate) store: FheUint8,
    pub(crate) jnz: FheUint8,
}

impl OpcodeContainer {

    pub fn new() -> OpcodeContainer {
        let opcodes_alu = OpcodeContainerAlu::new();
        
        OpcodeContainer{
            opcodes_alu,
            load: FheUint8::try_encrypt_trivial(0b0000_0001u8).unwrap(),
            load_r: FheUint8::try_encrypt_trivial(0b0100_0001u8).unwrap(),
            store: FheUint8::try_encrypt_trivial(0b0000_0010u8).unwrap(),
            jnz: FheUint8::try_encrypt_trivial(0b0010_0001u8).unwrap(),
        }
    }
    /// Prüft, ob es sich um einen Command handelt, welcher einen Wert aus dem RAM in den Akkumulator laden soll.
    pub fn is_load_command(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.load)
            | opcode.eq(&self.load_r)
    }

    /// Prüft, ob es sich um einen Command handelt, welcher einen Befehl aus dem RAM laden muss.
    pub fn has_to_load_operand_from_ram(&self, opcode: &FheUint8) -> FheUint8 {
        self.opcodes_alu.is_ram_opcode(opcode)
            | self.load_r.eq(opcode)
    }

    /// Prüft, ob es sich um einen Command handelt, welcher eine ALU-Berechnung auslösen soll.
    pub fn is_alu_command(&self, opcode: &FheUint8) -> FheUint8 {
        let alu_mask: &FheUint8 = &FheUint8::try_encrypt_trivial(0b1000_0000u8).unwrap();
        (opcode & alu_mask).eq(alu_mask)
    }

    /// Prüft, ob es sich um einen Command handelt, welcher einen Wert in den RAM schreiben soll.
    pub fn is_write_to_ram(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.store)
    }
    
    /// Prüft, ob es sich um einen Sprungbefehl handelt.
    pub fn is_jump_command(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.jnz)
    }
}