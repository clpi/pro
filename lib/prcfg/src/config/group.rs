use serde::{Deserialize, Serialize};
use strum::{
    AsRefStr, AsStaticStr, Display, EnumIs, EnumIter, EnumMessage, EnumProperty, EnumString,
    EnumTable, EnumVariantNames, VariantArray, VariantNames,
};
use uuid::Uuid;

/// Group metadata, kind of group
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct GroupInfo {
    #[serde(default = "Uuid::default")]
    pub group_id: Uuid,
    #[serde(default = "String::default")]
    pub name: String,
    #[serde(default = "String::default")]
    pub display: String,
    #[serde(default = "Uuid::default")]
    pub founding_user_id: Uuid,
}
