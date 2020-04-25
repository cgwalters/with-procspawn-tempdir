use with_procspawn_tempdir::with_procspawn_tempdir;
use anyhow::{bail, Result};

#[with_procspawn_tempdir]
fn basic() -> Result<()> {
    assert!(std::path::Path::new(".procspawn-tmpdir").exists());
    Ok(())
}

#[with_procspawn_tempdir]
fn wrapped_err() -> Result<()> {
    assert!(std::path::Path::new(".procspawn-tmpdir").exists());
    bail!("oops");
}

fn err() -> Result<()> {
    assert!(!std::path::Path::new(".procspawn-tmpdir").exists());
    assert!(wrapped_err().is_err());
    Ok(())
}

fn main() -> Result<()> {
    procspawn::init();
    basic()?;
    println!("ok basic");
    err()?;
    println!("ok err");
    Ok(())
}