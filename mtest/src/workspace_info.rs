use cargo_metadata::{self, Metadata};
use package_kind::PackageKind;
use package_info::PackageInfo;
use std::collections::HashMap;
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
        let mut package_id_to_dep_name_to_id = get_resolved_dependencies(&metadata);
        let packages = metadata.packages.iter()
            .map(|package| {
                let kind = match member_ids.contains(&package.id) {
                    true => PackageKind::Tested,
                    false => PackageKind::Dependency,
                };
                let dep_names_to_ids = package_id_to_dep_name_to_id.remove(&package.id)
                    .expect("3");
                PackageInfo::new(&*package.id, &*package.manifest_path, kind, dep_names_to_ids)
            })
            .collect();
        WorkspaceInfo {
            packages,
            workspace_root: PathBuf::from(&metadata.workspace_root),
            target_root: PathBuf::from(&metadata.target_directory),
        }
    }
}

fn get_resolved_dependencies<'a>(metadata: &'a Metadata) -> HashMap<&'a String, HashMap<String, String>> {
    metadata.resolve.as_ref()
        .expect("1")
        .nodes.iter()
        .map(|node| (&node.id, get_node_dependencies(&node.dependencies)))
        .collect()
}

fn get_node_dependencies(dependencies: &Vec<String>) -> HashMap<String, String> {
    dependencies.iter()
        .map(|id| (id_to_name(id), id.clone()))
        .collect()
}

fn id_to_name(id: &String) -> String {
    id.split(" ")
        .next()
        .expect("2")
        .to_string()
}
