use protocol::types::*;

define_packet!(0x03 => SetCompression; [
    threshold: VarInt
]);

