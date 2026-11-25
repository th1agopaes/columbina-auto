//! Columbina Auto: a top-down open-world driving game built on Sandbina.

fn main() {
    println!("{}", title());
}

/// Game title with the game and engine versions (the window title, later on).
fn title() -> String {
    let game = env!("CARGO_PKG_VERSION");
    let engine = sandbina::VERSION;
    format!("Columbina Auto v{game} (Sandbina v{engine})")
}
