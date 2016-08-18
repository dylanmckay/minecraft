use protocol::prelude::*;
use std::io::{Read, Write};

#[derive(Clone,Debug)]
pub struct Chat
{
    value: String,
}

impl ReadableType for Chat
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(Chat { value: String::read(read)? })
    }
}

impl WritableType for Chat
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.value.write(write)?;
        Ok(())
    }
}

impl Type for Chat
{
}

