use std::{
    io,
};
use tempdir::TempDir;
use super::{fsx, Repo};

#[test]
fn new() -> io::Result<()> {
    let tmp = TempDir::new("new")?;
    Repo::new(tmp.path())?;
    fsx::metadata(&tmp.path().join(".hllv"))?.check_dir()?;
    Ok(())
}

#[test]
fn new_create() -> io::Result<()> {
    let tmp = TempDir::new("new")?;
    Repo::new(&tmp.path().join("repo"))?;
    fsx::metadata(&tmp.path().join("repo/.hllv"))?.check_dir()?;
    Ok(())
}

#[test]
fn new_twice() -> io::Result<()> {
    let tmp = TempDir::new("new")?;
    Repo::new(tmp.path())?;
    assert!(Repo::new(tmp.path()).is_err());
    Ok(())
}
