use game::{client, Client};
use protocol::packet::ChunkData;

pub fn chunk_data(_client: &mut Client, packet: &ChunkData)
    -> Result<(), client::handle::Error> {
    println!("received chunk data");
    Ok(())
}

