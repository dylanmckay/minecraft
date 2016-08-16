pub use self::client::Client;

pub mod client;
pub mod handle;

pub fn run() {
    let mut client = Client::connect("127.0.0.1:25565");
    client.login();
    client.run();
}
