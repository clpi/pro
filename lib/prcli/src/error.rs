use strum::{AsRefStr, Display, EnumProperty, EnumString, IntoStaticStr};

pub type CLiResult<T> = Result<T, CliError>;

/// PRO CLI Error type
#[repr(u8)]
#[derive(Debug, Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum CliError {
    Misc(String) = 0x0000,
    Anyhow(anyhow::Error) = 0x0001,
    #[default]
    Other = 0x0002,
}


impl CliError {
    fn new() -> Self {
        CliError::Misc("".to_string())
    }
}

impl std::error::Error for CliError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}