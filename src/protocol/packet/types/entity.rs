use protocol::types::*;

define_packet!(0x1B => EntityStatus; [
    entity_id: i32,
    entity_status: u8
]);

