use protocol::prelude::*;
use std::io::{Read, Write};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Statistic
{
    pub name: String,
    pub value: VarInt,
}

impl ReadableType for Statistic
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let name = String::read(read)?;
        let value = VarInt::read(read)?;

        Ok(Statistic { name: name, value: value })
    }
}

impl WritableType for Statistic
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.name.write(write)?;
        self.value.write(write)?;

        Ok(())
    }
}

impl Type for Statistic { }

