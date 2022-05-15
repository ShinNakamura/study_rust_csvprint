use assert_cmd::Command;
// use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_stdinout() -> TestResult {
    let input = fs::read_to_string("tests/input/testin.csv")?;
    let expected = fs::read_to_string("tests/expected/testout.txt")?;
    let mut cmd = Command::cargo_bin("csvprint")?;
    cmd.write_stdin(input).assert().success().stdout(expected);
    Ok(())
}

