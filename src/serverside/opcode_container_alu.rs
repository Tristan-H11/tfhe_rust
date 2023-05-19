use tfhe::FheUint8;
use tfhe::prelude::*;

/// Datenstruktur zum Speichern aller ALU-Opcodes und Ausführen einfacher inhaltlicher Abfragen.
#[derive(Clone)]
pub struct OpcodeContainerAlu {
    pub(crate) add: FheUint8,
    pub(crate) or: FheUint8,
    pub(crate) and: FheUint8,
    pub(crate) xor: FheUint8,
    pub(crate) sub: FheUint8,
    pub(crate) mul: FheUint8,
    pub(crate) add_r: FheUint8,
    pub(crate) or_r: FheUint8,
    pub(crate) and_r: FheUint8,
    pub(crate) xor_r: FheUint8,
    pub(crate) sub_r: FheUint8,
    pub(crate) mul_r: FheUint8,
}

impl OpcodeContainerAlu {
    /// Prüft, ob der OpCode einen ALU-Befehl repräsentiert
    pub fn contains_opcode(&self, opcode: &FheUint8) -> FheUint8 {
        self.is_ram_opcode(opcode)
            | self.is_constand_opcode(opcode)
    }

    /// Prüft, ob es sich um einen ALU-Opcode handelt, welcher einen Wert aus dem RAM auslesen muss
    pub fn is_ram_opcode(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.add_r)
            | opcode.eq(&self.or_r)
            | opcode.eq(&self.and_r)
            | opcode.eq(&self.xor_r)
            | opcode.eq(&self.sub_r)
            | opcode.eq(&self.mul_r)
    }

    /// Prüft, ob es sich um einen ALU-Opcode handelt, welcher einen Wert als Konstante erwartet.
    pub fn is_constand_opcode(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.add)
            | opcode.eq(&self.or)
            | opcode.eq(&self.and)
            | opcode.eq(&self.xor)
            | opcode.eq(&self.sub)
            | opcode.eq(&self.mul)
    }

    pub fn is_add(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.add)
            | opcode.eq(&self.add_r)
    }

    pub fn is_and(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.and)
            | opcode.eq(&self.and_r)
    }

    pub fn is_or(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.or)
            | opcode.eq(&self.or_r)
    }

    pub fn is_xor(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.xor)
            | opcode.eq(&self.xor_r)
    }

    pub fn is_sub(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.sub)
            | opcode.eq(&self.sub_r)
    }

    pub fn is_mul(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.mul)
            | opcode.eq(&self.mul_r)
    }
}