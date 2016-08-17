use protocol::{Type, Error};
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub enum Slot
{
    Empty,
    Filled {
        block_id: i16,
        item_count: i8,
        item_damage: i16,
        // TODO: add NBT
    },
}

