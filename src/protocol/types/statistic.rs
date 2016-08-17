use protocol::{Type, Error};
use protocol::types::VarInt;
use std::io::{Read, Write};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Statistic
{
    pub name: String,
    pub value: VarInt,
}

impl Type for Statistic
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let name = String::read(read)?;
        let value = VarInt::read(read)?;

        Ok(Statistic { name: name, value: value })
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.name.write(write)?;
        self.value.write(write)?;

        Ok(())
    }
}

