pub use self::varint::VarInt;
pub use self::composite::Composite;

pub mod string;
pub mod integer;
pub mod varint;
pub mod composite;

use io::Error;

pub type ByteArray = Composite<u8>;

pub trait Type : Clone + ::std::fmt::Debug
{
    fn read(read: &mut ::std::io::Read) -> Result<Self, Error>;
    fn write(&self, write: &mut ::std::io::Write) -> Result<(), Error>;
}

