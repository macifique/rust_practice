use assert_cmd::Command;

#[test]

fn runs() {
    let mut cmd = Command::cargo_bin("true").unwrap(); 2
    cmd.assert().success(); 3
}