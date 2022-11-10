use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// Utils
// --------------------------------------------------
type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult<()> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("configs")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_false_cmd() -> TestResult<()> {
    Command::cargo_bin("configs")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_cat() -> TestResult<()> {
    run(
        &["read", "-p", "tests/expected/hello.txt", "-r", "Hello"],
        "tests/expected/hello.txt",
    )
}
// --------------------------------------------------
#[test]
fn test_echo() -> TestResult<()> {
    run(
        &["echo", "-p", "Hello world", "-r", "xx"],
        "tests/expected/hello.txt",
    )
}
