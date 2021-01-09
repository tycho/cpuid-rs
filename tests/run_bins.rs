use assert_cmd::prelude::*; // Add methods on commands
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::Command; // Run programs
use predicates::prelude::*;

fn dump_path(name: &str) -> String {
    let mut pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pathbuf.push("resources/test/dumps");
    pathbuf.push(name);
    pathbuf.as_path().to_str().unwrap().to_string()
}

#[test]
fn dump() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dump")?;
    cmd.assert().success();
    Ok(())
}

#[test]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decode")?;
    cmd.assert().success();
    Ok(())
}

#[test]
fn decode_on_missing_dump_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decode")?;
    cmd.arg("-f")
        .arg("bogus-file-path")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn decode_on_existing_dump() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decode")?;
    cmd.arg("-f")
        .arg(&dump_path(
            "GenuineIntel/GenuineIntel00806C1_TigerLake_CPUID3.txt",
        ))
        .assert()
        .success();
    Ok(())
}

#[test]
fn dump_generates_identical_dump() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dump")?;
    let path = dump_path("GenuineIntel/GenuineIntel00006F6_Merom_CPUID.txt");
    let contents = read_to_string(&path)?;
    cmd.arg("-f")
        .arg(&path)
        .assert()
        .stdout(predicate::eq(contents.as_str()))
        .success();
    Ok(())
}
