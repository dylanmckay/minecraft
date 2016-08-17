use game::{client, Client};
use protocol::packet::Statistics;

pub fn statistics(_client: &mut Client, packet: &Statistics)
    -> Result<(), client::handle::Error> {
    println!("statistics: {:?}", packet);
    Ok(())
}

