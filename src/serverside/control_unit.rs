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
    op_load: FheUint8,
    op_save: FheUint8,
}

impl ControlUnit {
    pub fn new(
        op_alu_add: FheUint8,
        op_alu_or: FheUint8,
        op_alu_and: FheUint8,
        op_alu_xor: FheUint8,
        op_load: FheUint8,
        op_save: FheUint8,
        zero_initializer: FheUint8,
        pc_init_value: FheUint8,
        program_data: Vec<(FheUint8, FheUint8)>,
        ram_size: usize
    ) -> ControlUnit {
        let alu = Alu {
            opcode_add: op_alu_add.clone(),
            opcode_and: op_alu_or.clone(),
            opcode_or: op_alu_and.clone(),
            opcode_xor: op_alu_xor.clone(),
            zero_flag: zero_initializer.clone(),
            overflow_flag: zero_initializer.clone(),
            carry_flag: zero_initializer.clone(),
        };

        let memory = MemoryUint8::new(
            zero_initializer.clone(),
            program_data,
            ram_size
        );

        ControlUnit {
            alu,
            memory,
            program_counter: pc_init_value,
            op_alu_add,
            op_alu_or,
            op_alu_and,
            op_alu_xor,
            op_load,
            op_save,
        }
    }

    pub fn get_ram(&mut self) -> Vec<(FheUint8, FheUint8)> {
        self.memory.get_data()
    }

    /// Führt die Fetch, Decode, execute, writeback Zyklen für die übergebene Anzahl an Zyklen aus.
    pub fn start(&mut self, cycles: u8) {
        println!("CU gestartet.");
        // Weil das Programm im cipherspace nicht terminieren kann, erstmal fixe cycles laufen lassen.
        for i in 0..cycles {
            println!("CU-Zyklus {} gestartet.", i);
            let memory_cell: (FheUint8, FheUint8) = self.memory.read_from_ram(&self.program_counter);
            let opcode: FheUint8 = memory_cell.0;
            let operand: FheUint8 = memory_cell.1;
            let accu: &FheUint8 = &self.memory.get_accu().clone();
            println!("CU Operanden und Accu ausgelesen.");

            // Boolscher Wert für "der akku bekommt einen neuen Wert"
            let is_write_alu_to_accu: FheUint8 = opcode.eq(&self.op_alu_add)
                | opcode.eq(&self.op_alu_and)
                | opcode.eq(&self.op_alu_or)
                | opcode.eq(&self.op_alu_xor);
            let is_write_value_to_accu: FheUint8 = opcode.eq(&self.op_load);

            let is_write_accu: FheUint8 = &is_write_alu_to_accu | &is_write_value_to_accu;
            println!("CU IsWriteAccu ausgewertet.");

            // Boolscher Wert für "der ram bekommt einen neuen Wert"
            let is_write_ram: FheUint8 = opcode.eq(&self.op_save);
            println!("CU IsWriteRam ausgewertet.");

            // Akku-Wert an die Adresse OPERAND shreiben, wenn geschrieben werden soll.
            self.memory.write_to_ram(
                &operand,
                accu.clone(),
                &is_write_ram,
            );
            println!("CU möglichen Schreibzugriff im RAM getätigt");

            let alu_result = self.alu.calculate(opcode.clone(), operand.clone(), accu.clone());
            println!("CU mögliches ALU Ergebnis bestimmt.");

            let possible_new_accu_value = alu_result * is_write_alu_to_accu + operand.clone() * is_write_value_to_accu;

            self.memory.write_accu(possible_new_accu_value, &is_write_accu);
        }
    }
}
