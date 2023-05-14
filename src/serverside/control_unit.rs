use tfhe::FheUint8;
use tfhe::prelude::FheTryTrivialEncrypt;
use crate::serverside::alu::Alu;
use crate::serverside::memory_uint8::MemoryUint8;

pub struct ControlUnit {
    alu: Alu,
    data_memory: MemoryUint8,
    program_memory: MemoryUint8,
    program_counter: FheUint8,
}

impl ControlUnit {

    pub fn new(alu: Alu, data: MemoryUint8, program: MemoryUint8, pc: FheUint8) {
        // TODO
    }

    pub fn start(&mut self) {


        // Loopen
        while true {
            let operand = ();
            let opcode = ();
            let accu = ();

            let is_write_accu = ();
            let is_write_ram = ();

            let alu_result = self.alu.calculate(&opcode, &operand, &accu);
            memory.write_to_ram(
                FheUint8::try_encrypt_trivial(0 as u8).unwrap(),
                a.clone(),
                is_write_ram
            );

        }
    }
}
