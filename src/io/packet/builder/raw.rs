use io::{Buffer, Type};
use io::types::VarInt;
use std::collections::VecDeque;
use std::cmp;

/// The maximum number of bytes a varint can take up.
const MAXIMUM_VARINT_SIZE: usize = 5;

/// Builds raw packets.
pub struct Raw
{
    /// The buffer of unprocessed bytes.
    byte_queue: VecDeque<u8>,

    current_packet: Option<PartialPacket>,
    completed_packets: VecDeque<PartialPacket>,
}

/// A possibly partially-read packet.
#[derive(Clone, Debug)]
pub struct PartialPacket
{
    pub size: usize,
    pub retrieved_data: Vec<u8>,
}

impl Raw
{
    pub fn new() -> Self {
        Raw {
            byte_queue: VecDeque::new(),
            current_packet: None,
            completed_packets: VecDeque::new(),
        }
    }

    /// Gives bytes to the builder.
    pub fn give_bytes(&mut self, data: &[u8]) {
        self.byte_queue.extend(data.iter().cloned());
        self.process();
    }

    /// Takes a packet off of the queue.
    pub fn take_packet(&mut self) -> Option<PartialPacket> {
        self.completed_packets.pop_front()
    }

    fn process(&mut self) {
        loop {
            if self.byte_queue.is_empty() { return; }

            if self.current_packet.is_some() {
                self.process_current_packet()
            } else {
                if self.byte_queue.len() >= MAXIMUM_VARINT_SIZE {
                    let (size, bytes_read) = {
                        // shit as but it hopefully works
                        let tmp: Vec<_> = self.byte_queue.iter().cloned().collect();
                        let mut tmp_buffer = Buffer::new(tmp);

                        let size = VarInt::read(&mut tmp_buffer).unwrap();
                        let bytes_read = tmp_buffer.position();

                        (size.0, bytes_read)
                    };

                    // Eat the 'size' bytes
                    for _ in 0..bytes_read { self.byte_queue.pop_front(); }

                    self.current_packet = Some(PartialPacket {
                        size: size as _,
                        retrieved_data: Vec::new(),
                    });

                    self.process_current_packet();
                } else {
                    // Not enough data to start a new packet, we at
                    // least need the length.
                    break;
                }
            }
        }
    }

    fn process_current_packet(&mut self) {
        if self.byte_queue.is_empty() { return; }

        {
            let packet = if let Some(ref mut packet) = self.current_packet { packet } else { return };

            let remaining_bytes = packet.remaining_bytes();
            let bytes_to_add = cmp::min(remaining_bytes, self.byte_queue.len());

            let mut new_bytes = Vec::new();

            for _ in 0..bytes_to_add {
                new_bytes.push(self.byte_queue.pop_front().unwrap());
            }

            packet.retrieved_data.extend(new_bytes.into_iter());
        }

        if self.current_packet.as_ref().unwrap().is_complete() {
            self.completed_packets.push_back(self.current_packet.as_ref().unwrap().clone());
            self.current_packet = None;
        }
    }
}

impl PartialPacket
{
    pub fn remaining_bytes(&self) -> usize {
        self.size - self.retrieved_data.len()
    }

    pub fn is_complete(&self) -> bool {
        self.remaining_bytes() == 0
    }
}

