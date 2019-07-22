mod metadata;

pub use metadata::{
    metadata,
    Metadata,
};

use std::{
    io, fs,
    path::Path,
};

pub fn create_dir(path: &Path) -> io::Result<()> {
    fs::create_dir(path).map_err(|e| {
        io::Error::new(e.kind(), format!("'{}': {}", path.display(), e))
    })
}
