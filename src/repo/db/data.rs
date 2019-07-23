use std::{
    io, fs,
    path::{Path, PathBuf},
    collections::{HashSet, HashMap},
};

use super::{
    fsx, Id, Tree,
};

/// Orphaned files that removed in some previous commit
pub struct Pool {
    files: HashSet<Id>,
}

pub struct Data {
    tree: Tree,
    pool: Pool,
}
