use cargo::core::{Package as CargoPackage, SourceId};
use cargo::core::manifest::EitherManifest;
use cargo::sources::PathSource;
use cargo::util::Config;
use cargo::util::toml;
use cargo_metadata::{Metadata, Package};
use filetime::FileTime;
use fs_extra;
use fs_extra::dir::CopyOptions;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use super::encode_id;

const TARGET_DIR: &str = "target";
const MOCKTOPUS_DIR: &str = "mocktopus";

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
        root.push(TARGET_DIR);
        root.push(MOCKTOPUS_DIR);
        match root.is_dir() {
            true => copier.fill_from_dir(&root),
            false => fs::create_dir_all(&root).expect("13"),
        }
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
        let dest = self.root.join(encode_id(&package.id));
        let src_manifest = PathBuf::from(&package.manifest_path);
        let src_root = src_manifest.parent().expect("14");
        for src in &get_package_files(package) {
            self.copy_file_and_parents(&dest, src_root, src);
        }
//        self.copy_dir(&dest, src_root);
        dest
    }

    fn copy_file_and_parents(&mut self, dest_root: &PathBuf, src_root: &Path, src: &PathBuf) {
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
            self.copy_file(&dest, src, src_meta)
        }
    }

//    fn create_dir<P: AsRef<Path>>(&mut self, dest: &PathBuf, src: P) {
//        self.create_dir_empty(dest);
//        self.create_dir_content(dest, src);
//    }

//    fn copy_fs_item(&mut self, dest: PathBuf, src: PathBuf) {
//        let src_meta = src.metadata().expect("21");
//        if src_meta.is_dir() {
//            self.create_dir(&dest);
//        } else if src_meta.is_file() {
//            self.copy_file(dest, src, src_meta)
//        }
//    }

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

    fn copy_file(&mut self, dest: &PathBuf, src: &PathBuf, src_meta: fs::Metadata) {
        if let Some(old_time) = self.old_files.remove(dest) {
            FileTime::from_last_modification_time(&src_meta);
            if old_time >= FileTime::from_last_modification_time(&src_meta) {
                return
            }
        } else if self.old_dirs.remove(dest) {
            fs::remove_dir_all(dest).expect("26");
        }
        fs::copy(src, dest).expect("27");
    }

//    fn copy_dir_content<P: AsRef<Path>>(&mut self, dest: &PathBuf, src_ref: P) {
//        let src = src_ref.as_ref();
//        fs::read_dir(src)
//            .expect("26")
//            .map(|entry| entry.expect("27").file_name())
//            .for_each(|file_name| self.copy_fs_item(dest.join(&file_name), src.join(&file_name)))
//    }

    pub fn remove_old(self) {
        self.old_dirs.iter()
            .filter(|dir| dir.exists())
            .for_each(|dir| fs::remove_dir_all(dir).expect("18"));
        self.old_files.keys()
            .filter(|file| file.exists())
            .for_each(|file| fs::remove_file(file).expect("19"));
    }
}


fn get_package_files(package: &Package) -> Vec<PathBuf> {
    let src_manifest = PathBuf::from(&package.manifest_path);
    let mut src = src_manifest.parent().expect("34");
    let source_id = SourceId::for_path(src).expect("32");
    let config = Config::default().expect("30");
    let (either_manifest, _) = toml::read_manifest(&src_manifest, &source_id, &config).expect("33");
    let manifest = match either_manifest {
        EitherManifest::Real(manifest) => manifest,
        EitherManifest::Virtual(_) => panic!("35"),
    };
    PathSource::new(&src_manifest, &source_id, &config)
        .list_files(&CargoPackage::new(manifest, &src_manifest))
        .expect("31")
}
