extern crate cargo_metadata;

fn main() {
    let metadata = cargo_metadata::metadata_deps(None, true).unwrap();
    let root = &metadata.workspace_root;
    for package in &metadata.packages {
        println!("\nID {}\nMANIFEST {}", package.id, package.manifest_path);
    }

    println!("ROOT {}", root);

}
