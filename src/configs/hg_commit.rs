use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HgCommitConfig<'a> {
    pub commit_hash_length: usize,  // Strange name in sync with git_commit…
    pub hash: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HgCommitConfig<'a> {
    fn new() -> Self {
        HgCommitConfig {
            commit_hash_length: 6,
            hash: SegmentConfig::default(),
            prefix: "(",
            suffix: ") ",
            style: Color::Green.bold(),
            disabled: true,
        }
    }
}
