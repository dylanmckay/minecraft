use io::{Type, Error};
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

impl Type for u8
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_u8()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u8(*self)?;
        Ok(())
    }
}

impl Type for u16
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_u16::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Type for i32
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_i32::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_i32::<BigEndian>(*self)?;
        Ok(())
    }
}

