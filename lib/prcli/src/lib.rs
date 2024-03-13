#[allow(unused_imports)]

pub(crate) mod error;
pub mod cli;
pub mod util;

pub mod prelude {
    pub use super::{
        cli::{Root, Cmd, self},
        util::*,
        error::{CLiResult, CliError},
    };
}