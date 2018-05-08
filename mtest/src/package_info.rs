use cargo::core::{Package as CargoPackage, SourceId};
use cargo::core::manifest::EitherManifest;
use cargo::sources::PathSource;
use cargo::util::Config;
use cargo::util::toml;
use cargo_metadata::{Package, Target};
use package_kind::PackageKind;
use std::collections::HashMap;
use std::path::PathBuf;

const TARGET_KIND_LIB: &str = "lib";

pub struct PackageInfo {
    pub id: String,
    pub kind: PackageKind,
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
    pub dep_names_to_ids: HashMap<String, String>,
    pub entry_points: Vec<PathBuf>,
}

impl PackageInfo {
    pub fn new(package: &Package, kind: PackageKind, dep_names_to_ids: HashMap<String, String>) -> Self {
        let mut root = PathBuf::from(&package.manifest_path);
        let files = get_package_files(&root);
        if !root.pop() {
            panic!("43");
        }
        let entry_points = package.targets.iter()
            .filter(|target| is_entry_point_needed(target, kind))
            .map(|target| PathBuf::from(&target.src_path))
            .collect();
        PackageInfo {
            id: package.id.clone(),
            kind,
            root,
            files,
            dep_names_to_ids,
            entry_points,
        }
    }
}

fn is_entry_point_needed(target: &Target, kind: PackageKind) -> bool {
    match kind {
        PackageKind::Tested => true,
        PackageKind::Dependency =>
            target.kind.iter()
                .any(|kind| kind == TARGET_KIND_LIB)
    }
}

fn get_package_files(src_manifest: &PathBuf) -> Vec<PathBuf> {
    let src = src_manifest.parent().expect("34");
    let source_id = SourceId::for_path(src).expect("32");
    let config = Config::default().expect("30");
    let (either_manifest, _) = toml::read_manifest(src_manifest, &source_id, &config).expect("33");
    let manifest = match either_manifest {
        EitherManifest::Real(manifest) => manifest,
        EitherManifest::Virtual(_) => return vec![src_manifest.clone()],
    };
    PathSource::new(src_manifest, &source_id, &config)
        .list_files(&CargoPackage::new(manifest, src_manifest))
        .expect("31")
}
