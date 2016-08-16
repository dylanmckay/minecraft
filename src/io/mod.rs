pub use self::types::Type;

pub mod types;

use std;

pub type Buffer = std::io::Cursor<Vec<u8>>;

