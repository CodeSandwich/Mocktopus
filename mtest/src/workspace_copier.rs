use super::encode_id;
use filetime::FileTime;
use package_info::PackageInfo;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use workspace_info::WorkspaceInfo;

const MOCKTOPUS_DIR:    &str = "mocktopus";
const DEPS_DIR:         &str = "deps";
const TESTED_DIR:       &str = "tested";

pub struct WorkspaceCopy {
    pub package_paths:  HashMap<String, PathBuf>,
    pub modified_files: HashSet<PathBuf>,
}

impl WorkspaceCopy {
    pub fn create(workspace_info: &WorkspaceInfo) -> Self {
        let mut workspace_copier = WorkspaceCopier::new(&workspace_info.target_root);
        for package_info in &workspace_info.packages {
            println!("Copying {}", package_info.id);
            workspace_copier.copy_package(package_info, &workspace_info.workspace_root)
        }
        workspace_copier.finish()
    }
}

struct WorkspaceCopier {
    old_files:      HashMap<PathBuf, FileTime>,
    old_dirs:       HashSet<PathBuf>,
    deps_root:      PathBuf,
    tested_root:    PathBuf,
    modified_files: HashSet<PathBuf>,
    package_paths:  HashMap<String, PathBuf>,
}

impl WorkspaceCopier {
    pub fn new(workspace_target_root: &PathBuf) -> Self {
        let mocktopus_dir   = workspace_target_root.join(MOCKTOPUS_DIR);
        let mut old_files   = HashMap::new();
        let mut old_dirs    = HashSet::new();
        let deps_root       = mocktopus_dir.join(DEPS_DIR);
        let tested_root     = mocktopus_dir.join(TESTED_DIR);
        collect_dir_content(&mut old_dirs, &mut old_files, &deps_root);
        collect_dir_content(&mut old_dirs, &mut old_files, &tested_root);
        WorkspaceCopier {
            old_files,
            old_dirs,
            deps_root,
            tested_root,
            modified_files: HashSet::new(),
            package_paths:  HashMap::new(),
        }
    }
    pub fn copy_package(&mut self, package_info: &PackageInfo, workspace_root: &PathBuf) {
        let src_root;
        let dest_root;
        if package_info.is_dep {
            src_root = &package_info.root;
            dest_root = self.deps_root.join(encode_id(&*package_info.id));
        } else {
            src_root = workspace_root;
            dest_root = self.tested_root.clone();
        }
        for file in &package_info.files {
            self.copy_file_and_parents(src_root, file, &dest_root)
        }
    }

    fn copy_file_and_parents(&mut self, src_root: &PathBuf, src: &PathBuf, dest_root: &PathBuf) {
        let src_rel = src.strip_prefix(src_root).expect("40");
        let mut dest = dest_root.clone();
        for src_rel_part in src_rel {
            self.create_dir(&dest);
            dest.push(src_rel_part);
        }
        let src_meta = src.metadata().expect("21");
        if src_meta.is_dir() {
            self.create_dir(&dest);
        } else if src_meta.is_file() {
            self.copy_file(src, src_meta, &dest)
        }
    }

    fn create_dir(&mut self, dest: &PathBuf) {
        if self.old_dirs.remove(dest) {
            return
        } else if self.old_files.remove(dest).is_some() {
            fs::remove_file(dest).expect("23")
        } else if dest.exists() {
            return
        }
        fs::create_dir(dest).expect("25")
    }

    fn copy_file(&mut self, src: &PathBuf, src_meta: fs::Metadata, dest: &PathBuf) {
        if let Some(old_time) = self.old_files.remove(dest) {
            FileTime::from_last_modification_time(&src_meta);
            if old_time >= FileTime::from_last_modification_time(&src_meta) {
                return
            }
        } else if self.old_dirs.remove(dest) {
            fs::remove_dir_all(dest).expect("26");
        }
        fs::copy(src, dest).expect("27");
        self.modified_files.insert(dest.clone());
    }

    pub fn finish(self) -> WorkspaceCopy {
        self.old_dirs.iter()
            .filter(|dir| dir.exists())
            .for_each(|dir| fs::remove_dir_all(dir).expect("18"));
        self.old_files.keys()
            .filter(|file| file.exists())
            .for_each(|file| fs::remove_file(file).expect("19"));
        WorkspaceCopy {
            package_paths: self.package_paths,
            modified_files: self.modified_files,
        }
    }
}

fn collect_dir_content(dirs: &mut HashSet<PathBuf>, files: &mut HashMap<PathBuf, FileTime>, dir: &PathBuf) {
    println!("COLLECTING FROM DIR {:?}", dir);
    for dir_entry_res in fs::read_dir(dir).expect("14") {
        let dir_entry = dir_entry_res.expect("15");
        let path = dir_entry.path();
        let metadata = dir_entry.metadata().expect("16");
        if metadata.is_dir() {
            collect_dir_content(dirs, files, &path);
            dirs.insert(path);
        } else if metadata.is_file() {
            let last_modification = FileTime::from_last_modification_time(&metadata);
            files.insert(path, last_modification);
        }
    }
}
