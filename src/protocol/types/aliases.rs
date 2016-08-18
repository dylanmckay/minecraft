use protocol::prelude::*;
use uuid::Uuid;

use std::io::prelude::*;

pub type EntityId = VarInt;

impl ReadableType for Uuid
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let uuid_str = String::read(read)?;
        Ok(Uuid::parse_str(&uuid_str)?)
    }
}

impl WritableType for Uuid
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let uuid_str = self.simple().to_string();
        uuid_str.write(write)
    }
}

impl Type for Uuid { }

