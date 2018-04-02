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
    pub package_id_to_dep_name_to_id: HashMap<String, HashMap<String, String>>,
}  // TODO merge package paths and package_id_to_dep_name_to_id

impl WorkspaceCopy {
    pub fn create(workspace_info: WorkspaceInfo) -> Self {
        let mut workspace_copier = WorkspaceCopier::new(&workspace_info);
        for package_info in &workspace_info.packages {
            println!("Copying {}", package_info.id);
            workspace_copier.copy_package(package_info)
        }
        println!("Finished copying packages");
        workspace_copier.finish(workspace_info.package_id_to_dep_name_to_id)
    }
}

struct WorkspaceCopier {
    old_files:      HashMap<PathBuf, FileTime>,
    old_dirs:       HashSet<PathBuf>,
    workspace_root: PathBuf,
    deps_root:      PathBuf,
    tested_root:    PathBuf,
    modified_files: HashSet<PathBuf>,
    package_paths:  HashMap<String, PathBuf>,
}

impl WorkspaceCopier {
    pub fn new(workspace_info: &WorkspaceInfo) -> Self {
        let mocktopus_dir = workspace_info.target_root.join(MOCKTOPUS_DIR);
        let deps_root = mocktopus_dir.join(DEPS_DIR);
        fs::create_dir_all(&deps_root).expect("43");
        let tested_root = mocktopus_dir.join(TESTED_DIR);
        fs::create_dir_all(&tested_root).expect("44");
        let mut copier = WorkspaceCopier {
            old_files:      HashMap::new(),
            old_dirs:       HashSet::new(),
            workspace_root: workspace_info.workspace_root.clone(),
            deps_root:      deps_root.clone(),
            tested_root:    tested_root.clone(),
            modified_files: HashSet::new(),
            package_paths:  HashMap::new(),
        };
        copier.collect_dir_content(&deps_root);
        copier.collect_dir_content(&tested_root);
        copier
    }

    fn collect_dir_content(&mut self, dir: &PathBuf) {
        for dir_entry_res in fs::read_dir(dir).expect("14") {
            let dir_entry = dir_entry_res.expect("15");
            let path = dir_entry.path();
            let metadata = dir_entry.metadata().expect("16");
            if metadata.is_dir() {
                self.collect_dir_content(&path);
                self.old_dirs.insert(path);
            } else if metadata.is_file() {
                let last_modification = FileTime::from_last_modification_time(&metadata);
                self.old_files.insert(path, last_modification);
            }
        }
    }

    pub fn copy_package(&mut self, package_info: &PackageInfo) {
        let src_root;
        let dest_root;
        match package_info.dep_root {
            Some(ref dep_root) => {
                src_root = dep_root.clone();
                dest_root = self.deps_root.join(encode_id(&*package_info.id));
            },
            None => {
                src_root = self.workspace_root.clone();
                dest_root = self.tested_root.clone();
            }
        }
        for file in &package_info.files {
            self.copy_file_and_parents(&src_root, file, &dest_root)
        }
        self.package_paths.insert(package_info.id.clone(), dest_root);
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

    pub fn finish(self, package_id_to_dep_name_to_id: HashMap<String, HashMap<String, String>>) -> WorkspaceCopy {
        self.old_dirs.iter()
            .filter(|dir| dir.exists())
            .for_each(|dir| fs::remove_dir_all(dir).expect("18"));
        self.old_files.keys()
            .filter(|file| file.exists())
            .for_each(|file| fs::remove_file(file).expect("19"));
        WorkspaceCopy {
            package_paths: self.package_paths,
            modified_files: self.modified_files,
            package_id_to_dep_name_to_id,
        }
    }
}

//fn create_dir_overwriting_files(root: &PathBuf, created_dir: &str) {
//    if let Ok(metadata) = dir.metadata() {
//        if metadata.is_dir() {
//            return
//        } else if metadata.is_file() {
//            fs::remove_file(&dir).expect("41")
//        }
//    }
//    fs::create_dir_all(&dir).expect("42")
//}
