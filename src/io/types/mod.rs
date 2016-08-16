pub use self::varint::VarInt;
pub use self::composite::Composite;

pub mod string;
pub mod integer;
pub mod varint;
pub mod composite;

use io::{Buffer, Error};

pub type ByteArray = Composite<u8>;

pub trait Type : Clone + ::std::fmt::Debug
{
    fn read(read: &mut Buffer) -> Result<Self, Error>;
    fn write(&self, write: &mut Buffer) -> Result<(), Error>;
}

