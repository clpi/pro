use std::collections::BTreeMap;
use std::net::SocketAddrV4;
use std::path::PathBuf;

use super::*;
use dirs::{
    audio_dir, cache_dir, config_dir, config_local_dir, data_dir, data_local_dir, document_dir,
    executable_dir, home_dir, picture_dir, preference_dir, public_dir, runtime_dir, state_dir,
    template_dir, video_dir,
};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant};
use serde::{Deserialize, Serialize};
use strum::{
    Display, EnumDiscriminants, EnumIs, EnumIter, EnumMessage, EnumProperty, EnumString, FromRepr,
};
use uuid::Uuid;

/// The shell config
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct CfgShell {
    #[serde(default = "CfgShell::zsh")]
    pub shell: String,
    #[serde(default = "BTreeMap::<String, super::ui::LsColors>::default")]
    pub color_map: BTreeMap<String, super::ui::LsColors>,
    #[serde(default = "BTreeMap::<String, String>::default")]
    pub key_map: BTreeMap<String, String>,
    #[serde(default = "BTreeMap::<String, String>::default")]
    pub opt_map: BTreeMap<String, String>,
    #[serde(default = "String::default")]
    pub font: String,
}
impl CfgShell {
    pub fn zsh() -> String {
        "/bin/zsh".into()
    }
    pub fn bash() -> String {
        "/bin/bash".into()
    }
    pub fn empty_color_map() -> BTreeMap<String, String> {
        BTreeMap::default()
    }
}
