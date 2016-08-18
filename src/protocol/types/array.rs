use protocol::prelude::*;
use std::io::{Read, Write};

/// A list of other types.
#[derive(Clone, Debug)]
pub struct Array<T>
{
    pub elements: Vec<T>,
}

impl<T> Array<T>
{
    pub fn read_with<F>(read: &mut Read, mut f: F) -> Result<Self, Error>
        where F: FnMut(&mut Read) -> Result<T, Error> {
        let length = VarInt::read(read)?;

        // FIXME: validate the length so we don't read too much data.

        let mut items = Vec::new();
        for _ in 0..length.0 {
            items.push(f(read)?);
        }

        Ok(Array { elements: items })
    }

    pub fn read_fixed_length(length: usize, read: &mut Read) -> Result<Self, Error>
        where T: ReadableType {
        let mut elements = Vec::new();
        for _ in (0..length).into_iter() {
            elements.push(T::read(read)?);
        }

        Ok(Array { elements: elements })
    }
}

impl<T: ReadableType> ReadableType for Array<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        Array::read_with(read, |read| T::read(read))
    }
}

impl<T: WritableType> WritableType for Array<T>
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

impl<T: Type> Type for Array<T> { }

