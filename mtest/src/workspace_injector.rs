use cargo_edit::{Dependency, Manifest};
use package_copy::PackageCopy;
use quote::ToTokens;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::iter;
use std::path::PathBuf;
use syn::{File, parse_file, parse_str};
use workspace_copy::WorkspaceCopy;

pub fn inject_workspace(workspace: &WorkspaceCopy) {
    for (package_id, package_copy) in &workspace.packages_by_id {
        println!("Mocking {} in {:?}", package_id, package_copy.root);
        inject_manifest(workspace, package_copy);
        inject_entry_points(workspace, package_copy);
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

fn inject_entry_points(workspace: &WorkspaceCopy, package_copy: &PackageCopy) {
    package_copy.entry_points.iter()
        .filter(|entry_point| workspace.modified_files.contains(*entry_point))
        .for_each(inject_entry_point);
}
fn inject_entry_point(entry_point: &PathBuf) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(entry_point)
        .expect(&format!("46 FILE {:?}", entry_point));
    let mut old_content = String::new();
    file.read_to_string(&mut old_content)
        .expect("47");
    let new_content = inject_file_content(old_content);
    file.seek(SeekFrom::Start(0))
        .expect("48");
    file.write(new_content.as_bytes())
        .expect("49");
}

fn inject_file_content(content: String) -> String {
    let mut file = parse_file(&content)
        .expect("50");
    let attr_file: File = parse_str("#![feature(proc_macro)]")
        .expect("51");
    file.attrs.extend(attr_file.attrs);
    file.into_tokens()
        .to_string()
}

fn inject_entry_point_old(entry_point: &PathBuf) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(entry_point)
        .expect(&format!("46 FILE {:?}", entry_point));
    let mut content = "#![feature(proc_macro)]\n".to_string();
    file.read_to_string(&mut content)
        .expect("47");
    file.seek(SeekFrom::Start(0))
        .expect("48");
    file.set_len(0)
        .expect("52");
    file.write(content.as_bytes())
        .expect("49");
}
