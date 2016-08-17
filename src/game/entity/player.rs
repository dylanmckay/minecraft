use game::{Entity, EntityKind, EntityId};

#[derive(Clone,Debug)]
pub struct Player
{
    pub id: EntityId,
    pub username: String,
}

impl Entity for Player
{
    fn entity_id(&self) -> EntityId { self.id }
}

impl Into<EntityKind> for Player
{
    fn into(self) -> EntityKind { EntityKind::Player(self) }
}

