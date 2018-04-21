use std::collections::HashMap;
use std::path::PathBuf;

pub struct PackageCopy {
    pub root: PathBuf,
    pub dep_names_to_ids: HashMap<String, String>,
}

impl PackageCopy {
    pub fn new(root: PathBuf, dep_names_to_ids: HashMap<String, String>) -> Self {
        PackageCopy { root, dep_names_to_ids, }
    }
}
