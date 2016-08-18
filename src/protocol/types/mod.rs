pub use self::varint::VarInt;
pub use self::array::Array;
pub use self::position::Position;
pub use self::aliases::*;
pub use self::slot::Slot;
pub use self::chat::Chat;

#[macro_use]
pub mod composite;
pub mod string;
pub mod integer;
pub mod varint;
pub mod array;
pub mod position;
pub mod slot;
pub mod chat;
pub mod aliases;

use protocol::Error;

pub type ByteArray = Array<u8>;

pub trait ReadableType : Clone + ::std::fmt::Debug
{
    fn read(read: &mut ::std::io::Read) -> Result<Self, Error>;

    fn optional_read(read: &mut ::std::io::Read) -> Result<Option<Self>, Error> {
        let has_value = bool::read(read)?;

        if has_value {
            Self::read(read).map(|v| Some(v))
        } else {
            Ok(None)
        }
    }
}

impl<T: ReadableType> ReadableType for Option<T>
{
    fn read(read: &mut ::std::io::Read) -> Result<Self, Error> {
        T::optional_read(read)
    }
}


pub trait WritableType : Clone + ::std::fmt::Debug
{
    fn write(&self, write: &mut ::std::io::Write) -> Result<(), Error>;

    fn optional_write(value: &Option<Self>, write: &mut ::std::io::Write) -> Result<(), Error> {
        use byteorder::WriteBytesExt;

        if let Some(ref v) = *value {
            write.write_u8(1)?;
            v.write(write)?;
        } else {
            write.write_u8(0)?;
        }

        Ok(())
    }

    fn write_vec(&self) -> Result<Vec<u8>, Error> {
        use std::io::Cursor;

        let mut buffer = Cursor::new(Vec::new());
        self.write(&mut buffer)?;

        Ok(buffer.into_inner())
    }
}

impl<T: WritableType> WritableType for Option<T>
{
    fn write(&self, write: &mut ::std::io::Write) -> Result<(), Error> {
        T::optional_write(self, write)
    }
}

pub trait Type : Clone + ::std::fmt::Debug + ReadableType + WritableType
{
    fn size_bytes(&self) -> usize {
        self.write_vec().unwrap().len()
    }
}

impl<T: Type> Type for Option<T> { }

