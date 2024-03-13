#[derive(Clone, PartialEq, Debug, Default)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    Anyhow(String),
    #[default]
    Other,
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}
impl From<std::num::ParseFloatError> for Error {
    fn from(value: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(value)
    }
}
impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Anyhow(value.to_string())
    }
}
