use std::path::PathBuf;

use dirs::{config_dir, config_local_dir, data_dir, data_local_dir, executable_dir, home_dir};
use serde::{Deserialize, Serialize};

/// Any personal data, matters, etc.
#[derive(Debug, Clone, PartialEq, Hash,  Eq, Ord, PartialOrd, Serialize, Default, Deserialize)]
pub struct CfgLocal {
    #[serde(skip)]
    config_dir:  PathBuf,
    pub spaces: Vec<CfgSpace>,

}

/// A configuration space
#[derive(Debug, Clone, PartialEq, Hash,  Eq, Ord, PartialOrd, Serialize, Default, Deserialize)]
pub struct CfgSpace {
    dir: PathBuf,
    pub name: String,
    pub path: PathBuf,
    pub active: bool,

}

impl CfgSpace {
    pub fn new<P: Into<PathBuf>>(name: &str, path: P, dir: P) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            dir: dir.into(),
            active: true,
        }
    }
    pub fn shut_down(&mut self) {
        self.active = false;
    }
}