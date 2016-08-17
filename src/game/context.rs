use game::Entities;

pub struct Context
{
    pub entities: Entities,
}

impl Context
{
    pub fn new() -> Self {
        Context {
            entities: Entities::new(),
        }
    }
}

