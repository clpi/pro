pub mod group;
pub mod inst;
pub mod local;
pub mod rt;
pub mod meta;
pub mod shell;
pub mod ui;

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

/// Configuration for the client
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct GuestRole {
    pub display_name: String,
    pub kpair: (String, String),
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct UserRole {
    pub display_name: String,
    pub kpair: (String, String),
}

/// The top-level config
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct Cfg {
    #[serde(skip)]
    data_dir: PathBuf,
    pub meta: meta::CfgMeta,
    pub local: local::CfgLocal,
}

impl Cfg {}
