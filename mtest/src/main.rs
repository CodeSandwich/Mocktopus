#![feature(conservative_impl_trait)]

extern crate cargo_edit;
extern crate cargo_metadata;
extern crate data_encoding;
extern crate fs_extra;

use cargo_edit::{Dependency, Manifest};
use cargo_metadata::{Metadata, Package};
use data_encoding::BASE64URL_NOPAD;
use fs_extra::dir::CopyOptions;
use std::fmt::Write;
use std::fs;
use std::iter;
use std::path::PathBuf;

const MOCKTOPUS_DIR: &str = ".mocktopus";

fn main() {
    let metadata = cargo_metadata::metadata_deps(None, true).expect("0");
    let workspace_target = create_workspace_target(&metadata);
    for package in &metadata.packages {
        println!("     Mocking {}", package.id);
        let package_path = copy_package(package, &workspace_target);
        inject_manifest(package_path, &package.id, &metadata);
//        let dep_ids = get_dependenies_ids(&metadata, &package.id);
//        let mut manifest = Manifest::open(&target)
//            .expect("3");
//        let sections = manifest.get_sections();
//        sections.iter()
//            .flat_map(|&(ref section, ref item)|
//                item.as_table_like()
//                    .expect("4")
//                    .iter()
//                    .zip(iter::repeat(section))
//                    .filter_map(|((name, _), section)| create_dependency(&dep_ids, name).map(|dep| (section, dep))))
//            .collect::<Vec<_>>()
//            .into_iter()
//            .for_each(|(section, dependency)| manifest.update_table_entry(&*section, &dependency).expect("7"));
//        manifest.write_to_file(&mut Manifest::find_file(&target).expect("8")).expect("9");
    }
    println!("Finished mocking");
    println!("cargo test --manifest-path ")
}

fn inject_manifest(package_path: PathBuf, package_id: &str, metadata: &Metadata) {
    let dep_ids = get_dependenies_ids(&metadata, package_id);
    let package_path_opt = Some(package_path);
    let mut manifest = Manifest::open(&package_path_opt)
        .expect("3");
    let sections = manifest.get_sections();
    sections.iter()
        .flat_map(|&(ref section, ref item)|
            item.as_table_like()
                .expect("4")
                .iter()
                .zip(iter::repeat(section))
                .filter_map(|((name, _), section)| create_dependency(&dep_ids, name).map(|dep| (section, dep))))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(section, dependency)| manifest.update_table_entry(&*section, &dependency).expect("7"));
    manifest.write_to_file(&mut Manifest::find_file(&package_path_opt).expect("8")).expect("9");
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

fn create_workspace_target(metadata: &Metadata) -> PathBuf {
    let mut workspace_target = PathBuf::from(&metadata.workspace_root);
    workspace_target.push(MOCKTOPUS_DIR);
    fs_extra::dir::create(&workspace_target, true)
        .expect("13");
    workspace_target
}

fn copy_package(package: &Package, workspace_target: &PathBuf) -> PathBuf {
    let copy_opts = CopyOptions {
        copy_inside: true,
        ..CopyOptions::new()
    };
    let sources = fs::read_dir(PathBuf::from(&package.manifest_path).parent().expect("14"))
        .expect("15")
        .map(|res| res.expect("16"))
        .filter(|entry| entry.file_name() != *MOCKTOPUS_DIR && entry.file_name() != *"target")
        .map(|entry| entry.path())
        .collect();
    let target = workspace_target.join(encode_id(&package.id));
    fs_extra::dir::create(&target, true)
        .expect("17");
    fs_extra::copy_items(&sources, &target, &copy_opts)
        .expect("18");
    target
}

const VALID_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

fn encode_id(id_str: &str) -> String {
    fn flush_encoded(id: &[u8], invalid_char_idx: &mut Option<usize>, curr_idx: usize, result: &mut String) {
        if let Some(idx) = invalid_char_idx.take() {
            write!(result, ".{}.", BASE64URL_NOPAD.encode(&id[idx..curr_idx])).unwrap()
        }
    }

    let id = id_str.as_bytes();
    let mut result = String::new();
    let mut invalid_char_idx = None;
    for (idx, curr_char) in id.iter().enumerate() {
        if VALID_CHARS.contains(curr_char) {
            flush_encoded(id, &mut invalid_char_idx, idx, &mut result);
            result.push(*curr_char as char);
        } else {
            invalid_char_idx.get_or_insert(idx);
        }
    }
    flush_encoded(id, &mut invalid_char_idx, id.len(), &mut result);
    result
}
