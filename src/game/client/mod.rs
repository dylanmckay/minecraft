pub use self::client::Client;
pub use self::state::State;
pub use self::proto_game::ProtoGame;

pub mod client;
pub mod handle;
pub mod state;
pub mod proto_game;

pub fn run() {
    let mut client = Client::connect("127.0.0.1:25565");
    client.login();
    client.run();
}
