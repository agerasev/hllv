use std::{
    io, fs,
    path::{Path, PathBuf},
    net::ToSocketAddrs,
};
use fs2::{FileExt};
use super::fsx;


pub struct Db {
    path: PathBuf,
}

impl Db {
    pub fn new(path: &Path) -> io::Result<Self> {
        match fsx::metadata(path) {
            Ok(m) => m.check_dir(),
            Err(_) => fsx::create_dir(path),
        }
        .and_then(|()| fsx::create_dir(&path.join(".hllv")))
        .map(|()| Self { path: path.to_path_buf() })
    }
    pub fn open(path: &Path) -> io::Result<Self> {
        fsx::metadata(&path.join(".hllv"))
        .and_then(|m| m.check_dir())
        .map(|()| Self { path: path.to_path_buf() })
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}
