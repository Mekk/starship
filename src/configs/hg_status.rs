use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HgStatusConfig<'a> {
    // pub ahead: SegmentConfig<'a>,
    // pub behind: SegmentConfig<'a>,
    // pub diverged: SegmentConfig<'a>,
    // pub show_sync_count: bool,
    pub added: SegmentConfig<'a>,
    pub added_count: CountConfig,
    pub deleted: SegmentConfig<'a>,
    pub deleted_count: CountConfig,
    pub missing: SegmentConfig<'a>,
    pub missing_count: CountConfig,
    pub modified: SegmentConfig<'a>,
    pub modified_count: CountConfig,
    pub untracked: SegmentConfig<'a>,
    pub untracked_count: CountConfig,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HgStatusConfig<'a> {
    fn new() -> Self {
        HgStatusConfig {
            // ahead: SegmentConfig::new("⇡"),
            // behind: SegmentConfig::new("⇣"),
            added: SegmentConfig::new("+"),
            added_count: CountConfig::default(),
            deleted: SegmentConfig::new("✘"),
            deleted_count: CountConfig::default(),
            missing: SegmentConfig::new("!"),
            missing_count: CountConfig::default(),
            modified: SegmentConfig::new("✓"),
            modified_count: CountConfig::default(),
            untracked: SegmentConfig::new("?"),
            untracked_count: CountConfig::default(),
            prefix: "[",
            suffix: "] ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}

#[derive(Clone, Copy, ModuleConfig, Default)]
pub struct CountConfig {
    pub enabled: bool,
    pub style: Option<Style>,
}
