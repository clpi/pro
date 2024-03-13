#![allow(clippy::type_id_on_box)]
#![allow(type_alias_bounds)]
#![allow(semicolon_in_expressions_from_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::{
    any, borrow::{Borrow, Cow}, error::Error,
};

use anyhow::Context;
use serde::de::Error as SError;
use strum::{EnumProperty, EnumIter, Display, EnumDiscriminants, EnumIs, EnumMessage, EnumString, EnumTryAs, FromRepr};

pub type CfgResult<T> = Result<T, CfgErr<'static, 'static>>;

#[derive(Debug, Display, EnumIs, EnumTryAs, EnumMessage, EnumDiscriminants)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum CfgErr<'a, 'b> {
    #[strum(to_string = "parse_in")]
    ParseIn(ParseInErr<'a>),
    #[strum(to_string = "parse_out")]
    ParseOut(ParseOutErr<'b>),
    #[strum(to_string = "misc")]
    Misc(anyhow::Error),
}

#[repr(u8)]
#[derive(Debug, Display, EnumIs,EnumTryAs, EnumMessage, EnumDiscriminants)]
pub enum ParseInErr<'a> {
    Toml(toml::ser::Error) = 0x00,
    Io(std::io::Error) = 0x01,
    Anyhow(anyhow::Error) = 0x02,
    UuidParseError(uuid::Error) = 0x03,
    Misc(Cow<'a, str>) = 0x04,
}

#[repr(u8)]
#[derive(Debug, Display, EnumIs, EnumTryAs, EnumMessage, EnumDiscriminants)]
pub enum ParseOutErr<'b> {
    Toml(toml::ser::Error) = 0x00,
    Io(std::io::Error) = 0x01,
    Anyhow(anyhow::Error) = 0x02,
    UuidParseError(uuid::Error) = 0x03,
    Misc(Cow<'b, str>) = 0x04,
}

impl<'a> From<anyhow::Error> for ParseInErr<'a> {
    fn from(value: anyhow::Error) -> Self {
        ParseInErr::Anyhow(value)
    }
}
impl<'b> From<anyhow::Error> for ParseOutErr<'b> {
    fn from(value: anyhow::Error) -> Self {
        ParseOutErr::Anyhow(value)
    }
}
impl<'a> From<std::io::Error> for ParseInErr<'a> {
    fn from(value: std::io::Error) -> Self {
        ParseInErr::Io(value)
    }
}
impl<'b> From<std::io::Error> for ParseOutErr<'b> {
    fn from(value: std::io::Error) -> Self {
        ParseOutErr::Io(value)
    }
}
impl<'a> From<uuid::Error> for ParseInErr<'a> {
    fn from(value: uuid::Error) -> Self {
        ParseInErr::UuidParseError(value)
    }
}
impl<'b> From<uuid::Error> for ParseOutErr<'b> {
    fn from(value: uuid::Error) -> Self {
        ParseOutErr::UuidParseError(value)
    }
}   
impl<'a, 'b> From<ParseInErr<'a>> for CfgErr<'a, 'b> {
    fn from(value: ParseInErr<'a>) -> Self {
        match value {
            ParseInErr::Anyhow(e) => CfgErr::<'a, 'b>::ParseIn(e.into()),
            ParseInErr::Io(e) => CfgErr::<'a, 'b>::ParseIn(e.into()),
            ParseInErr::Misc(e) => CfgErr::<'a, 'b>::ParseIn(ParseInErr::<'a>::Misc(Cow::Owned(e.to_string()))),
            ParseInErr::Toml(e) => CfgErr::<'a, 'b>::ParseIn(ParseInErr::<'a>::Toml(e.into())),
            ParseInErr::UuidParseError(e) => CfgErr::<'a, 'b>::ParseIn(e.into()),
        }
    }
}
impl<'a, 'b> From<ParseOutErr<'a>> for CfgErr<'a, 'b> {
    fn from(value: ParseOutErr<'a>) -> CfgErr<'a, 'b> {
        let val = value.get_detailed_message().unwrap_or_default();
        CfgErr::<'a, 'b>::ParseOut(ParseOutErr::Misc(Cow::from(val)))
    }
}
impl<'a, 'b> From<CfgErr<'a, 'b>> for std::io::Error {
    fn from(value: CfgErr<'a, 'b>) -> Self {
        use std::io::{Error, ErrorKind};
        match value {
            CfgErr::Misc(s) => Error::new(ErrorKind::Other, Cow::Owned(s.to_string())),
            CfgErr::ParseIn(e) => Error::new(ErrorKind::Other, Cow::Owned(e.to_string())),
            CfgErr::ParseOut(e) => Error::new(ErrorKind::Other, Cow::Owned(e.to_string())),
        }
    }
}
impl<'a, 'b> From<CfgErr<'a, 'b>> for std::io::Result<()> {
    fn from(value: CfgErr<'a, 'b>) -> Self {
        Err(std::io::Error::new(std::io::ErrorKind::Other, value.to_string()))
    }
}
impl<'a, 'b> From<CfgErr<'a, 'b>> for anyhow::Result<()> {
    fn from(value: CfgErr<'a, 'b>) -> Self {
        Err(anyhow::Error::msg(value.to_string()))
    }
}
impl<'a> std::error::Error for ParseInErr<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseInErr::Io(e) => Some(e),
            ParseInErr::Toml(e) => Some(e),
            ParseInErr::Anyhow(e) => e.root_cause().source(),
            ParseInErr::UuidParseError(e) => Some(e),
            ParseInErr::Misc(s) => None,
        }
    }
}
impl<'b> std::error::Error for ParseOutErr<'b> {
    fn description(&self) -> &str {
        match self {
            ParseOutErr::Toml(e) => e.description(),
            ParseOutErr::Io(e) => e.description(),
            ParseOutErr::Anyhow(e) => e.root_cause().description(),
            ParseOutErr::UuidParseError(e) => e.description(),
            ParseOutErr::Misc(e) => e.borrow(),
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            ParseOutErr::Io(e) => Some(e),
            Self::Toml(e) => Some(e),
            ParseOutErr::Anyhow(e) => Some(e.root_cause()),
            ParseOutErr::UuidParseError(e) => Some(e),
            Self::Misc(s) => None,
        }
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseOutErr::Toml(e) => Some(e),
            ParseOutErr::Io(e) => Some(e),
            ParseOutErr::Anyhow(e) => e.root_cause().source(),
            ParseOutErr::UuidParseError(e) => Some(e),
            ParseOutErr::Misc(e) => None,
        }
    }
}
impl<'a, 'b> std::error::Error for CfgErr<'_, '_> {
    fn description(&'_ self) -> &'_ str {
        match self {
            CfgErr::Misc(s) => "",
            CfgErr::ParseIn(e) => match e {
                ParseInErr::Toml(e) => e.description(),
                ParseInErr::Io(e) => e.description(),
                ParseInErr::Anyhow(e) => e.root_cause().description(),
                ParseInErr::UuidParseError(e) => e.description(),
                ParseInErr::Misc(e) => e.borrow(),
            }
            CfgErr::ParseOut(e) => match e {
                ParseOutErr::Toml(e) => e.description(),
                ParseOutErr::Io(e) => e.description(),
                ParseOutErr::Anyhow(e) => e.root_cause().description(),
                ParseOutErr::UuidParseError(e) => e.description(),
                ParseOutErr::Misc(e) => e.borrow(),
            }
        }
    }
    fn cause(&'_ self) -> Option<&dyn Error> {
        match self {
            CfgErr::Misc(s) => None,
            CfgErr::ParseIn(e) => None,
            CfgErr::ParseOut(e) => None,
        }
    }
    fn source(&'_ self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CfgErr::Misc(s) => None,
            CfgErr::ParseIn(e) => None,
            CfgErr::ParseOut(e) => None,
        }
    }
}