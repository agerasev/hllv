use std::{
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use super::db::Db;

pub struct Repo {
    db: Db,
}
impl Repo {
    pub fn new(path: &Path) -> io::Result<Self> {
        Db::new(path)
        .map(|db| Self { db })
    }
    pub fn open(path: &Path) -> io::Result<Self> {
        Db::open(path)
        .map(|db| Self { db })
    }
    pub fn path(&self) -> &Path {
        self.db.path()
    }

    pub fn push<S: Read + Write>(&self, stream: S) -> io::Result<()> {
        unimplemented!()
    }
    pub fn pull<S: Read + Write>(&mut self, stream: S) -> io::Result<()> {
        unimplemented!()
    }
}
