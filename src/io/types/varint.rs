use io::{Type, Buffer};
use std::io;

/// A variable sized int.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct VarInt(pub i32);

impl Type for VarInt
{
    fn read(buffer: &mut Buffer) -> io::Result<Self> {
        use integer_encoding::VarInt;
        use std::io::Seek;

        let(i, byte_count) = i32::decode_var_vec(buffer.get_mut());
        buffer.seek(io::SeekFrom::Current(byte_count as _))?;

        Ok(VarInt(i))
    }

    fn write(&self, buffer: &mut Buffer) -> io::Result<()> {
        use std::io::Write;
        use integer_encoding::VarInt;

        let bytes = self.0.encode_var_vec();
        buffer.write(&bytes)?;

        Ok(())
    }
}

