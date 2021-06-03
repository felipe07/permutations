use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn succeeds_if_binari_called_with_supported_input() {
    Command::cargo_bin("permutations")
        .unwrap()
        .write_stdin("1, 2, 3")
        .assert()
        .stdout("[1, 2, 3]\n[2, 1, 3]\n[3, 1, 2]\n[1, 3, 2]\n[2, 3, 1]\n[3, 2, 1]\n");
}

#[test]
fn panics_if_binari_called_with_unsupported_number_separator() {
    Command::cargo_bin("permutations")
        .unwrap()
        .write_stdin("1; 2; 3")
        .assert()
        .stderr(predicate::str::contains("An error occurred while parsing input to numbers"));
}

#[test]
fn panics_if_binari_called_with_unsupported_input() {
    Command::cargo_bin("permutations")
        .unwrap()
        .write_stdin("hola")
        .assert()
        .stderr(predicate::str::contains("An error occurred while parsing input to numbers"));
}
