use protocol::{Type, Error};
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

impl Type for bool
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let byte = read.read_u8()?;

        Ok(if byte == 0 { false } else { true })
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Type for u8
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_u8()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_u8(*self)?; Ok(()) }
}

impl Type for i8
{
    fn read(read: &mut Read) -> Result<Self, Error> { Ok(read.read_i8()?) }
    fn write(&self, write: &mut Write) -> Result<(), Error> { write.write_i8(*self)?; Ok(()) }
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

impl Type for i16
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_i16::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_i16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Type for u32
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_u32::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u32::<BigEndian>(*self)?;
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

impl Type for u64
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_u64::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_u64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Type for i64
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_i64::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Type for f32
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_f32::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_f32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Type for f64
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(read.read_f64::<BigEndian>()?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write_f64::<BigEndian>(*self)?;
        Ok(())
    }
}


