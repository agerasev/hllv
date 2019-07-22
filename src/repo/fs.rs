use std::{
    io, fs,
    path::{Path, PathBuf},
    net::ToSocketAddrs,
};
//use fs2::{FileExt};
use super::{Repo, ListenEvent};

fn check_dir(path: &Path) -> io::Result<()> {
    fs::metadata(path)
    .and_then(|md| {
        if md.is_dir() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("'{}' is not a directory", path.display()),
            ))
        }
    })
}

pub struct FsRepo {
    path: PathBuf,
}

impl FsRepo {
    
}

impl Repo for FsRepo {
    fn new(name: &Path) -> io::Result<Self> {
        check_dir(name)
        .and_then(|()| Self::open(name))
    }
    fn open(name: &Path) -> io::Result<Self> {
        check_dir(name)
        .and_then(|()| {
            Ok(Self { path: name.to_path_buf() })
        })
    }
    fn name(&self) -> &Path {
        &self.path
    }

    fn listen<A: ToSocketAddrs, F: Fn(ListenEvent)>(&mut self, addr: A, f: F) -> io::Result<()> {
        unimplemented!();
    }
    fn push<A: ToSocketAddrs>(&mut self, addr: A, name: &str) -> io::Result<()> {
        unimplemented!();
    }
    fn pull<A: ToSocketAddrs>(&mut self, addr: A, name: &str) -> io::Result<()> {
        unimplemented!();
    }
}
