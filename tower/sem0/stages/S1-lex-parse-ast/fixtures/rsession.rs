use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq)]
pub enum OptLevel {
    No,
    Less,
    Default,
    Aggressive,
    Size,
}

#[derive(Clone)]
pub enum DebugInfo {
    None,
    Limited,
    Full,
}

pub struct Options {
    pub opt_level: OptLevel,
    pub debuginfo: DebugInfo,
    pub crate_name: Option<String>,
    pub search_paths: Vec<String>,
    pub cfg: BTreeMap<String, Option<String>>,
    pub incremental: bool,
    pub edition: Edition,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Edition {
    Edition2015,
    Edition2018,
    Edition2021,
    Edition2024,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            opt_level: OptLevel::No,
            debuginfo: DebugInfo::None,
            crate_name: None,
            search_paths: Vec::new(),
            cfg: BTreeMap::new(),
            incremental: false,
            edition: Edition::Edition2021,
        }
    }
}

impl Options {
    pub fn is_optimized(&self) -> bool {
        !matches!(self.opt_level, OptLevel::No)
    }

    pub fn set_cfg(&mut self, key: &str, val: Option<&str>) {
        self.cfg.insert(key.to_string(), val.map(|v| v.to_string()));
    }
}

pub const DEFAULT_EDITION: Edition = Edition::Edition2021;
