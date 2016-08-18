use protocol::prelude::*;
use std::io::prelude::*;

#[derive(Clone, Debug)]
pub struct ChunkData
{
    // pub chunk_x: Int,
    // pub chunk_z: Int,
    // pub ground_up_continuous: bool,
    // pub primary_bit_mask: VarInt,
    // pub size: VarInt,
    // pub data: Array<ChunkSection>,
    // pub biomes: Option<Array<u8>>,
    // pub block_entities: Array<nbt::Blob>,
}

impl ::protocol::packet::Realization for ChunkData
{
    const PACKET_ID: VarInt = VarInt(0x20);
    const DESCRIPTION: &'static str = "ChunkData";

    fn parse(_read: &mut Read) -> Result<Self, Error> {
        // let (chunk_x, chunk_z) = (Int::read(read)?, Int::read(read)?);
        // let ground_up_continuous = bool::read(read)?;
        // let primary_bit_mask = VarInt::read(read)?;
        // let size = VarInt::read(read)?;
        //
        // let array_section_length = primary_bit_mask.0.count_ones();
        // let data = Array::read_fixed_length(array_section_length as _, read)?;
        //
        // let biomes = if ground_up_continuous {
        //     Some(Array::read_fixed_length(256, read)?)
        // } else {
        //     None
        // };
        //
        // let block_entities = Array::read(read)?;
        //
        // Ok(ChunkData {
        //     chunk_x: chunk_x, chunk_z: chunk_z,
        //     ground_up_continuous: ground_up_continuous,
        //     primary_bit_mask: primary_bit_mask,
        //     size: size,
        //     data: data,
        //     biomes: biomes,
        //     block_entities: block_entities,
        // })
        Ok(ChunkData { })
    }

    fn write_payload(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
    }
}

