use io::{Type, Error};
use io::types::VarInt;
use std::io::{Read, Write};

/// A list of other types.
#[derive(Clone, Debug)]
pub struct Composite<T: Type>
{
    pub elements: Vec<T>,
}

impl<T: Type> Type for Composite<T>
{
    fn read(read: &mut Read) -> Result<Self, Error> {
        let length = VarInt::read(read)?;

        // FIXME: validate the length so we don't read too much data.

        let mut items = Vec::new();

        for _ in 0..length.0 {
            items.push(T::read(read)?);
        }

        Ok(Composite {
            elements: items,
        })
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        let length = VarInt(self.elements.len() as _);
        length.write(write)?;

        for element in self.elements.iter() {
            element.write(write)?;
        }

        Ok(())
    }
}

