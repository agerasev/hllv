use std::{
    io, mem::drop,
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
    let tmp = TempDir::new("new_create")?;
    Repo::new(&tmp.path().join("repo"))?;
    fsx::metadata(&tmp.path().join("repo/.hllv"))?.check_dir()?;
    Ok(())
}

#[test]
fn new_twice() -> io::Result<()> {
    let tmp = TempDir::new("new_twice")?;
    Repo::new(tmp.path())?;
    assert!(Repo::new(tmp.path()).is_err());
    Ok(())
}

#[test]
fn open() -> io::Result<()> {
    let tmp = TempDir::new("open")?;
    Repo::new(tmp.path())?;
    Repo::open(tmp.path())?;
    Ok(())
}

#[test]
fn open_twice() -> io::Result<()> {
    let tmp = TempDir::new("open")?;
    Repo::new(tmp.path())?; // create repo
    let a = Repo::open(tmp.path())?; // open it
    assert!(Repo::open(tmp.path()).is_err()); // try open in parallel
    drop(a); // close repo
    Repo::open(tmp.path())?; // open again
    Ok(())
}
