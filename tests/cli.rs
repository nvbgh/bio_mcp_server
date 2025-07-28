use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_echo_server() {
    let mut cmd = Command::cargo_bin("bio_mcp_server").unwrap();
    cmd.write_stdin("hello\n")
        .assert()
        .stdout(predicate::str::contains("hello"))
        .success();
}
