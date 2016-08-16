use io::types::*;
use io::packet;
use io::Error;
use std;

pub const PACKET_ID: i32 = 0x00;

#[derive(Clone)]
pub struct LoginStart
{
    pub username: String,
}

impl packet::Realization for LoginStart
{
    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(LoginStart {
            username: String::read(&mut cursor)?,
        })
    }
}

