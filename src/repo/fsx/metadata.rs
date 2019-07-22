use std::{
    io, fs,
    path::Path,
    ops::Deref,
};


pub struct Metadata<'a> {
    meta: fs::Metadata,
    path: &'a Path,
}

impl<'a> Metadata<'a> {
    pub fn new(meta: fs::Metadata, path: &'a Path) -> Self {
        Self { meta, path }
    }
    pub fn check_dir(&self) -> io::Result<()> {
        if self.is_dir() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("'{}' is not a directory", self.path.display()),
            ))
        }
    }
    pub fn check_file(&self) -> io::Result<()> {
        if self.is_dir() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("'{}' is not a file", self.path.display()),
            ))
        }
    }
}

impl<'a> Deref for Metadata<'a> {
    type Target = fs::Metadata;
    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

pub fn metadata(path: &Path) -> io::Result<Metadata> {
    fs::metadata(path).map(|meta| Metadata::new(meta, path))
}
