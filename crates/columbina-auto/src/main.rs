//! Columbina Auto: a top-down open-world driving game built on Sandbina.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let title = title();
    // `--version` prints the title and exits without a window (the tests and the CI use it).
    if std::env::args().nth(1).as_deref() == Some("--version") {
        println!("{title}");
        return Ok(());
    }
    sandbina::run(&title)?;
    Ok(())
}

/// Game title with the game and engine versions, shown on the window.
fn title() -> String {
    let game = env!("CARGO_PKG_VERSION");
    let engine = sandbina::VERSION;
    format!("Columbina Auto v{game} (Sandbina v{engine})")
}
