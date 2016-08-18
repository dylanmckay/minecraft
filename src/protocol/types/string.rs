use protocol::prelude::*;
use std::io::{Read, Write};

impl ReadableType for String
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let byte_count = VarInt::read(read)?.0;

        let mut str_bytes = Vec::new();
        read.take(byte_count as _).read_to_end(&mut str_bytes)?;

        if byte_count as usize != str_bytes.len() {
            return Err(Error::BadData("string is shorter than prefix varint claims".to_owned()));
        }

        let str = String::from_utf8(str_bytes)?;
        Ok(str)
    }
}

impl WritableType for String
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let byte_count = VarInt((self.as_bytes().len() as i32));
        byte_count.write(write)?;

        write.write(self.as_bytes())?;

        Ok(())
    }
}

impl Type for String { }

