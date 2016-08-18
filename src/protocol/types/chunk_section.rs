use protocol::prelude::*;
use std::io::prelude::*;

pub const SIDE_LENGTH: usize = 16;
pub const TOTAL_BLOCKS: usize = SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH;

#[derive(Clone, Debug)]
pub struct ChunkSection
{
    pub bits_per_block: u8,
    pub palette: Array<VarInt>,
    pub data: Array<Long>,
    pub block_light: Array<u8>,
    pub sky_light: Option<Array<u8>>,
}

impl ReadableType for ChunkSection
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let bits_per_block = u8::read(read)?;
        let palette = Array::read(read)?;
        let data = Array::read(read)?;
        let block_light = Array::read_fixed_length(TOTAL_BLOCKS / 2, read)?;

        let mut sky_light_bytes = Vec::new();
        read.read_to_end(&mut sky_light_bytes)?;

        let sky_light = if !sky_light_bytes.is_empty() {
            Some(Array { elements: sky_light_bytes })
        } else {
            None
        };

        Ok(ChunkSection {
            bits_per_block: bits_per_block,
            palette: palette,
            data: data,
            block_light: block_light,
            sky_light: sky_light,
        })
    }
}

impl WritableType for ChunkSection
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        self.bits_per_block.write(write)?;
        self.palette.write(write)?;
        self.data.write(write)?;
        self.block_light.write(write)?;

        if let Some(ref sky_light) = self.sky_light {
            sky_light.write(write)?;
        }

        Ok(())
    }
}

impl Type for ChunkSection { }

