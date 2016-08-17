use game::{client, Client};
use protocol::packet::PluginMessage;

pub fn plugin_message(_client: &mut Client, packet: &PluginMessage)
    -> Result<(), client::handle::Error> {
    println!("received plugin message on channel: '{}'", packet.channel);
    Ok(())
}

