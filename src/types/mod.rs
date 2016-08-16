pub use self::varint::VarInt;

pub mod string;
pub mod varint;

use std::io;

pub type Buffer = io::Cursor<Vec<u8>>;

pub trait Type : Clone
{
    fn read(read: &mut Buffer) -> io::Result<Self>;
    fn write(&self, write: &mut Buffer) -> io::Result<()>;
}

