use game::State;
use std::io::{Read, Write};
use io::{types, packet, PacketBuilder};

use mio::*;
use mio::tcp::TcpStream;

use io;
use std;

const INITIAL_STATE: State = State::Handshake;
const CLIENT: Token = Token(1);
const TICK_TIMER: Token = Token(2);

const TICK_MS: u64 = 50;

pub struct Client
{
    pub current_state: State,

    server_stream: TcpStream,
    packet_builder: PacketBuilder,
}

impl Client
{
    pub fn new(server_stream: TcpStream) -> Self {
        Client {
            current_state: INITIAL_STATE,
            server_stream: server_stream,
            packet_builder: PacketBuilder::new(),
        }
    }

    pub fn connect<A: std::net::ToSocketAddrs>(addr: A) -> Self {
        let addr = addr.to_socket_addrs().unwrap().next().expect("socket address not working");

        let socket = TcpStream::connect(&addr).expect("error while connecting to tcp stream");
        Client::new(socket)
    }

    pub fn login(&mut self) {
        let server_addr = self.server_stream.peer_addr().expect("no socket addresses found");
        let (ip, port) = (format!("{}", server_addr.ip()), server_addr.port());

        self.send_packet(&packet::Handshake {
            protocol_version: types::VarInt(210),
            server_address: ip,
            server_port: port,
            next_state: packet::types::handshake::STATE_LOGIN,
        });

        self.send_packet(&packet::LoginStart { username: "dylan".to_owned() });
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

    pub fn send_packet<P: packet::Realization>(&mut self, packet: &P) {
        let mut buffer = io::Buffer::new(Vec::new());
        packet.write(&mut buffer).expect("failed while writing packet");
        self.server_stream.write(&buffer.get_ref()).unwrap();

        self.server_stream.flush().expect("error while flushing");
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

                    self.packet_builder.give(&buffer[0..bytes_read]);

                    for packet in self.packet_builder.consume_packets() {
                        println!("received packet: {:#?}", packet);
                    }
                }
            }
            _ => panic!("unexpected token"),
        }
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, _timeout: Self::Timeout) {
        println!("timeout");
        event_loop.timeout_ms(TICK_TIMER, TICK_MS).expect("failed to set up tick timer");
    }
}

pub fn run() {
    let mut client = Client::connect("127.0.0.1:25565");
    client.login();
    client.run();
}

