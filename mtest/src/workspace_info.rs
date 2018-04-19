use cargo_metadata::{self, Metadata};
use package_info::PackageInfo;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct WorkspaceInfo {
    pub packages: Vec<PackageInfo>,
    pub workspace_root: PathBuf,
    pub target_root: PathBuf,
    pub package_id_to_dep_name_to_id: HashMap<String, HashMap<String, String>>,
}

impl WorkspaceInfo {
    pub fn new(manifest_path: Option<&Path>) -> Self {
        let metadata = cargo_metadata::metadata_deps(manifest_path, true).expect("i0");
        let member_ids = metadata.workspace_members.iter()
            .map(|member| format!("{} {} ({})", member.name, member.version, member.url))
            .collect::<Vec<_>>();
        let packages = metadata.packages.iter()
            .map(|package| {
                let id = &*package.id;
                let manifest_path = &*package.manifest_path;
                let is_dep = !member_ids.contains(&package.id);
                PackageInfo::new(id, manifest_path, is_dep)
            })
            .collect();
        WorkspaceInfo {
            packages,
            workspace_root: PathBuf::from(&metadata.workspace_root),
            target_root: PathBuf::from(&metadata.target_directory),
            package_id_to_dep_name_to_id: get_resolved_dependencies(&metadata),
        }
    }
}

fn get_resolved_dependencies<'a>(metadata: &'a Metadata) -> HashMap<String, HashMap<String, String>> {
    metadata.resolve.as_ref()
        .expect("1")
        .nodes.iter()
        .map(|node| (node.id.clone(), get_node_dependencies(&node.dependencies)))
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
