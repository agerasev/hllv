use std::{
    io, fs,
    path::{Path, PathBuf},
    collections::HashMap,
};
use super::{
    fsx, Id,
};


/// Files that is in working directory
pub struct Tree {
    files: HashMap<PathBuf, Id>,
}

pub struct History {

}

impl Tree {
    fn new(files: HashMap<PathBuf, Id>) -> Self {
        Self { files }
    }
    fn from_file(path: &Path) -> io::Result<Self> {
        unimplemented!()
    }
}
