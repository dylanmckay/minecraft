use game::{client, Client};
use protocol::packet::PlayerAbilities;

pub fn player_abilities(_client: &mut Client, packet: &PlayerAbilities)
    -> Result<(), client::handle::Error> {
    println!("player abilities: {:?}", packet);
    Ok(())
}

