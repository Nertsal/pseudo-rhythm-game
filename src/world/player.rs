use super::*;

pub type PlayerAction = Action;

#[derive(Debug)]
pub struct Player {
    pub unit: UnitId,
}

impl Player {
    pub fn new(unit: UnitId) -> Self {
        Self { unit }
    }
}
