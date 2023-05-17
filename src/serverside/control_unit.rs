use tfhe::FheUint8;
use tfhe::prelude::*;

use crate::serverside::alu::Alu;
use crate::serverside::memory_uint8::MemoryUint8;

pub struct ControlUnit {
    alu: Alu,
    memory: MemoryUint8,
    program_counter: FheUint8,
    // OP-Codes, damit die unterschiedlichen Modi arithmetisch ausgewertet werden können.
    op_alu_add: FheUint8,
    op_alu_or: FheUint8,
    op_alu_and: FheUint8,
    op_alu_xor: FheUint8,
    op_alu_sub: FheUint8,
    op_alu_mul: FheUint8,
    op_alu_add_r: FheUint8,
    op_alu_or_r: FheUint8,
    op_alu_and_r: FheUint8,
    op_alu_xor_r: FheUint8,
    op_alu_sub_r: FheUint8,
    op_alu_mul_r: FheUint8,
    op_load: FheUint8,
    op_load_r: FheUint8,
    op_save: FheUint8,
}

impl ControlUnit {
    pub fn new(
        op_alu_add: FheUint8,
        op_alu_or: FheUint8,
        op_alu_and: FheUint8,
        op_alu_xor: FheUint8,
        op_alu_sub: FheUint8,
        op_alu_mul: FheUint8,
        op_alu_add_r: FheUint8,
        op_alu_or_r: FheUint8,
        op_alu_and_r: FheUint8,
        op_alu_xor_r: FheUint8,
        op_alu_sub_r: FheUint8,
        op_alu_mul_r: FheUint8,
        op_load: FheUint8,
        op_load_r: FheUint8,
        op_save: FheUint8,
        zero_initializer: FheUint8,
        pc_init_value: FheUint8,
        program_data: Vec<(FheUint8, FheUint8)>,
        ram_size: usize,
    ) -> ControlUnit {
        let alu = Alu {
            opcode_add: op_alu_add.clone(),
            opcode_and: op_alu_and.clone(),
            opcode_or: op_alu_or.clone(),
            opcode_xor: op_alu_xor.clone(),
            zero_flag: zero_initializer.clone(),
            overflow_flag: zero_initializer.clone(),
            carry_flag: zero_initializer.clone(),
        };

        let memory = MemoryUint8::new(
            zero_initializer.clone(),
            program_data,
            ram_size,
        );

        ControlUnit {
            alu,
            memory,
            program_counter: pc_init_value,
            op_alu_add,
            op_alu_or,
            op_alu_and,
            op_alu_xor,
            op_alu_sub,
            op_alu_mul,
            op_alu_add_r,
            op_alu_or_r,
            op_alu_and_r,
            op_alu_xor_r,
            op_alu_sub_r,
            op_alu_mul_r,
            op_load,
            op_load_r,
            op_save,
        }
    }

    pub fn get_ram(&mut self) -> Vec<(FheUint8, FheUint8)> {
        self.memory.get_data()
    }

    /// Führt die Fetch, Decode, execute, writeback Zyklen für die übergebene Anzahl an Zyklen aus.
    pub fn start(&mut self, cycles: u8) {
        println!("[ControlUnit] CU gestartet.");
        let one: FheUint8 = FheUint8::try_encrypt_trivial(1 as u8).unwrap();
        let alu_mask: FheUint8 = FheUint8::try_encrypt_trivial(0b0000_1110 as u8).unwrap();
        let alu_from_ram_mask: FheUint8 = FheUint8::try_encrypt_trivial(0b0001_0000 as u8).unwrap();
        // Weil das Programm im cipherspace nicht terminieren kann, erstmal fixe cycles laufen lassen.
        for i in 0..cycles {
            println!("\n[ControlUnit] Zyklus {} gestartet.", i);
            let memory_cell: (FheUint8, FheUint8) = self.memory.read_from_ram(&self.program_counter);
            let opcode: FheUint8 = memory_cell.0;
            let operand: FheUint8 = memory_cell.1;
            let accu: FheUint8 = self.memory.get_accu().clone();
            println!("[ControlUnit] Operanden und Accu ausgelesen.");

            let is_alu_command: FheUint8 = self.is_alu_command(&opcode, &one);
            let is_load_command: FheUint8 = self.is_load_command(&opcode);
            let is_write_accu: FheUint8 = &is_alu_command | &is_load_command;
            println!("[ControlUnit] IsWriteAccu ausgewertet.");

            let is_write_ram: FheUint8 = opcode.eq(&self.op_save);
            println!("[ControlUnit] IsWriteRam ausgewertet.");

            // Akku-Wert an die Adresse OPERAND shreiben, wenn geschrieben werden soll.
            self.memory.write_to_ram(
                &operand,
                accu.clone(),
                &is_write_ram,
            );
            println!("[ControlUnit] möglichen Schreibzugriff im RAM getätigt");

            // Maskierung auf den Command-Anteil, damit sowohl x als auch x_R erkannt werden.
            let alu_opcode: FheUint8 = &opcode & &alu_mask;

            let is_operand_from_ram: FheUint8 = self.is_operand_from_ram(&opcode);
            let ram_value: FheUint8 = self.memory.read_from_ram(&operand).1;
            let calculation_data: FheUint8 = operand * (&one - &is_operand_from_ram)
                + ram_value * (is_operand_from_ram);
            println!("[ControlUnit] Operanden (RAM oder Konstante) ausgewertet.");

            let alu_result = self.alu.calculate(alu_opcode, calculation_data.clone(), accu.clone(), &is_alu_command);
            println!("[ControlUnit] mögliches ALU Ergebnis bestimmt.");

            let possible_new_accu_value = alu_result * is_alu_command + calculation_data.clone() * is_load_command;

            self.memory.write_accu(possible_new_accu_value, &is_write_accu);

            self.program_counter = &self.program_counter + &one;
        }
    }

    fn is_load_command(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.op_load)
            | opcode.eq(&self.op_load_r)
    }

    /// Es handelt sich um einen ALU-Command, wenn im letzten Bit eine 0 ist.
    fn is_alu_command(&self, opcode: &FheUint8, lsb_mask: &FheUint8) -> FheUint8 {
        lsb_mask - (opcode & lsb_mask)
        // 1 - LSB => Wenn das letzte Bit eine 1 (kein alu Befehl) ist, kommt 0 raus, ist es eine 0 (alu Befehl) kommt 1 raus.
    }
    fn is_operand_from_ram(&self, opcode: &FheUint8) -> FheUint8 {
        opcode.eq(&self.op_load_r)
            | opcode.eq(&self.op_alu_add_r)
            | opcode.eq(&self.op_alu_or_r)
            | opcode.eq(&self.op_alu_and_r)
            | opcode.eq(&self.op_alu_xor_r)
            | opcode.eq(&self.op_alu_mul_r)
            | opcode.eq(&self.op_alu_sub_r)
        // Das bit steht an 5. Stelle, daher muss geshiftet werden.
    }
}
