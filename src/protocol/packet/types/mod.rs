pub use self::handshake::Handshake;
pub use self::login_start::LoginStart;
pub use self::login_success::LoginSuccess;
pub use self::encryption_request::EncryptionRequest;
pub use self::encryption_response::EncryptionResponse;
pub use self::set_compression::SetCompression;

macro_rules! define_packet {
    ($id:expr => $name:ident; [
         $( $field:ident: $ty:ident),+ ]) => {
        #[derive(Clone, Debug)]
        pub struct $name
        {
            $(
                pub $field: $ty,
            )*
        }

        impl ::protocol::packet::Realization for $name
        {
            const PACKET_ID: ::protocol::types::VarInt = VarInt($id);
            const DESCRIPTION: &'static str = stringify!($name);

            fn parse(read: &mut ::std::io::Read)
                -> Result<Self, ::protocol::Error> {
                use ::protocol::types::*;

                Ok($name {
                    $(
                        $field: $ty::read(read)?,
                    )*
                })
            }

            fn write_payload(&self, write: &mut ::std::io::Write)
                -> Result<(), ::protocol::Error> {

                $( self.$field.write(write)?; )*

                Ok(())
            }
        }
    }
}

pub mod handshake;
pub mod login_start;
pub mod login_success;
pub mod encryption_request;
pub mod encryption_response;
pub mod set_compression;

