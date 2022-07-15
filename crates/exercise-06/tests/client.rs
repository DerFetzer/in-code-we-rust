use assert_cmd::Command;
use std::time::Duration;

#[test]
fn echo_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = std::process::Command::new("cargo")
        .args(["run", "--bin", "exercise-06"])
        .spawn()?;

    std::thread::sleep(Duration::from_secs(1));

    let mut cli_cmd = Command::cargo_bin("client")?;
    let assert = cli_cmd
        .args(["127.0.0.1", "11223"])
        .write_stdin("test\n")
        .timeout(Duration::from_secs(1))
        .assert();

    server.kill()?;

    assert.failure().stdout("test\n");

    Ok(())
}
