//! End-to-end check: the real binary prints its title with `--version`, without a window.

use std::process::Command;

#[test]
fn prints_the_game_and_engine_versions() {
    let output = Command::new(env!("CARGO_BIN_EXE_columbina-auto"))
        .arg("--version")
        .output()
        .expect("the game binary should start");

    assert!(output.status.success(), "exit status: {}", output.status);
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.starts_with("Columbina Auto v"), "stdout: {stdout:?}");
    assert!(stdout.contains("(Sandbina v"), "stdout: {stdout:?}");
}
