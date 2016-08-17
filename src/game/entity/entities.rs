use game::EntityKind;

/// A collection of entities.
#[derive(Debug)]
pub struct Entities
{
    entities: Vec<EntityKind>,
}

impl Entities
{
    pub fn new() -> Self {
        Entities {
            entities: Vec::new(),
        }
    }

    pub fn add<E>(&mut self, entity: E) where E: Into<EntityKind> {
        self.entities.push(entity.into());
    }
}

