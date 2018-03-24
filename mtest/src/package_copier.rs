use cargo_metadata::{Metadata, Package};
use filetime::FileTime;
use fs_extra;
use fs_extra::dir::CopyOptions;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use super::encode_id;

const MOCKTOPUS_DIR: &str = ".mocktopus";

pub struct PackageCopier {
    old_files:  HashMap<PathBuf, FileTime>,
    old_dirs:   HashSet<PathBuf>,
    root:       PathBuf,
}

impl PackageCopier {
    pub fn new(metadata: &Metadata) -> Self {
        let mut copier = PackageCopier {
            old_files:  HashMap::new(),
            old_dirs:   HashSet::new(),
            root:       PathBuf::new(),
        };
        let mut root = PathBuf::from(&metadata.workspace_root);
        root.push(MOCKTOPUS_DIR);
        if !root.is_dir() {
            fs::create_dir(&root).expect("13")
        }
        copier.fill_from_dir(&root);
        copier.root = root;
        copier
    }

    fn fill_from_dir<P: AsRef<Path>>(&mut self, dir: P) {
        for dir_entry_res in fs::read_dir(dir).expect("14") {
            let dir_entry = dir_entry_res.expect("15");
            let path = dir_entry.path();
            let metadata = dir_entry.metadata().expect("16");
            if metadata.is_dir() {
                self.fill_from_dir(&path);
                self.old_dirs.insert(path);
            } else if metadata.is_file() {
                self.old_files.insert(path, FileTime::from_last_modification_time(&metadata));
            }
        }
    }

    pub fn copy_package(&mut self, package: &Package) -> PathBuf {
        let copy_opts = CopyOptions {
            copy_inside: true,
            ..CopyOptions::new()
        };
        let sources = fs::read_dir(PathBuf::from(&package.manifest_path).parent().expect("14"))
            .expect("15")
            .map(|res| res.expect("16"))
            .filter(|entry| entry.file_name() != *MOCKTOPUS_DIR && entry.file_name() != *"target")
            .map(|entry| entry.path())
            .collect();
        let target = self.root.join(encode_id(&package.id));
        fs_extra::dir::create(&target, true)
            .expect("17");
        fs_extra::copy_items(&sources, &target, &copy_opts)
            .expect("18");
        target
    }

    pub fn remove_old(self) {
        self.old_dirs.iter()
            .filter(|dir| dir.exists())
            .for_each(|dir| fs::remove_dir_all(dir).expect("18"));
        self.old_files.keys()
            .filter(|file| file.exists())
            .for_each(|file| fs::remove_file(file).expect("19"));
    }
}
