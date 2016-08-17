pub use self::player::Player;

pub mod player;

pub trait Entity
{
    fn entity_id(&self) -> ::game::EntityId;
}

