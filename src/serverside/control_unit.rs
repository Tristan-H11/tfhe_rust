use tfhe::FheUint16;
use crate::serverside::alu::Alu;
use crate::serverside::memory_uint16::MemoryUint16;

pub struct ControlUnit {
    alu: Alu,
    data_memory: MemoryUint16,
    program_memory: MemoryUint16,
    program_counter: FheUint16
}

impl ControlUnit {
    pub fn start() {
        // Loopen
    }

    fn fetch() {
        // Wert per program_counter aus dem program_memory laden
    }

    fn decode() {
        // Den gefetchten Wert mit 0b0000_0000_0011_1111 verunden, damit man den
    }

    fn execute() {
        // decodeten Befehl ausführen
        // gefetchten Wert um 6 shiften und dann Masken für die Parameter auflegen
    }

    fn step() {
        // PC inkrementieren
    }
}
