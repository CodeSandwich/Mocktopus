use std::collections::HashMap;
use std::path::PathBuf;

pub struct PackageCopy {
    pub root: PathBuf,
    pub dep_names_to_ids: HashMap<String, String>,
    pub entry_points: Vec<PathBuf>,
}

impl PackageCopy {
    pub fn new(root: PathBuf, dep_names_to_ids: HashMap<String, String>, entry_points: Vec<PathBuf>) -> Self {
        PackageCopy {
            root,
            dep_names_to_ids,
            entry_points,
        }
    }
}
