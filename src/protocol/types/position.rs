use protocol::prelude::*;
use std::io::{Read, Write};

use byteorder::{BigEndian,ReadBytesExt, WriteBytesExt};

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ReadableType for Position {
    fn read(read: &mut Read) -> Result<Self, Error> {
        let val = read.read_u64::<BigEndian>()?;

        Ok(Position {
            x: (val >> 38) as i32,
            y: ((val >> 26) & 0xFFF) as i32,
            z: (val << 38 >> 38) as i32,
        })
    }
}

impl WritableType for Position
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let (x,y,z) = (self.x as u64, self.y as u64, self.z as u64);
        let val = ((x & 0x3FFFFFF) << 38) | ((y & 0xFFF) << 26) | (z & 0x3FFFFFF);

        write.write_u64::<BigEndian>(val)?;
        Ok(())
    }
}

impl Type for Position { }

impl Into<::na::Vector3<f32>> for Position
{
    fn into(self) -> ::na::Vector3<f32> {
        ::na::Vector3::new(self.x as _, self.y as _, self.z as _)
    }
}

