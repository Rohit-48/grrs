/* TEST CASES
What happens when the file doesn’t exist?
 What is the output when there is no match?
 Does our program exit with an error when we forget one (or both) arguments?
*/
use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
use predicates::prelude::*; // Used for writing assertions

#[test]
fn file_doesnt_exist() ->Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("grrs");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}


/// fixtures and assertions for testing.
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
// integration test 0
#[test]
fn passing_empty_str() -> Result<(), Box<dyn std::error::Error>>{
    let file = assert_fs::NamedTempFile::new("empty.txt")?;
    file.write_str("Line one\nLine two\nLine three")?;

    let mut cmd = cargo_bin_cmd!('grrs);
    cmd.arg("").arg(file.path());

    cmd.assert()
        .success()
        .stdout("Line one\nLine two\nLine three\n");
}
// integration test 1
#[test]
fn empty_pattern_should_fail() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Line one\nLine two")?;

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.arg("").arg(file.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("pattern must not be empty"));

    Ok(())
}