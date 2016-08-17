use std::io::{Read, Write};
use protocol::{self, types, packet, Packet, Error};
use game::client;

use mio::*;
use mio::tcp::TcpStream;

use std;

const CLIENT: Token = Token(1);
const TICK_TIMER: Token = Token(2);

const TICK_MS: u64 = 50;

pub struct Client
{
    pub state: client::State,

    server_stream: TcpStream,
    connection: protocol::Connection,
}

impl Client
{
    pub fn new(server_stream: TcpStream) -> Self {
        Client {
            state: client::State::initial(),
            server_stream: server_stream,
            connection: protocol::Connection::new(packet::Source::Server),
        }
    }

    pub fn connect<A: std::net::ToSocketAddrs>(addr: A) -> Self {
        let addr = addr.to_socket_addrs().unwrap().next().expect("socket address not working");

        let socket = TcpStream::connect(&addr).expect("error while connecting to tcp stream");
        Client::new(socket)
    }

    pub fn login(&mut self) {
        if let client::State::ProtoGame(client::ProtoGame::HandshakePending) = self.state {
        } else {
            panic!("too late to log in");
        }

        // FIXME: this code is commented out because it would sporadically give 'socket not
        // connected' errors.
        // let server_addr = self.server_stream.peer_addr().expect("no socket addresses found");
        // let (ip, port) = (format!("{}", server_addr.ip()), server_addr.port());
        let (ip, port) = ("192.168.1.1".to_owned(), 25565);

        self.send_packet(&packet::Handshake {
            protocol_version: types::VarInt(210),
            server_address: ip,
            server_port: port,
            next_state: packet::types::handshake::STATE_LOGIN,
        });

        self.state = client::State::ProtoGame(client::ProtoGame::PendingLoginStart);

        self.send_packet(&packet::LoginStart { username: "dylan".to_owned() });

        self.state = client::State::ProtoGame(client::ProtoGame::PendingLoginResponse);
    }

    pub fn tick(&mut self) {
        while let Some(result) = self.connection.take_packet(self.state.protocol_state()) {
            match result {
                Ok(ref packet) => self.handle_packet(packet),
                Err(e) => match e {
                    Error::UnknownPacket(ref data) => {
                        println!("unknown packet id: {:#x}", data.packet_id.0);
                    },
                    _ => {
                        panic!("unexpected error: {:#?}", e);
                    }
                },
            }
        }
    }

    pub fn run(&mut self) {
        // Create an event loop
        let mut event_loop = EventLoop::new().unwrap();

        // Register the socket
        event_loop.register(&self.server_stream, CLIENT, EventSet::readable(),
                            PollOpt::edge()).unwrap();

        event_loop.timeout_ms(TICK_TIMER, TICK_MS).expect("failed to set up tick timer");

        // Start handling events
        event_loop.run(self).unwrap();
    }

    fn send_packet<P: packet::Realization>(&mut self, packet: &P) {
        packet.write(&mut self.server_stream).expect("failed while writing packet");
        self.server_stream.flush().expect("error while flushing");
    }

    fn handle_packet(&mut self, packet: &Packet) {
        client::handle::packet(self, packet).unwrap()
    }
}

impl Handler for Client {
    type Timeout = Token;
    type Message = ();

    fn ready(&mut self, _event_loop: &mut EventLoop<Self>, token: Token, event_set: EventSet) {
        match token {
            CLIENT => {
                if event_set.is_readable() {
                    let mut buffer = [0u8; 10000];
                    let bytes_read = self.server_stream.read(&mut buffer).unwrap();

                    self.connection.give_bytes(&buffer[0..bytes_read]);
                }
            }
            _ => panic!("unexpected token"),
        }
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, _timeout: Self::Timeout) {
        self.tick();

        event_loop.timeout_ms(TICK_TIMER, TICK_MS).expect("failed to set up tick timer");
    }
}

