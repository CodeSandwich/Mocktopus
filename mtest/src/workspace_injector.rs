use cargo_edit::{Dependency, Manifest};
use package_copy::PackageCopy;
use std::collections::HashMap;
use std::iter;
use workspace_copy::WorkspaceCopy;

pub fn inject_workspace(workspace: &WorkspaceCopy) {
    for (package_id, package_copy) in &workspace.packages_by_id {
        println!("Mocking {} in {:?}", package_id, package_copy.root);
        inject_manifest(workspace, package_copy);
    }
    println!("Finished mocking packages");
}

const MANIFEST_FILENAME: &str = "Cargo.toml";

fn inject_manifest(workspace: &WorkspaceCopy, package_copy: &PackageCopy) {
    let manifest_path = package_copy.root.join(MANIFEST_FILENAME);
    if !workspace.modified_files.contains(&manifest_path) {
        return
    }
    let package_path_opt = Some(manifest_path);
    let mut package_manifest = Manifest::open(&package_path_opt)
        .expect("3");

    replace_deps_with_mocks(&mut package_manifest, workspace, package_copy);
    add_mocktopus_dep(&mut package_manifest);
    package_manifest.write_to_file(&mut Manifest::find_file(&package_path_opt).expect("8")).expect("9");
}

fn replace_deps_with_mocks(package_manifest: &mut Manifest, workspace: &WorkspaceCopy, package_copy: &PackageCopy) {
    let sections = package_manifest.get_sections();
    let dep_replacements = sections.iter()
        .flat_map(|&(ref dep_path, ref item)|
            item.as_table_like()
                .expect("4")
                .iter()
                .zip(iter::repeat(dep_path))
                .filter_map(|((dep_name, _), dep_path)|
                    create_dependency(dep_name, &package_copy.dep_names_to_ids, &workspace.packages_by_id)
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

fn create_dependency(name: &str, dep_names_to_ids: &HashMap<String, String>,
        packages_by_id: &HashMap<String, PackageCopy>) -> Option<Dependency> {
    dep_names_to_ids.get(name)
        .map(|id| packages_by_id.get(id).expect("44"))
        .map(|package| package.root.to_str().expect("45"))
        .map(|path| Dependency::new(name).set_path(path))
}
