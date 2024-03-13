pub mod config;
pub mod util;
pub(crate) mod error;

pub use self::{
    error::{CfgErr, CfgResult},
    util::*,
    config::{
        Cfg,
        group::GroupInfo,
        local::{CfgLocal, CfgSpace},
        rt::CfgRt,
        shell::CfgShell,
        inst::{CfgInst, ServerData},
        ui::{Style, LsColors},
    },
};