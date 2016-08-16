use io::{Buffer, Type, Error};
use io::types::VarInt;

impl<'a> Type for String
{
    fn read(buffer: &mut Buffer) -> Result<Self, Error> {
        use std::io::Read;

        let byte_count = VarInt::read(buffer)?.0;

        let mut str_bytes = Vec::new();
        buffer.take(byte_count as _).read_to_end(&mut str_bytes)?;

        // FIXME: this shouldn't be a panic
        assert_eq!(byte_count as usize, str_bytes.len());

        let str_bytes: Vec<u16> = str_bytes.chunks(2).map(|a| {
            ((a[0] as u16) << 8) | ((a[1] as u16) << 0)
        }).collect();

        match String::from_utf16(str_bytes.as_slice()) {
            Ok(val) => Ok(val),
            // FIXME: this shouldn't be a panic
            Err(_) => panic!("invalid utf16"),
        }
    }

    fn write(&self, buffer: &mut Buffer) -> Result<(), Error> {
        use std::io::Write;

        let byte_count = VarInt(self.chars().map(|c| c.len_utf16() as i32).sum());
        byte_count.write(buffer)?;

        for code_unit in self.encode_utf16() {
            let bytes = &[
                ((code_unit & 0xff00) >> 8) as u8 |
                ((code_unit & 0x00ff) >> 0) as u8
            ];

            buffer.write(bytes)?;
        }

        Ok(())
    }
}

