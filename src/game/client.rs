use game::State;
use mio::*;
use mio::tcp::TcpStream;
use io::mgr::PacketBuilder;
use std::io::{Read, Write};
use io::packet;
use io::types::*;
use io;
use io::packet::Realization;

const INITIAL_STATE: State = State::Handshake;
const CLIENT: Token = Token(1);


pub struct Client
{
    pub current_state: State,
}

impl Client
{
    pub fn new() -> Self {
        Client {
            current_state: INITIAL_STATE,
        }
    }
}

pub struct PacketHandler
{
    socket: TcpStream,
    packet_builder: PacketBuilder,
}

impl PacketHandler
{
    pub fn new(socket: TcpStream) -> Self {
        PacketHandler {
            socket: socket,
            packet_builder: PacketBuilder::new(),
        }
    }
}

impl Handler for PacketHandler {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, _event_loop: &mut EventLoop<Self>, token: Token, event_set: EventSet) {
        match token {
            CLIENT => {
                println!("event: {:?}", event_set);
                if event_set.is_readable() {
                    println!("readable");
                    let mut buffer = [0u8; 512];
                    self.socket.read(&mut buffer).unwrap();

                    println!("read: {:?}", buffer.as_ref());

                    self.packet_builder.give(&buffer);

                    for packet in self.packet_builder.consume_packets() {
                        println!("received packet: {:#?}", packet);
                    }
                }
            }
            _ => panic!("unexpected token"),
        }
    }
}

pub fn run() {
    let addr = "127.0.0.1:25565".parse().expect("error while parsing ip");

    // Create an event loop
    // let mut event_loop = EventLoop::new().unwrap();

    // Setup the client socket
    let mut sock = TcpStream::connect(&addr).expect("error while connecting to tcp stream");

    // Register the socket
    // event_loop.register(&sock, CLIENT, EventSet::readable(),
    //                     PollOpt::edge()).unwrap();

    let handshake = packet::Handshake {
        protocol_version: VarInt(210),
        server_address: "dylanmckay.io".to_owned(),
        server_port: 25565,
        next_state: packet::types::handshake::STATE_LOGIN,
    };

    let login_start = packet::LoginStart { username: "dylan".to_owned() };

    println!("sending handshake");

    {
        let mut buffer = io::Buffer::new(Vec::new());
        handshake.write(&mut buffer).expect("failed while writing handshake");

        for b in buffer.get_ref().iter() {
            println!("sent: {} - {:8b}", *b as char, b);
        }

        let bytes_written = sock.write(&buffer.get_ref()).unwrap();
        println!("wrote {} bytes", bytes_written);
    }

    println!("sending login start");

    {
        let mut buffer = io::Buffer::new(Vec::new());
        login_start.write(&mut buffer).expect("failed while writing login start");

        for b in buffer.get_ref().iter() {
            println!("sent: {} - {:8b}", *b as char, b);
        }

        let bytes_written = sock.write(&buffer.get_ref()).unwrap();
        println!("wrote {} bytes", bytes_written);
    }

    sock.flush().expect("error while flushing");

    for b in sock.bytes() {
        if let Ok(a) = b {
            println!("{} - {:b}", a as char, a);
        }
    }

    // let mut handler = PacketHandler::new(sock);

    // Start handling events
    // event_loop.run(&mut handler).unwrap();
}

