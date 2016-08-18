use protocol::prelude::*;
use uuid::Uuid;

use std::io::prelude::*;

pub type EntityId = VarInt;

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

