pub use self::varint::VarInt;

pub mod string;
pub mod varint;

use io::{Buffer, Error};

pub trait Type : Clone
{
    fn read(read: &mut Buffer) -> Result<Self, Error>;
    fn write(&self, write: &mut Buffer) -> Result<(), Error>;
}

