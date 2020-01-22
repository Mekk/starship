use std::path::{Path,PathBuf};
use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module, RootModuleConfig};

use crate::configs::hg_branch::HgBranchConfig;

/// Creates a module with the Hg bookmark or branch in the current directory
///
/// Will display the bookmark or branch name if the current directory is an hg repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let hg_dir = root_of_current_hg_repo(context);
    if hg_dir == None {
        return None;
    }
    let hg_dir = hg_dir.unwrap();

    let mut module = context.new_module("hg_branch");
    let config = HgBranchConfig::try_load(module.config);
    module.set_style(config.style);

    module.get_prefix().set_value("on ");

    module.create_segment("symbol", &config.symbol);

    // TODO: Once error handling is implemented, warn the user if their config
    // truncation length is nonsensical
    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

    let branch_name =
        get_hg_current_bookmark(&hg_dir).unwrap_or_else(|| get_hg_branch_name(&hg_dir));

    let truncated_graphemes = get_graphemes(&branch_name, len);
    // The truncation symbol should only be added if we truncated
    let truncated_and_symbol = if len < graphemes_len(&branch_name) {
        let truncation_symbol = get_graphemes(config.truncation_symbol, 1);
        truncated_graphemes + &truncation_symbol
    } else {
        truncated_graphemes
    };

    module.create_segment(
        "name",
        &config.branch_name.with_value(&truncated_and_symbol),
    );

    Some(module)
}

fn get_hg_branch_name(hg_dir: &Path) -> String {
    std::fs::read_to_string(hg_dir.join("branch"))
        .map(|s| s.trim().into())
        .unwrap_or_else(|_| "default".to_string())
}

fn get_hg_current_bookmark(hg_dir: &Path) -> Option<String> {
    std::fs::read_to_string(hg_dir.join("bookmarks.current"))
        .map(|s| s.trim().into())
        .ok()
}

fn get_graphemes(text: &str, length: usize) -> String {
    UnicodeSegmentation::graphemes(text, true)
        .take(length)
        .collect::<Vec<&str>>()
        .concat()
}

fn graphemes_len(text: &str) -> usize {
    UnicodeSegmentation::graphemes(&text[..], true).count()
}


// TODO: merge is_dir_inside_hg_repo and root_of_current_hg_repo
//       into some helper module

/// Returns /path/to/.hg directory of repo containing current
/// dir or None if we aren't inside one.
pub fn root_of_current_hg_repo(ctx: &Context) -> Option<PathBuf> {
    return root_of_hg_repo(ctx.current_dir.as_path());
}

fn root_of_hg_repo(path: &Path) -> Option<PathBuf> {
    for candidate in path.ancestors() {
        let mut cp = candidate.to_path_buf();
        cp.push(".hg");
        if cp.is_dir() {
            return Some(cp);
        }
    }
    return None;
}
