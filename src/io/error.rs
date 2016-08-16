use std;

pub enum Error
{
    BadData(String),
    Io(std::io::Error),
    InvalidUtf16(std::string::FromUtf16Error),
}

impl From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::string::FromUtf16Error> for Error
{
    fn from(error: std::string::FromUtf16Error) -> Self {
        Error::InvalidUtf16(error)
    }
}

