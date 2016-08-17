use game::{client, Client};
use protocol::packet::EntityStatus;

pub fn entity_status(_client: &mut Client, packet: &EntityStatus)
    -> Result<(), client::handle::Error> {
    println!("entity status: {:?}", packet);
    Ok(())
}
