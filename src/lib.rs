use std::{
    io,
    path::{Path},
    net::{ToSocketAddrs},
};

pub fn new(cwd: &Path, name: String) -> io::Result<()> {
    unimplemented!();
}

pub fn init(cwd: &Path) -> io::Result<()> {
    unimplemented!();
}

pub fn listen<A: ToSocketAddrs>(cwd: &Path, addr: A) -> io::Result<()> {
    unimplemented!();
}

pub fn push<A: ToSocketAddrs>(cwd: &Path, addr: A, name: String) -> io::Result<()> {
    unimplemented!()
}

pub fn pull<A: ToSocketAddrs>(cwd: &Path, addr: A, name: String) -> io::Result<()> {
    unimplemented!()
}
