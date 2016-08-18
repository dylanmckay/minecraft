use protocol::prelude::*;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

/// A variable sized int.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct VarInt(pub i32);

impl ReadableType for VarInt
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        const PART : u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;

        loop {
            let b = try!(read.read_u8()) as u32;
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
}

impl WritableType for VarInt
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        const PART : u32 = 0x7F;
        let mut val = self.0 as u32;

        loop {
            if (val & !PART) == 0 {
                try!(write.write_u8(val as u8));
                return Result::Ok(());
            }
            try!(write.write_u8(((val & PART) | 0x80) as u8));
            val >>= 7;
        }
    }
}

impl Type for VarInt { }

