pub use self::varint::VarInt;
pub use self::composite::Composite;
pub use self::aliases::*;

pub mod string;
pub mod integer;
pub mod varint;
pub mod composite;
pub mod aliases;

use protocol::Error;

pub type ByteArray = Composite<u8>;

pub trait Type : Clone + ::std::fmt::Debug
{
    fn read(read: &mut ::std::io::Read) -> Result<Self, Error>;
    fn write(&self, write: &mut ::std::io::Write) -> Result<(), Error>;

    fn write_vec(&self) -> Result<Vec<u8>, Error> {
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());
        self.write(&mut buffer)?;

        Ok(buffer.into_inner())
    }

    fn size_bytes(&self) -> usize {
        self.write_vec().unwrap().len()
    }
}

