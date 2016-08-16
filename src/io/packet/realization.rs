use io::Error;

// TODO: come up with better name
pub trait Realization : Clone
{
    fn parse(data: Vec<u8>) -> Result<Self, Error>;
}

