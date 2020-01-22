use std::process::{Command, Output};
use std::path::Path;

use super::{Context, Module, RootModuleConfig};

use crate::configs::hg_commit::HgCommitConfig;

/// Creates a module with the Hg commit in the current directory
///
/// Will display the commit hash if the current directory is a hg repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    if ! is_hg_repo(context) {
        return None;
    }

    log::trace!("Mercurial repo detected");

    let mut module = context.new_module("hg_commit");
    let config = HgCommitConfig::try_load(module.config);
    if config.disabled {
        return None;
    };

    let hg_head_info = get_hg_current_commit(context);

    if let Some(hg_head) = hg_head_info {
        log::debug!("repo head: {:?}", hg_head);

        module
            .get_prefix()
            .set_value(config.prefix)
            .set_style(config.style);
        module
            .get_suffix()
            .set_value(config.suffix)
            .set_style(config.style);
        module.set_style(config.style);

        let truncated_hash = &hg_head[ .. config.commit_hash_length];

        module.create_segment(
            "hash",
            &config.hash.with_value(&truncated_hash),
        );

        Some(module)
    }
    else {
        log::trace!("failed to find head");

        None
    }
}

/// Call hg id to grab current commit
fn get_hg_current_commit(ctx: &Context) -> Option<String> {
    let Output { stdout, .. } = Command::new("hg")
        .arg("id")    // .args(&["id"])
        .current_dir(ctx.current_dir.as_path())
        .output()
        .ok()?;

    let stdout = String::from_utf8(stdout).ok()?;
    // .lines()     .take(1)
    let result = stdout.trim();
    if result.is_empty() { None } else { Some(result.to_string()) }
}

/// Checks whether given directory is inside some mercurial repo
fn is_dir_inside_hg_repo(path: &Path) -> bool {
    for candidate in path.ancestors() {
        let mut cp = candidate.to_path_buf();
        cp.push(".hg");
        if cp.is_dir() {
            return true;
        }
    }
    return false;
}

/// Checks whether current directory is inside some mercurial repo
pub fn is_hg_repo(ctx: &Context) -> bool {
    is_dir_inside_hg_repo(ctx.current_dir.as_path())
}

