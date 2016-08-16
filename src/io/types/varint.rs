use io::{Type, Buffer, Error};
use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

/// A variable sized int.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct VarInt(pub i32);

impl VarInt
{
    pub fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        const PART : u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;

        loop {
            let b = try!(buf.read_u8()) as u32;
            val |= (b & PART) << (size * 7);
            size += 1;

            if size > 5 {
                // TODO: turn this into error result
                panic!("varint too big");
            }
            if (b & 0x80) == 0 {
                break
            }
        }

        Result::Ok(VarInt(val as i32))
    }

    pub fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        const PART : u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                try!(buf.write_u8(val as u8));
                return Result::Ok(());
            }
            try!(buf.write_u8(((val & PART) | 0x80) as u8));
            val >>= 7;
        }
    }
}

impl Type for VarInt
{
    fn read(buffer: &mut Buffer) -> Result<Self, Error> {
        VarInt::read_from(buffer)
    }

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        self.write_to(buffer)
    }
}

