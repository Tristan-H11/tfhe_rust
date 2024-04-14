use crate::encrypt_trivial;
use crate::serverside::opcode_container_alu::OpcodeContainerAlu;
use tfhe::prelude::*;
use tfhe::{FheBool, FheUint8};

///
/// Datenstruktur zum Speichern aller Opcodes und Ausführen einfacher inhaltlicher Abfragen.
///
#[derive(Clone)]
pub struct OpcodeContainer {
    pub(crate) opcodes_alu: OpcodeContainerAlu,
    pub(crate) load: FheUint8,
    pub(crate) load_r: FheUint8,
    pub(crate) store: FheUint8,
    pub(crate) jnz: FheUint8,
}

impl OpcodeContainer {
    ///
    /// Erzeugt einen neuen OpcodeContainer.
    ///
    /// # Returns
    /// * Einen neuen OpcodeContainer.
    pub fn new() -> OpcodeContainer {
        let opcodes_alu = OpcodeContainerAlu::new();

        OpcodeContainer {
            opcodes_alu,
            load: encrypt_trivial!(0b0000_0001u8),
            load_r: encrypt_trivial!(0b0100_0001u8),
            store: encrypt_trivial!(0b0000_0010u8),
            jnz: encrypt_trivial!(0b0010_0001u8),
        }
    }
    ///
    /// Prüft, ob es sich um einen Command handelt, welcher einen Wert aus dem RAM in den Akkumulator laden soll.
    ///
    /// # Arguments
    /// * `opcode` - Der zu prüfende OpCode.
    ///
    /// # Returns
    /// * `1`, wenn es sich um einen Load-Command handelt, sonst `0`.
    pub fn is_load_command(&self, opcode: &FheUint8) -> FheBool {
        opcode.eq(&self.load) | opcode.eq(&self.load_r)
    }

    ///
    /// Prüft, ob es sich um einen Command handelt, welcher einen Befehl aus dem RAM laden muss.
    ///
    /// # Arguments
    /// * `opcode` - Der zu prüfende OpCode.
    ///
    /// # Returns
    /// * `1`, wenn es sich um einen Load-Operand-Command handelt, sonst `0`.
    pub fn has_to_load_operand_from_ram(&self, opcode: &FheUint8) -> FheBool {
        self.opcodes_alu.is_ram_opcode(opcode) | self.load_r.eq(opcode)
    }

    ///
    /// Prüft, ob es sich um einen Command handelt, welcher eine ALU-Berechnung auslösen soll.
    ///
    /// # Arguments
    /// * `opcode` - Der zu prüfende OpCode.
    ///
    /// # Returns
    /// * `1`, wenn es sich um einen ALU-Command handelt, sonst `0`.
    pub fn is_alu_command(&self, opcode: &FheUint8) -> FheBool {
        let alu_mask: &FheUint8 = &encrypt_trivial!(0b1000_0000u8);
        (opcode & alu_mask).eq(alu_mask)
    }

    ///
    /// Prüft, ob es sich um einen Command handelt, welcher einen Wert in den RAM schreiben soll.
    ///
    /// # Arguments
    /// * `opcode` - Der zu prüfende OpCode.
    ///
    /// # Returns
    /// * `1`, wenn es sich um einen Store-Command handelt, sonst `0`.
    pub fn is_write_to_ram(&self, opcode: &FheUint8) -> FheBool {
        opcode.eq(&self.store)
    }

    ///
    /// Prüft, ob es sich um einen Sprungbefehl handelt.
    ///
    /// # Arguments
    /// * `opcode` - Der zu prüfende OpCode.
    ///
    /// # Returns
    /// * `1`, wenn es sich um einen Sprungbefehl handelt, sonst `0`.
    pub fn is_jump_command(&self, opcode: &FheUint8) -> FheBool {
        opcode.eq(&self.jnz)
    }
}
