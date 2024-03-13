use serde::{Deserialize, Serialize};
use strum::{
    AsRefStr, AsStaticRef, AsStaticStr, Display, EnumIs, EnumIter, EnumMessage, EnumProperty,
    EnumString, EnumTable, EnumVariantNames, FromRepr, IntoStaticStr, ParseError, VariantArray,
    VariantNames,
};
use uuid::Uuid;

/// The color mapped to the entity, with style(s)
#[derive(
    Debug,
    Default,
    Display,
    EnumMessage,
    EnumProperty,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Hash,
    Serialize,
    FromRepr,
    strum::IntoStaticStr,
    strum::VariantNames,
    Deserialize,
    EnumIs,
    EnumIter,
    EnumString,
)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(tag = "color", content = "value")]
#[repr(i8)]
pub enum LsColors {
    #[default]
    Reset = 0,
    White(Vec<Style>) = 8,
    Red(Vec<Style>) = 1,
    Blue(Vec<Style>) = 4,
    Cyan(Vec<Style>) = 7,
    Green(Vec<Style>) = 2,
    Yellow(Vec<Style>) = 3,
    Magenta(Vec<Style>) = 5,
    Gray(Vec<Style>) = 6,
}
/// Style of colormap. Multiple may be applied
#[derive(
    Debug,
    Default,
    Display,
    EnumMessage,
    EnumProperty,
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
    EnumString,
)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[serde(untagged)]
#[repr(u8)]
pub enum Style {
    #[default]
    Unstyled = 0,
    Bold = 1,
    Italic = 2,
    Dim = 3,
    Flash = 4,
    Underlined = 5,
}
