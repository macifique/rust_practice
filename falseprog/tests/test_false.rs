use assert_cmd::Command;

#[test]

fn test_false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure().stdout("Heldloka!\n");
}