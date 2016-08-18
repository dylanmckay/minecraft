use game::{client, Client};
use protocol::packet::PlayerListItem;

pub fn player_list_item(_client: &mut Client, packet: &PlayerListItem)
    -> Result<(), client::handle::Error> {
    println!("player list item: {:?}", packet);
    Ok(())
}

