use protocol::types::*;

define_packet!(0x23 => JoinGame; [
    entity_id: i32,
    game_mode: u8,
    dimension: i32,
    difficulty: u8,
    max_players: u8,
    level_type: String,
    reduced_debug_info: bool
]);

