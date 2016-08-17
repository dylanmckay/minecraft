pub use self::handshake::{Handshake, Statistics};
pub use self::login::{LoginStart, LoginSuccess};
pub use self::encryption::{EncryptionRequest, EncryptionResponse};
pub use self::compression::SetCompression;
pub use self::management::{JoinGame, ServerDifficulty, SpawnPosition};
pub use self::messaging::PluginMessage;
pub use self::player::{PlayerAbilities, HeldItemChange};
pub use self::entity::EntityStatus;

macro_rules! define_packet {
    ($id:expr => $name:ident; [
         $( $field:ident: $ty:ty),+ ]) => {
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
                #[allow(unused_imports)]
                use ::protocol::types::*;

                Ok($name {
                    $(
                        $field: ::protocol::Type::read(read)?,
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
pub mod login;
pub mod encryption;
pub mod compression;
pub mod management;
pub mod messaging;
pub mod player;
pub mod entity;

