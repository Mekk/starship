use std::process::{Command, Output};

use super::{Context, Module, RootModuleConfig};

use crate::configs::hg_commit::HgCommitConfig;

/// Creates a module with the Hg commit in the current directory
///
/// Will display the commit hash if the current directory is a hg repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_hg_repo = context.try_begin_scan()?.set_folders(&[".hg"]).is_match();
    if !is_hg_repo {
        return None;
    }

    let mut module = context.new_module("hg_commit");
    let config = HgCommitConfig::try_load(module.config);
    if config.disabled {
        return None;
    };

    let hg_head_info = get_hg_current_commit(context);

    if let Some(hg_head) = hg_head_info {
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
