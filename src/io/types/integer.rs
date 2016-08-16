use io::{Buffer, Type, Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

impl Type for u16
{
    fn read(buffer: &mut Buffer) -> Result<Self, Error> {
        Ok(buffer.read_u16::<BigEndian>()?)
    }

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        buffer.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}

