extern crate cargo;
extern crate cargo_edit;
extern crate cargo_metadata;
extern crate data_encoding;
extern crate filetime;
extern crate fs_extra;

mod filename_encoder;
mod package_info;
mod package_kind;
mod workspace_copy;
mod workspace_info;
mod workspace_injector;

use workspace_copy::WorkspaceCopy;
use workspace_info::WorkspaceInfo;
use workspace_injector::inject_workspace;

fn main() {
    let workspace_info = WorkspaceInfo::new(None);
    let workspace_copy = WorkspaceCopy::create(workspace_info);
    inject_workspace(&workspace_copy);
}
