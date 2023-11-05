use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use tfhe::prelude::*;
use tfhe::{set_server_key, FheUint8, ServerKey};
use crate::encrypt_trivial;

use crate::serverside::alu::Alu;
use crate::serverside::memory_uint8::MemoryUint8;
use crate::serverside::opcode_container::OpcodeContainer;

///
/// Darstellung der ControlUnit.
///
/// Hält fachlich die Alu, den Speicher und den ProgramCounter.
///
pub struct ControlUnit {
    alu: Alu,
    memory: MemoryUint8,
    program_counter: FheUint8,
    opcodes: OpcodeContainer,
    server_key: ServerKey,
}

impl ControlUnit {
    ///
    /// Erzeugt eine neue ControlUnit.
    ///
    /// # Arguments
    /// * `opcodes` - Die Opcodes, die die ControlUnit kennen soll.
    /// * `zero_initializer` - Der Wert, mit dem die Flags initialisiert werden sollen.
    /// * `pc_init_value` - Der Wert, mit dem der ProgramCounter initialisiert werden soll.
    /// * `program_data` - Die Daten, die der Speicher initial halten soll.
    /// * `ram_size` - Die Größe des Speichers.
    /// * `server_key` - Der Schlüssel, der für die Berechnungen verwendet werden soll.
    ///
    /// # Returns
    /// * Eine neue ControlUnit.
    pub fn new(
        opcodes: OpcodeContainer,
        zero_initializer: FheUint8,
        pc_init_value: FheUint8,
        program_data: Vec<(FheUint8, FheUint8)>,
        ram_size: usize,
        server_key: ServerKey,
    ) -> ControlUnit {
        let alu = Alu {
            opcodes: opcodes.opcodes_alu.clone(),
            zero_flag: zero_initializer.clone(),
            overflow_flag: zero_initializer.clone(),
            carry_flag: zero_initializer.clone(),
        };

        let memory = MemoryUint8::new(program_data, ram_size);

        ControlUnit {
            alu,
            memory,
            program_counter: pc_init_value,
            opcodes,
            server_key,
        }
    }

    ///
    /// Gibt den Speicher zurück.
    ///
    pub fn get_ram(&mut self) -> Vec<(FheUint8, FheUint8)> {
        self.memory.get_data()
    }

    ///
    /// Startet die ControlUnit und folglich die gesamte Simulation.
    ///
    /// # Arguments
    /// * `cycles` - Die Anzahl der Zyklen, die die ControlUnit laufen soll.
    ///
    pub fn start(&mut self, cycles: u8) {
        println!("[ControlUnit] CU gestartet.");
        let one: &FheUint8 = &encrypt_trivial!(1u8);
        let mut accu: FheUint8 = encrypt_trivial!(0u8);

        for i in 1..(cycles + 1) {
            println!("\n[ControlUnit] Zyklus {} gestartet.", i);
            let start_cycle = Instant::now();
            let start_time = Instant::now();

            let memory_cell: (FheUint8, FheUint8) =
                self.memory.read_from_ram(&self.program_counter);
            let opcode: &FheUint8 = &memory_cell.0;
            let operand: &FheUint8 = &memory_cell.1;
            println!(
                "[ControlUnit, {}ms] Operanden und Accu ausgelesen.",
                start_time.elapsed().as_millis()
            );

            let program_counter_thread =
                self.calculate_program_counter(one, opcode, operand, self.server_key.clone());

            let start_time = Instant::now();
            let is_alu_command: FheUint8 = self.opcodes.is_alu_command(opcode);
            let is_load_command: FheUint8 = self.opcodes.is_load_command(opcode);
            let is_write_accu: FheUint8 = &is_alu_command | &is_load_command;

            let is_write_ram: FheUint8 = self.opcodes.is_write_to_ram(opcode);
            println!(
                "[ControlUnit, {}ms] IsWriteAccu und IsWriteRam ausgewertet.",
                start_time.elapsed().as_millis()
            );

            // Akku-Wert an die Adresse OPERAND schreiben, falls geschrieben werden soll.
            self.memory.write_to_ram(operand, &accu, &is_write_ram);
            println!("[ControlUnit] möglichen Schreibzugriff im RAM getätigt");

            let start_time = Instant::now();
            let has_to_load_operand_from_ram: FheUint8 =
                self.opcodes.has_to_load_operand_from_ram(opcode);
            let ram_value: FheUint8 = self.memory.read_from_ram(operand).1;
            let calculation_data: FheUint8 = operand * (one - &has_to_load_operand_from_ram)
                + ram_value * (has_to_load_operand_from_ram);
            println!(
                "[ControlUnit, {}ms] Operand (ob RAM oder Konstante) ausgewertet.",
                start_time.elapsed().as_millis()
            );

            let alu_result: FheUint8 =
                self.alu
                    .calculate(opcode, &calculation_data, &accu, &is_alu_command);
            println!("[ControlUnit] mögliches ALU Ergebnis bestimmt.");

            let start_time = Instant::now();
            let possible_new_accu_value: FheUint8 =
                alu_result * is_alu_command + calculation_data.clone() * is_load_command;
            let one: FheUint8 = encrypt_trivial!(1u8);
            accu = possible_new_accu_value * &is_write_accu + &accu * (one - &is_write_accu);

            println!(
                "[ControlUnit, {}ms] Akkumulatorwert bestimmt und geschrieben.",
                start_time.elapsed().as_millis()
            );

            self.program_counter = program_counter_thread.join().unwrap();

            println!(
                "[ControlUnit] Zyklus {} in {}ms beendet.",
                i,
                start_cycle.elapsed().as_millis()
            );
        }
    }

    ///
    /// Berechnet den neuen ProgramCounter.
    ///
    /// # Arguments
    /// * `one` - Die 1, die für die Berechnung benötigt wird.
    /// * `opcode` - Der Opcode, der für die Berechnung benötigt wird.
    /// * `operand` - Der Operand, der für die Berechnung benötigt wird.
    /// * `key` - Der Schlüssel, der für die Berechnung benötigt wird.
    ///
    /// # Returns
    /// * Den neuen ProgramCounter in einem JoinHandle.
    fn calculate_program_counter(
        &mut self,
        one: &FheUint8,
        opcode: &FheUint8,
        operand: &FheUint8,
        key: ServerKey,
    ) -> JoinHandle<FheUint8> {
        let pc = self.program_counter.clone();
        let zero_flag = self.alu.zero_flag.clone();
        let opcode = opcode.clone();
        let opcodes = self.opcodes.clone();
        let operand = operand.clone();
        let one = one.clone();
        thread::spawn(move || {
            set_server_key(key);
            let start_time = Instant::now();
            let is_jump: FheUint8 = opcodes.is_jump_command(&opcode);

            // (1 - cond) = !cond, weil 1 - 1 = 0 und 1 - 0 = 1.
            let is_no_jump: FheUint8 = &one - &is_jump;
            let incremented_pc: FheUint8 = pc + &one;
            let jnz_condition: FheUint8 = zero_flag * &is_jump;
            // pc = ((pc + 1) * noJump) + (operand * jump)
            let result = incremented_pc * is_no_jump + operand * jnz_condition;
            println!(
                "[ControlUnit, {}ms] ProgramCounter bestimmt und gesetzt.",
                start_time.elapsed().as_millis()
            );
            result
        })
    }
}
