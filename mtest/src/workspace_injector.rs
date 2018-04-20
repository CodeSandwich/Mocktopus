use cargo_edit::{Dependency, Manifest};
//use cargo_metadata::Metadata;
use std::collections::HashMap;
use std::iter;
use std::path::PathBuf;
use workspace_copy::WorkspaceCopy;

pub fn inject_workspace(workspace: &WorkspaceCopy) {
    for (package_id, package_path) in &workspace.package_paths {
        println!("Mocking {} in {:?}", package_id, package_path);
        inject_manifest(workspace, package_id, package_path);
    }
    println!("Finished mocking packages");
}

fn inject_manifest(workspace: &WorkspaceCopy, package_id: &String, package_path: &PathBuf) {
    let package_path_opt = Some(package_path.clone());
    let mut package_manifest = Manifest::open(&package_path_opt)
        .expect("3");
    let deps = workspace.package_id_to_dep_name_to_id.get(package_id)
        .expect("43");

    replace_deps_with_mocks(&mut package_manifest, deps, &workspace.package_paths);
    add_mocktopus_dep(&mut package_manifest);
    package_manifest.write_to_file(&mut Manifest::find_file(&package_path_opt).expect("8")).expect("9");
}

fn replace_deps_with_mocks(package_manifest: &mut Manifest, deps: &HashMap<String, String>,
                           package_paths: &HashMap<String, PathBuf>) {
    let sections = package_manifest.get_sections();
    let dep_replacements = sections.iter()
        .flat_map(|&(ref dep_path, ref item)|
            item.as_table_like()
                .expect("4")
                .iter()
                .zip(iter::repeat(dep_path))
                .filter_map(|((dep_name, _), dep_path)|
                    create_dependency(deps, dep_name, package_paths)
                        .map(|dep| (dep_path, dep))))
        .collect::<Vec<_>>();
    for (dep_path, dep) in dep_replacements {
        package_manifest.update_table_entry(&*dep_path, &dep)
            .expect("7")
    }
}

fn add_mocktopus_dep(package_manifest: &mut Manifest) {
    let dep_path = ["dependencies".to_string()];
    let dep = Dependency::new("code-sandwich-crates-io-release-test").set_version("*");
    package_manifest.insert_into_table(&dep_path, &dep)
        .expect("10")
}

//fn get_dependenies_ids<'a>(metadata: &'a Metadata, id: &str) -> &'a [String] {
//    metadata.resolve.as_ref()
//        .expect("1")
//        .nodes.iter()
//        .find(|n| n.id == id)
//        .expect("2")
//        .dependencies
//        .as_slice()
//}

fn create_dependency(deps: &HashMap<String, String>, name: &str, package_paths: &HashMap<String, PathBuf>)
        -> Option<Dependency> {
    deps.get(name)
        .map(|id| package_paths.get(id).expect("44").to_str().expect("45"))
        .map(|path| Dependency::new(name).set_path(path))
//    Some(Dependency::new(name)
//        .set_path(path))
}

//fn id_matches_name(id: &String, name: &str) -> bool {
//    id.starts_with(name) && id.split_at(name.len()).1.starts_with(" ")
//}
//
//fn id_to_dep_path(id: &str) -> String {
//    PathBuf::from("..")
//        .join(encode_id(id))
//        .into_os_string()
//        .into_string()
//        .unwrap()
//}
