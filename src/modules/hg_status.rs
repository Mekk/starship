
extern crate regex;

use regex::Regex;
use std::process::{Command, Output};
use std::error::Error;
use std::collections::HashMap;

use super::{Context, Module, RootModuleConfig};
use super::hg_commit::{is_hg_repo};

use crate::config::SegmentConfig;
use crate::configs::hg_status::{CountConfig, HgStatusConfig};

// use std::borrow::BorrowMut;


/// Creates a module with the Hg status of the currnt directory
///
/// Will display the branch name if the current directory is a Mercurial repo,
/// adding also various symbols to signal it's state.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    if ! is_hg_repo(context) {
        return None;
    }

    log::trace!("Mercurial repo detected");

    let mut module = context.new_module("hg_status");
    let config: HgStatusConfig = HgStatusConfig::try_load(module.config);

    module
        .get_prefix()
        .set_value(config.prefix)
        .set_style(config.style);
    module
        .get_suffix()
        .set_value(config.suffix)
        .set_style(config.style);
    module.set_style(config.style);

    let repo_status = get_hg_repo_status(context);
    log::debug!("Repo status: {:?}", repo_status);

    if let Ok(repo_status) = repo_status {
        create_segment_with_count(
            &mut module,
            "added",
            repo_status.added,
            &config.added,
            config.added_count,
        );

        create_segment_with_count(
            &mut module,
            "deleted",
            repo_status.deleted,
            &config.deleted,
            config.deleted_count,
        );

        create_segment_with_count(
            &mut module,
            "modified",
            repo_status.modified,
            &config.modified,
            config.modified_count,
        );

        create_segment_with_count(
            &mut module,
            "missing",
            repo_status.missing,
            &config.missing,
            config.missing_count,
        );

        create_segment_with_count(
            &mut module,
            "untracked",
            repo_status.untracked,
            &config.untracked,
            config.untracked_count,
        );
    }

    if module.is_empty() {
        return None;
    }

    Some(module)
}

fn create_segment_with_count<'a>(
    module: &mut Module<'a>,
    name: &str,
    count: usize,
    config: &SegmentConfig<'a>,
    count_config: CountConfig,
) {
    if count > 0 {
        module.create_segment(name, &config);

        if count_config.enabled {
            module.create_segment(
                &format!("{}_count", name),
                &SegmentConfig::new(&count.to_string()).with_style(count_config.style),
            );
        }
    }
}

/// Gets the number of files in various hg states (modified, deleted, etc...)
fn get_hg_repo_status(ctx: &Context) -> Result<HgStatus, Box<dyn Error>> {
    let Output { stdout, .. } = Command::new("hg")
        .arg("status")  // .args(&["status", "-A"])
        .current_dir(ctx.current_dir.as_path())
        .output()
        ?;

    log::trace!("raw hg status output: {:?}", stdout);

    let pattern = Regex::new(r"^(.)\s").unwrap();

    // List of status letters, 'A', 'M', '!', …
    let statuses :Vec<String> = String::from_utf8(stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| { cap[1].to_string() })
        .collect();

    log::trace!("file statuses: {:?}", statuses);

    // :HashMap<&String, usize> 
    let counts = statuses.iter().fold(
        HashMap::<String, usize>::new(),
        |mut cnt, x| {
            *cnt.entry(x.to_string()).or_insert(0) += 1;
            cnt
        });
             
    log::trace!("counts: {:?}", counts);

    let repo_status: HgStatus = HgStatus {
        added: *counts.get("A").unwrap_or(&0),
        deleted: *counts.get("R").unwrap_or(&0),
        modified: *counts.get("M").unwrap_or(&0),
        missing: *counts.get("!").unwrap_or(&0),
        untracked: *counts.get("&").unwrap_or(&0),
    };

    Ok(repo_status)
}

#[derive(Default, Debug, Copy, Clone)]
struct HgStatus {
    added: usize,
    deleted: usize,
    modified: usize,
    missing: usize,
    untracked: usize,
}
