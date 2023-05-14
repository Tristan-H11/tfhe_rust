use crate::clientside::{client, verify};
use crate::serverside::server;

mod clientside;
mod serverside;

/// Einstiegspunkt für die Entwicklung.
/// Später müssen die drei `start()` Funktionen die Main-Funktionen der entsprechenden
/// Client- oder Server-Programme werden.
fn main() {
    client::start().expect("Fehler im Client!");
    println!("-------");
    println!("Client Ausführung beendet!");
    println!("-------");
    server::start().expect("Fehler im Server!");
    println!("-------");
    println!("Server Ausführung beendet!");
    println!("-------");
    verify::start().expect("Fehler im Verify!");
    println!("-------");
    println!("Verify Ausführung beendet!");
    println!("-------");
}