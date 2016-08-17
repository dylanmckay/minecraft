
/// The source of a packet.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Source
{
    /// The packet came from the server.
    Server,
    /// The packet came from the client.
    Client,
}

