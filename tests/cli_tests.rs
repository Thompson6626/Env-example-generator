use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_cli_generates_env_example() {
    let test_input = "DB_USER=root\nDB_PASS=secret\n# Comment\nAPI_KEY=12345";
    fs::write("tests/test.env", test_input).unwrap();

    let mut cmd = Command::cargo_bin("ghostenv").unwrap();
    cmd.args(&["-i", "tests/test.env", "-o", "tests/test.env.example"])
        .assert()
        .success();

    let output = fs::read_to_string("tests/test.env.example").unwrap();
    let expected_output = "DB_USER=\nDB_PASS=\nAPI_KEY=";
    assert_eq!(output.trim(), expected_output);
}

#[test]
fn test_cli_fails_on_missing_file() {
    let mut cmd = Command::cargo_bin("ghostenv").unwrap();
    cmd.args(&["-i", "tests/non_existent.env", "-o", "tests/output.env"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Input file \'non_existent.env\' not found"));
}
