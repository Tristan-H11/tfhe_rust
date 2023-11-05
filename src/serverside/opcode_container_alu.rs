use tfhe::prelude::*;
use tfhe::FheUint8;
use crate::encrypt_trivial;

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
    alu_ram_mask: FheUint8,
    alu_const_mask: FheUint8,
}

impl OpcodeContainerAlu {
    pub(crate) fn new() -> OpcodeContainerAlu {
        OpcodeContainerAlu {
            add: encrypt_trivial!(0b1000_0001u8),
            or: encrypt_trivial!(0b1000_0010u8),
            and: encrypt_trivial!(0b1000_0011u8),
            xor: encrypt_trivial!(0b1000_0100u8),
            sub: encrypt_trivial!(0b1000_0101u8),
            mul: encrypt_trivial!(0b1000_0110u8),
            add_r: encrypt_trivial!(0b1100_0001u8),
            or_r: encrypt_trivial!(0b1100_0010u8),
            and_r: encrypt_trivial!(0b1100_0011u8),
            xor_r: encrypt_trivial!(0b1100_0100u8),
            sub_r: encrypt_trivial!(0b1100_0101u8),
            mul_r: encrypt_trivial!(0b1100_0110u8),
            alu_ram_mask: encrypt_trivial!(0b1100_0000u8),
            alu_const_mask: encrypt_trivial!(0b1000_0000u8),
        }
    }
}

impl OpcodeContainerAlu {
    /// Prüft, ob der OpCode einen ALU-Befehl repräsentiert
    pub fn contains_opcode(&self, opcode: &FheUint8) -> FheUint8 {
        self.is_ram_opcode(opcode) | self.is_constant_opcode(opcode)
    }

    /// Prüft, ob es sich um einen ALU-Opcode handelt, welcher einen Wert aus dem RAM auslesen muss
    pub fn is_ram_opcode(&self, opcode: &FheUint8) -> FheUint8 {
        (opcode & &self.alu_ram_mask).eq(&self.alu_ram_mask)
    }

    /// Prüft, ob es sich um einen ALU-Opcode handelt, welcher einen Wert als Konstante erwartet.
    pub fn is_constant_opcode(&self, opcode: &FheUint8) -> FheUint8 {
        let one: FheUint8 = encrypt_trivial!(1u8);
        let msb_equal = (opcode & &self.alu_const_mask).eq(&self.alu_const_mask);
        let not_ram_flag = one - self.is_ram_opcode(opcode);
        msb_equal.eq(not_ram_flag) // (Erstes Bit gesetzt) == (zweites Bit nicht gesetzt)
    }

    pub fn is_add(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.add) | opcode.eq(&self.add_r)
    }

    pub fn is_and(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.and) | opcode.eq(&self.and_r)
    }

    pub fn is_or(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.or) | opcode.eq(&self.or_r)
    }

    pub fn is_xor(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.xor) | opcode.eq(&self.xor_r)
    }

    pub fn is_sub(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.sub) | opcode.eq(&self.sub_r)
    }

    pub fn is_mul(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.mul) | opcode.eq(&self.mul_r)
    }
}
