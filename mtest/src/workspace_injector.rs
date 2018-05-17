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
    let dep = Dependency::new("mocktopus").set_git("https://github.com/CodeSandwich/Mocktopus.git");
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
    println!("Injecting file {:?}", entry_point);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(entry_point)
        .expect(&format!("46 FILE {:?}", entry_point));
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("47");
    let injections = match get_injections(&content) {
        Ok(injections) => injections,
        Err(()) => return,
    };
    file.seek(SeekFrom::Start(0))
        .expect("48");
    file.set_len(0)
        .expect("50");
    write_source_with_injections(&content, &mut file, injections);
}

extern crate proc_macro2;
use self::proc_macro2::{LineColumn, TokenStream};
use std::iter::{once, repeat};
use std::fs::File as FsFile;
use syn::{Item, parse2, Visibility};
extern crate proc_macro;
use self::proc_macro::{Diagnostic, Level};

fn get_injections(file_content: &str) -> Result<Vec<(LineColumn, &'static str)>, ()> {
    let mut file = match parse_file_content(file_content) {
        Ok(file) => file,
        Err(error) => {
            Diagnostic::new(Level::Warning, format!("Failed to mock crate: {}", error))
                .emit();
            return Err(())
        },
    };
    let mod_injection_points = file.items.iter()
        .filter_map(get_mod_injection_point)
        .zip(repeat("#[extern::mocktopus::macros::mockable]"));
    let feature_injection_point = (
        LineColumn { line: 1, column: 0 },
        "#![feature(proc_macro, proc_macro_mod, extern_in_paths, proc_macro_path_invoc)]"
    );
    Ok(once(feature_injection_point)
        .chain(mod_injection_points)
        .collect())
}

fn parse_file_content(file_content: &str) -> Result<File, String> {
    let token_stream = file_content.parse()
        .map_err(|e| format!("{:?}", e))?;
    parse2(token_stream)
        .map_err(|e| e.to_string())
}

fn get_mod_injection_point(item: &Item) -> Option<LineColumn> {
    match *item {
        Item::Mod(ref item_mod) => {
            match item_mod.vis {
                Visibility::Public(ref vis_public)          => vis_public.pub_token.0,
                Visibility::Crate(ref vis_crate)            => vis_crate.crate_token.0,
                Visibility::Restricted(ref vis_restricted)  => vis_restricted.pub_token.0,
                Visibility::Inherited                       => item_mod.mod_token.0,
            }.start().into()
        }
        _ => None,
    }
}

fn write_source_with_injections(source: &str, sink: &mut FsFile, injections: Vec<(LineColumn, &str)>) {
    let mut injections_iter = injections.into_iter().peekable();
    for (mut text, line) in source.lines().zip(1..) {
        let mut col = 0;
        loop {
            match injections_iter.peek() {
                Some((line_col, injection)) if line_col.line == line => {
                    let idx = match line_col.column - col {
                        0 => 0,
                        offset => text.char_indices()
                            .skip(offset)
                            .next()
                            .expect("60")
                            .0,
                    };
                    let (before, after) = text.split_at(idx);
                    write!(sink, "{}{} ", before, injection).expect("61");
                    text = after;
                    col = line_col.column;
                },
                _ => {
                    writeln!(sink, "{}", text).expect("62");
                    break;
                }
            }
            injections_iter.next();
        }
    }
}
