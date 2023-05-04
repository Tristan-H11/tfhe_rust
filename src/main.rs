mod memory_access;
mod alu;
mod server;
mod client;
mod verify;

fn main() {
    client::start().expect("Fehler im Client!");
    server::start().expect("Fehler im Server!");
    verify::start().expect("Fehler im Verify!");
}

/*
/// https://docs.zama.ai/tfhe-rs/high-level-api/operations#arithmetic-operations.-1
*/
