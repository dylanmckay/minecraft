use protocol::prelude::*;
use std::io::{Read, Write};
use nbt;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone,Debug)]
pub enum Slot
{
    Empty,
    Filled {
        block_id: i16,
        item_count: i8,
        item_damage: i16,
        nbt: Option<nbt::Blob>,
    },
}

impl ReadableType for Slot
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let block_id = read.read_i16::<BigEndian>()?;

        if block_id == -1 {
            Ok(Slot::Empty)
        } else {
            let item_count = read.read_i8()?;
            let item_damage = read.read_i16::<BigEndian>()?;

            let nbt = if block_id == 0 {
                None
            } else {
                Some(nbt::Blob::from_reader(read)?)
            };

            Ok(Slot::Filled {
                block_id: block_id,
                item_count: item_count,
                item_damage: item_damage,
                nbt: nbt,
            })
        }

    }
}

impl WritableType for Slot
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        match *self {
            Slot::Empty => {
                write.write_i16::<BigEndian>(-1)?;
            },
            Slot::Filled { block_id, item_count, item_damage, ref nbt } => {
                write.write_i16::<BigEndian>(block_id)?;
                write.write_i8(item_count)?;
                write.write_i16::<BigEndian>(item_damage)?;

                if let Some(ref nbt) = *nbt {
                    nbt.write(write)?;
                }
            },
        }

        Ok(())
    }
}

impl Type for Slot { }

