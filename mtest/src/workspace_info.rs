use cargo_metadata;
use package_info::PackageInfo;
use std::path::{Path, PathBuf};

pub struct WorkspaceInfo {
    pub packages: Vec<PackageInfo>,
    pub workspace_root: PathBuf,
    pub target_root: PathBuf,
}

impl WorkspaceInfo {
    pub fn new(manifest_path: Option<&Path>) -> Self {
        let metadata = cargo_metadata::metadata_deps(manifest_path, true).expect("i0");
        let member_ids = metadata.workspace_members.iter()
            .map(|member| format!("{} {} ({})", member.name, member.version, member.url))
            .collect::<Vec<_>>();
        let mut packages = metadata.packages.iter()
            .map(|package| {
                let id = &*package.id;
                let manifest_path = &*package.manifest_path;
                let is_dep = !member_ids.contains(&package.id);
                PackageInfo::new(id, manifest_path, is_dep)
            })
            .collect::<Vec<_>>();
        WorkspaceInfo {
            packages,
            workspace_root: PathBuf::from(&metadata.workspace_root),
            target_root: PathBuf::from(&metadata.target_directory),
        }
    }
}

