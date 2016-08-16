use io::{Buffer, Type, Error};
use io::types::VarInt;

impl<'a> Type for String
{
    fn read(buffer: &mut Buffer) -> Result<Self, Error> {
        use std::io::Read;

        let byte_count = VarInt::read(buffer)?.0;

        let mut str_bytes = Vec::new();
        buffer.take(byte_count as _).read_to_end(&mut str_bytes)?;

        if byte_count as usize != str_bytes.len() {
            return Err(Error::BadData("string is shorter than prefix varint claims".to_owned()));
        }

        let str = String::from_utf8(str_bytes)?;
        Ok(str)
    }

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        use std::io::Write;

        let byte_count = VarInt((self.as_bytes().len() as i32));
        byte_count.write(buffer)?;

        buffer.write(self.as_bytes())?;

        Ok(())
    }
}

