use protocol::{Type, Error};
use std::io::{Read, Write};

#[derive(Clone,Debug)]
pub struct Chat
{
    value: String,
}

impl Type for Chat
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(Chat { value: String::read(read)? })
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.value.write(write)?;
        Ok(())
    }
}

