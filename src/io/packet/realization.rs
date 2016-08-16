use io::{Buffer, Error};

// TODO: come up with better name
pub trait Realization : Clone
{
    const PACKET_ID: i32;

    fn parse(data: Vec<u8>) -> Result<Self, Error>;

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error>;
}

