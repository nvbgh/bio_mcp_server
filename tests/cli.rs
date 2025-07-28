use assert_cmd::Command;
use chrono::Local;
use predicates::prelude::*;

#[test]
fn test_ping_command() {
    let mut cmd = Command::cargo_bin("bio_mcp_server").unwrap();
    cmd.write_stdin("ping\n")
        .assert()
        .stdout(predicate::str::contains("pong"))
        .success();
}

#[test]
fn test_date_command() {
    let mut cmd = Command::cargo_bin("bio_mcp_server").unwrap();
    let today = Local::now().date_naive().to_string();
    cmd.write_stdin("date\n")
        .assert()
        .stdout(predicate::str::contains(today))
        .success();
}

#[test]
fn test_unknown_command() {
    let mut cmd = Command::cargo_bin("bio_mcp_server").unwrap();
    cmd.write_stdin("unknown\n")
        .assert()
        .stdout(predicate::str::contains("unknown command"))
        .success();
}
