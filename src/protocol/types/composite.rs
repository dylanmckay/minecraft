use protocol::prelude::*;
use std::io::{Read, Write};

/// A list of other types.
#[derive(Clone, Debug)]
pub struct Composite<T>
{
    pub elements: Vec<T>,
}

impl<T> Composite<T>
{
    pub fn read_with<F>(read: &mut Read, mut f: F) -> Result<Self, Error>
        where F: FnMut(&mut Read) -> Result<T, Error> {
        let length = VarInt::read(read)?;

        // FIXME: validate the length so we don't read too much data.

        let mut items = Vec::new();
        for _ in 0..length.0 {
            items.push(f(read)?);
        }

        Ok(Composite { elements: items })
    }
}

impl<T: ReadableType> ReadableType for Composite<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Composite::read_with(read, |read| T::read(read))
    }
}

impl<T: WritableType> WritableType for Composite<T>
{
    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let length = VarInt(self.elements.len() as _);
        length.write(write)?;

        for element in self.elements.iter() {
            element.write(write)?;
        }

        Ok(())
    }
}

impl<T: Type> Type for Composite<T> { }

