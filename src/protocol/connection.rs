use protocol::{Packet, Error, GameState};
use protocol::packet::{self, builder};

#[derive(Clone,Debug)]
pub struct Connection
{
    pub state: State,
    pub builder: builder::Cooked,
}

#[derive(Clone,Debug)]
pub struct State
{
    pub source: packet::Source,
    pub compression: Compression,
    pub encryption: Encryption,
}

#[derive(Clone,Debug)]
pub enum Encryption
{
    Disabled,
}

#[derive(Clone,Debug)]
pub enum Compression
{
    Disabled,
    Enabled {
        /// The minimum size for a packet to be compressed.
        threshold: usize,
    },
}

impl Connection
{
    pub fn new(source: packet::Source) -> Self {
        Connection {
            state: State::initial(source),
            builder: builder::Cooked::new(),
        }
    }

    pub fn take_packet(&mut self, game_state: GameState)
        -> Option<Result<Packet, Error>> {
        let result = self.builder.take_packet(&self.state, game_state);

        if let Some(Ok(packet)) = result {
            self.state.preprocess_packet(&packet);

            Some(Ok(packet))
        } else {
            result
        }
    }

    pub fn give_bytes(&mut self, bytes: &[u8]) {
        self.builder.give_bytes(bytes);
    }
}

impl State
{
    pub fn initial(source: packet::Source) -> Self {
        State {
            source: source,
            compression: Compression::Disabled,
            encryption: Encryption::Disabled,
        }
    }

    /// Preprocess a packet before we retrieve it. Some packets
    /// change the way we parse packets (i.e. setting compression).
    fn preprocess_packet(&mut self, packet: &Packet) {
        match *packet {
            Packet::SetCompression(ref packet) => {
                self.compression = if packet.threshold.0 >= 0 {
                    Compression::Enabled { threshold: packet.threshold.0 as _ }
                } else {
                    Compression::Disabled
                };
            },
            _ => {
                // We don't care about any other packets.
            }
        }
    }
}

impl Compression
{
    pub fn is_enabled(&self) -> bool {
        if let Compression::Enabled { .. } = *self { true } else { false }
    }
}

