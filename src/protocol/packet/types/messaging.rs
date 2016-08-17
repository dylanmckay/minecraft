use protocol::types::*;

define_packet!(0x18 => PluginMessage; [
    channel: String,
    data: ByteArray
]);

