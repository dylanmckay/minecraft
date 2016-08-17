use protocol;
use std;

#[derive(Debug)]
pub enum Error
{
    BadData(String),
    Io(std::io::Error),
    InvalidUtf8(std::string::FromUtf8Error),
    UnknownPacket(protocol::packet::Data),
    Nbt(::nbt::Error),
}

impl From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::string::FromUtf8Error> for Error
{
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::InvalidUtf8(error)
    }
}

impl From<::nbt::Error> for Error
{
    fn from(e: ::nbt::Error) -> Self {
        Error::Nbt(e)
    }
}

