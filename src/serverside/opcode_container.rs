use tfhe::FheUint8;
use tfhe::prelude::FheEq;
use crate::serverside::opcode_container_alu::OpcodeContainerAlu;

/// Datenstruktur zum Speichern aller Opcodes und ausführen einfacher inhaltlicher Abfragen.
#[derive(Clone)]
pub struct OpcodeContainer {
    pub(crate) opcodes_alu: OpcodeContainerAlu,
    pub(crate) load: FheUint8,
    pub(crate) load_r: FheUint8,
    pub(crate) save: FheUint8,
}

impl OpcodeContainer {
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
        self.opcodes_alu.contains_opcode(opcode)
    }

    /// Prüft, ob es sich um einen Command handelt, welcher einen Wert in den RAM schreiben soll.
    pub fn is_write_to_ram(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.save)
    }
}