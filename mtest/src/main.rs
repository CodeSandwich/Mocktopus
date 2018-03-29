#![feature(conservative_impl_trait)]

extern crate cargo;
extern crate cargo_edit;
extern crate cargo_metadata;
extern crate data_encoding;
extern crate filetime;
extern crate fs_extra;

mod package_copier;

use cargo_edit::{Dependency, Manifest};
use cargo_metadata::Metadata;
use data_encoding::BASE64URL_NOPAD;
use package_copier::PackageCopier;
use std::fmt::Write;
use std::iter;
use std::path::PathBuf;

fn main() {
    let metadata = cargo_metadata::metadata_deps(None, true).expect("0");
    let mut package_copier = PackageCopier::new(&metadata);
    for package in &metadata.packages {
        println!("     Mocking {}", package.id);
        let package_path = package_copier.copy_package(package);
        inject_manifest(package_path, &package.id, &metadata);
    }
    println!("Finished mocking");
}

fn inject_manifest(package_path: PathBuf, package_id: &str, metadata: &Metadata) {
    let package_path_opt = Some(package_path);
    let mut package_manifest = Manifest::open(&package_path_opt)
        .expect("3");
    replace_deps_with_mocks(&mut package_manifest, package_id, metadata);
    add_mocktopus_dep(&mut package_manifest);
    package_manifest.write_to_file(&mut Manifest::find_file(&package_path_opt).expect("8")).expect("9");
}

fn replace_deps_with_mocks(package_manifest: &mut Manifest, package_id: &str, metadata: &Metadata) {
    let dep_ids = get_dependenies_ids(&metadata, package_id);
    let sections = package_manifest.get_sections();
    let dep_replacements = sections.iter()
        .flat_map(|&(ref dep_path, ref item)|
            item.as_table_like()
                .expect("4")
                .iter()
                .zip(iter::repeat(dep_path))
                .filter_map(|((dep_name, _), dep_path)|
                    create_dependency(&dep_ids, dep_name)
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

fn get_dependenies_ids<'a>(metadata: &'a Metadata, id: &str) -> &'a [String] {
    metadata.resolve.as_ref()
        .expect("1")
        .nodes.iter()
        .find(|n| n.id == id)
        .expect("2")
        .dependencies
        .as_slice()
}

fn create_dependency(dep_ids: &[String], name: &str) -> Option<Dependency> {
    dep_ids.iter()
        .find(|id| id_matches_name(id, name))
        .map(|id| Dependency::new(name).set_path(&id_to_dep_path(id)))
}

fn id_matches_name(id: &String, name: &str) -> bool {
    id.starts_with(name) && id.split_at(name.len()).1.starts_with(" ")
}

fn id_to_dep_path(id: &str) -> String {
    PathBuf::from("..")
        .join(encode_id(id))
        .into_os_string()
        .into_string()
        .unwrap()
}

fn encode_id(id_str: &str) -> String {
    let id = id_str.as_bytes();
    let mut result = String::new();
    let mut escaped_seq_start = None;
    for (curr_idx, curr_byte) in id.iter().cloned().enumerate() {
        if byte_is_valid(curr_byte) {
            escape_seq(id, &mut escaped_seq_start, curr_idx, &mut result);
            result.push(curr_byte as char); // All valid chars are ASCII (1-byte UTF-8)
        } else {
            escaped_seq_start.get_or_insert(curr_idx);
        }
    }
    escape_seq(id, &mut escaped_seq_start, id.len(), &mut result);
    result
}

fn byte_is_valid(byte: u8) -> bool {
    BASE64URL_NOPAD.specification().symbols.as_bytes().contains(&byte)
        && byte != b'.'
}

fn escape_seq(id: &[u8], invalid_char_idx: &mut Option<usize>, curr_idx: usize, result: &mut String) {
    if let Some(idx) = invalid_char_idx.take() {
        write!(result, ".{}.", BASE64URL_NOPAD.encode(&id[idx..curr_idx])).unwrap()
    }
}
