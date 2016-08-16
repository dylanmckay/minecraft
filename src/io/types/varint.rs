use io::{Type, Buffer, Error};
use std::io;

/// A variable sized int.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct VarInt(pub i32);

impl VarInt
{
    pub fn required_bytes(&self) -> usize {
        use integer_encoding::VarInt;
        self.0.required_space()
    }
}

impl Type for VarInt
{
    fn read(buffer: &mut Buffer) -> Result<Self, Error> {
        use integer_encoding::VarInt;
        use std::io::Seek;

        let(i, byte_count) = i32::decode_var_vec(buffer.get_mut());
        buffer.seek(io::SeekFrom::Current(byte_count as _))?;

        Ok(VarInt(i))
    }

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        use std::io::Write;
        use integer_encoding::VarInt;

        let bytes = self.0.encode_var_vec();
        buffer.write(&bytes)?;

        Ok(())
    }
}

