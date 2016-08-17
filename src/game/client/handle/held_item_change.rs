use game::{client, Client};
use protocol::packet::HeldItemChange;

pub fn held_item_change(_client: &mut Client, packet: &HeldItemChange)
    -> Result<(), client::handle::Error> {
    println!("held item change: {:?}", packet);
    Ok(())
}
