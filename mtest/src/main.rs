extern crate cargo_metadata;
extern crate fs_extra;

use cargo_metadata::Metadata;
use std::fs;
use std::path::PathBuf;
use fs_extra::dir::CopyOptions;

const MOCKTOPUS_DIR: &str = ".mocktopus";

fn main() {
    let metadata = cargo_metadata::metadata_deps(None, true).unwrap();
    copy_packages(&metadata);
//    let root = &metadata.workspace_root;
//    for package in &metadata.packages {
//        println!("\nID {}\nMANIFEST {}", package.id, package.manifest_path);
//    }
//    println!("ROOT {}", root);


}

fn copy_packages(metadata: &Metadata) {
    let mut workspace_target = PathBuf::from(&metadata.workspace_root);
    workspace_target.push(MOCKTOPUS_DIR);
    fs_extra::dir::create(&workspace_target, true)
        .unwrap();
    let copy_opts = CopyOptions {
        copy_inside: true,
        ..CopyOptions::new()
    };
    for package in &metadata.packages {
        let sources = fs::read_dir(PathBuf::from(&package.manifest_path).parent().unwrap())
            .unwrap()
            .map(|res| res.unwrap())
            .filter(|entry| entry.file_name() != *MOCKTOPUS_DIR)
            .map(|entry| entry.path())
            .collect();
        let target = workspace_target.join(&package.name);
        fs_extra::dir::create(&target, true)
            .unwrap();
        fs_extra::copy_items(&sources, target, &copy_opts)
            .unwrap();
    }
}
