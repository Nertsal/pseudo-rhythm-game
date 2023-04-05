use super::*;

pub type PlayerAction = Action;

#[derive(Debug)]
pub struct Player {
    pub entity: EntityId,
}

impl Player {
    pub fn new(entity_id: EntityId) -> Self {
        Self { entity: entity_id }
    }
}
