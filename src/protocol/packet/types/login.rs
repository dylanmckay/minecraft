use protocol::types::*;

define_packet!(0x00 => LoginStart; [
    username: String
]);

define_packet!(0x02 => LoginSuccess; [
    uuid: String,
    username: String
]);

