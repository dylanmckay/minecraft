use protocol::types::*;

define_packet!(0x01 => EncryptionRequest; [
    server_id: String,
    public_key: ByteArray,
    verify_token: ByteArray
]);

define_packet!(0x01 => EncryptionResponse; [
    shared_secret: ByteArray,
    verify_token: ByteArray
]);

