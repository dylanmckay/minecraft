use io::{Error, Type};
use io::types::VarInt;
use std::io::{Read, Write, Cursor};
use std;

// TODO: come up with better name
pub trait Realization : Clone + std::fmt::Debug
{
    const PACKET_ID: VarInt;
    const DESCRIPTION: &'static str;

    fn parse(read: &mut Read) -> Result<Self, Error>;

    fn write_payload(&self, write: &mut Write) -> Result<(), Error>;

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        // Write payload to temporary buffer.
        let mut payload = Cursor::new(Vec::new());
        self.packet_id().write(&mut payload)?;
        self.write_payload(&mut payload)?;


        // Write the length of the payload.
        let packet_size = payload.get_ref().len();
        VarInt(packet_size as _).write(write)?;

        write.write(payload.get_ref())?;

        Ok(())
    }

    fn packet_id(&self) -> VarInt { Self::PACKET_ID }
    fn description(&self) -> &'static str { Self::DESCRIPTION }
}

