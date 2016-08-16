use io::{Type, Buffer, Error};
use io::types::VarInt;

/// A list of other types.
#[derive(Clone, Debug)]
pub struct Composite<T: Type>
{
    pub elements: Vec<T>,
}

impl<T: Type> Type for Composite<T>
{
    fn read(buffer: &mut Buffer) -> Result<Self, Error> {
        let length = VarInt::read(buffer)?;

        // FIXME: validate the length so we don't read too much data.

        let mut items = Vec::new();

        for _ in 0..length.0 {
            items.push(T::read(buffer)?);
        }

        Ok(Composite {
            elements: items,
        })
    }

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        let length = VarInt(self.elements.len() as _);
        length.write(buffer)?;

        for element in self.elements.iter() {
            element.write(buffer)?;
        }

        Ok(())
    }
}

