pub use self::types::Type;
pub use self::error::Error;

pub mod types;
pub mod error;
pub mod packet;

pub type Buffer = ::std::io::Cursor<Vec<u8>>;

