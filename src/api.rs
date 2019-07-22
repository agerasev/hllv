use std::{
    io,
    path::Path,
};

pub trait Api {
    fn new(cwd: &Path, path: &Path) -> io::Result<()>;
    fn init(cwd: &Path) -> io::Result<()>;

    fn listen(cwd: &Path, addr: (&str, u16)) -> io::Result<()>;
    fn push(cwd: &Path, addr: (&str, u16), name: &Path) -> io::Result<()>;
    fn pull(cwd: &Path, addr: (&str, u16), name: &Path) -> io::Result<()>;
}

pub enum MainApi {}
impl Api for MainApi {
    fn new(cwd: &Path, path: &Path) -> io::Result<()> {
        unimplemented!()
    }
    fn init(cwd: &Path) -> io::Result<()> {
        unimplemented!()
    }

    fn listen(cwd: &Path, addr: (&str, u16)) -> io::Result<()> {
        unimplemented!()
    }
    fn push(cwd: &Path, addr: (&str, u16), name: &Path) -> io::Result<()> {
        unimplemented!()
    }
    fn pull(cwd: &Path, addr: (&str, u16), name: &Path) -> io::Result<()> {
        unimplemented!()
    }
}
