pub use self::varint::VarInt;

pub mod string;
pub mod varint;

use io::Buffer;
use std::io;

pub trait Type : Clone
{
    fn read(read: &mut Buffer) -> io::Result<Self>;
    fn write(&self, write: &mut Buffer) -> io::Result<()>;
}

