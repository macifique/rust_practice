use std::error::Error;
use std::fs;
use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn usage() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs_with_text() -> TestResult{
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn h1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn h2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn h2b() -> TestResult {
    helper(&vec!["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn h2nb() -> TestResult {
    helper(&vec!["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

#[test]
fn h1nb() -> TestResult {
    helper(&vec!["-n", "Hello  there"], "tests/expected/hello1.n.txt")
}


#[test]
fn h1b() -> TestResult {
    helper(&vec!["Hello there"], "tests/expected/hello1.txt")
}



fn helper(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}