use crate::clientside::{client, verify};
use crate::serverside::server;
use std::time::Instant;

mod clientside;
mod serverside;
pub mod macros;

/// Einstiegspunkt für die Entwicklung.
/// Später müssen die drei `start()` Funktionen die Main-Funktionen der entsprechenden
/// Client- oder Server-Programme werden.
fn main() {
    let start = Instant::now();
    client::start().expect("Fehler im Client!");
    println!("-------");
    println!(
        "Client Ausführung beendet! Zeit in ms: {}",
        (start.elapsed()).as_millis()
    );
    println!("-------");

    let start = Instant::now();
    server::start().expect("Fehler im Server!");
    println!("-------");
    println!(
        "Server Ausführung beendet! Zeit in ms: {}",
        (start.elapsed()).as_millis()
    );
    println!("-------");

    let start = Instant::now();
    verify::start().expect("Fehler im Verify!");
    println!("-------");
    println!(
        "Verify Ausführung beendet! Zeit in ms: {}",
        (start.elapsed()).as_millis()
    );
    println!("-------");
}
