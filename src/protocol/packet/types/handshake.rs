use protocol::types::*;

pub const STATE_STATUS: VarInt = VarInt(1);
pub const STATE_LOGIN: VarInt = VarInt(2);

define_packet!(0x00 => Handshake; [
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt
]);

define_composite_type!(Statistic => [
   name: String,
   value: String
]);

define_packet!(0x07 => Statistics; [
    statistics: Array<Statistic>
]);

