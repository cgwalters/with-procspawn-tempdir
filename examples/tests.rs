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

#[with_procspawn_tempdir]
fn nores() {
}

// #[with_procspawn_tempdir]
// fn nores_panic() {
//     panic!("oops")
// }

fn main() -> Result<()> {
    procspawn::init();
    let cwd = std::env::current_dir()?;
    basic()?;
    assert_eq!(cwd, std::env::current_dir()?);
    println!("ok basic");
    err()?;
    println!("ok err");
    nores();
    println!("ok nores");
    // assert!(std::panic::catch_unwind(|| { nores_panic() }).is_err());
    // println!("ok nores_panic");
    Ok(())
}