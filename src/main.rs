use crate::clientside::{client, verify};
use crate::serverside::server;

mod clientside;
mod serverside;

/// Einstiegspunkt für die Entwicklung.
/// Später müssen die drei `start()` Funktionen die Main-Funktionen der entsprechenden
/// Client- oder Server-Programme werden.
fn main() {
    client::start().expect("Fehler im Client!");
    server::start().expect("Fehler im Server!");
    verify::start().expect("Fehler im Verify!");
}

/*
/// https://docs.zama.ai/tfhe-rs/high-level-api/operations#arithmetic-operations.-1
*/
