pub use self::player::Player;
pub use self::entities::Entities;

pub mod player;
pub mod entities;

pub trait Entity : ::std::fmt::Debug + Into<EntityKind>
{
    fn entity_id(&self) -> ::game::EntityId;
}

// TODO: better name
#[derive(Debug)]
pub enum EntityKind
{
    Player(Player),
}

impl Entity for EntityKind
{
    fn entity_id(&self) -> ::game::EntityId {
        match *self {
            EntityKind::Player(ref e) => e.entity_id(),
        }
    }
}

