use protocol::prelude::*;
use uuid::Uuid;
use nbt;

use std::io::prelude::*;

pub type EntityId = VarInt;
pub type Int = i32;
pub type Long = i64;

impl ReadableType for Uuid
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let val1 = u64::read(read)?;
        let val2 = u64::read(read)?;

        let bytes1: [u8; 8] = unsafe { ::std::mem::transmute(val1) };
        let bytes2: [u8; 8] = unsafe { ::std::mem::transmute(val2) };
        let bytes: Vec<u8> = [bytes1, bytes2].iter().flat_map(|a| a.iter().cloned()).collect();

        Ok(Uuid::from_bytes(&bytes)?)
    }
}

impl WritableType for Uuid
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        write.write(self.as_bytes())?;
        Ok(())
    }
}

impl Type for Uuid { }


impl ReadableType for nbt::Blob
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Ok(nbt::Blob::from_reader(read)?)
    }
}

impl WritableType for nbt::Blob
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        nbt::Blob::write(self, write)?;
        Ok(())
    }
}

impl Type for nbt::Blob { }

