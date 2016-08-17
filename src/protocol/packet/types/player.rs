use protocol::types::*;

define_packet!(0x2B => PlayerAbilities; [
    flags: u8,
    flying_speed: f32,
    field_of_view_modifier: f32
]);

