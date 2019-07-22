mod fs;
pub use fs::FsRepo;

use std::{
    io::{self, Read, Write},
    path::Path,
    net::{ToSocketAddrs},
};

pub trait Repo {
    fn new(path: &Path) -> io::Result<Self>;
    fn open(path: &Path) -> io::Result<Self>;
    fn path(&self) -> &Path;

    fn push<W: Write>(&mut self, stream: W) -> io::Result<()>;
    fn pull<R: Read>(&mut self, stream: R) -> io::Result<()>;
}

pub trait Manager {
    fn listen<A: ToSocketAddrs>(&mut self, addr: A, path: &Path) -> io::Result<()>;
}