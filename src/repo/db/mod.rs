mod history;
mod data;

use std::{
    io, fs,
    path::{Path, PathBuf},
    net::ToSocketAddrs,
};
use fs2::{FileExt};
use super::fsx;
use history::Tree;

pub type Id = [u8; 32];

pub struct Db {
    path: PathBuf,
    lock: fs::File,
}

impl Db {
    fn create_lock(path: &Path) -> io::Result<()> {
        fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path.join(".hllv/lock"))
        .map(|_f| ())
    }

    fn lock(path: &Path) -> io::Result<fs::File> {
        fs::File::open(&path.join(".hllv/lock"))
        .and_then(|f| {
            f.try_lock_exclusive()
            .map(|()| f)
        }) 
    }

    pub fn new(path: &Path) -> io::Result<Self> {
        match fsx::metadata(path) {
            Ok(m) => m.check_dir(),
            Err(_) => fsx::create_dir(path),
        }
        .and_then(|()| fsx::create_dir(&path.join(".hllv")))
        .and_then(|()| Self::create_lock(path))
        .and_then(|()| Self::open(path))
    }
    pub fn open(path: &Path) -> io::Result<Self> {
        fsx::metadata(&path.join(".hllv"))
        .and_then(|m| m.check_dir())
        .and_then(|()| Self::lock(path))
        .map(|lock| Self { path: path.to_path_buf(), lock })
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}
