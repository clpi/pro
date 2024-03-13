use serde::{Deserialize, Serialize};
use strum::{
    AsRefStr, AsStaticStr, Display, EnumIs, EnumIter, EnumMessage, EnumProperty, EnumString,
    EnumTable, EnumVariantNames, VariantArray, VariantNames,
};

/// Major preferences of the user on the network specifically
#[derive(
    Display,
    EnumIter,
    EnumIs,
    Serialize,
    Debug,
    Deserialize,
    Default,
    EnumString,
    EnumProperty,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
)]
#[repr(C)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "participation", content = "preference")]
pub enum Preference {
    #[default]
    Observe = 0x0000,
    Contribute = 0x0001,
    Active = 0x0002,
}

/// Full net privelege and role
#[repr(u8)]
#[derive(
    Debug,
    Default,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    EnumIs,
    EnumIter,
    VariantNames,
    EnumMessage,
    EnumString,
    EnumProperty,
)]

/// Privelege associated with a position of authority in the network
#[repr(C)]
#[serde(tag = "privelege", content = "role")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Privelege {
    #[default]
    Guest,
    Admin(AdminRole) = 0x0001,
    Moderator(ModRole) = 0x0002,
    HighUser(HighUser) = 0x0003,
    User(UserRole) = 0x0004,
    LowUser(LowUser) = 0x0005,
}

/// Private data associated with a LowUser and their relegated permissions
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct LowUser {
    pub kp: (String, String),
    pub display: String,
}

/// Private data associated with a HighUser and their relegated permissions
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct HighUser {
    pub kp: (String, String),
    pub display: String,
}

/// Private data associated with a Normal User and their relegated permissions
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct UserRole {
    pub kp: (String, String),
    pub display: String,
}

/// Private data associated with a Guest User and their relegated permissions
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct GuestRole {
    pub display: Option<String>,
}

/// Private data associated with a Global/Local mod and their relegated permissions
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct ModRole {
    pub kp: (String, String),
    pub display: String,
}

/// Private data associated with a Global/Local Admin and their relegated permissions
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct AdminRole {
    pub kp: (String, String),
    pub display: String,
}

/// The meta part of the config
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct CfgMeta {
    pub user_id: String,
    pub priv_key: String,
    pub participation: Preference,
    #[serde(default = "Privelege::default")]
    pub privelege: Privelege,
}
