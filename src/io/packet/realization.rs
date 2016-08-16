use io::{Buffer, Error, Type};
use io::types::VarInt;
use std;

// TODO: come up with better name
pub trait Realization : Clone + std::fmt::Debug
{
    const PACKET_ID: VarInt;

    fn parse(data: Vec<u8>) -> Result<Self, Error>;

    fn write_payload(&self, buffer: &mut Buffer) -> Result<(), Error>;

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        use std::io::Write;

        println!("position before: {}", buffer.position());

        // Write payload to temporary buffer.
        let mut payload = Buffer::new(Vec::new());
        self.packet_id().write(&mut payload)?;
        self.write_payload(&mut payload)?;


        // Write the length of the payload.
        let packet_size = payload.get_ref().len();

        println!("packet size: {}", packet_size);
        VarInt(packet_size as _).write(buffer)?;

        buffer.write(payload.get_ref())?;
        println!("position after: {}", buffer.position());

        Ok(())
    }

    fn packet_id(&self) -> VarInt { Self::PACKET_ID }
}

