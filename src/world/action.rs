use super::*;

#[derive(Debug, Clone)]
pub enum Action {
    Move(ActionMove),
    UseItem(ActionUseItem),
}

#[derive(Debug, Clone)]
pub struct ActionUseItem {
    pub item: ItemId,
}

#[derive(Debug, Clone)]
pub enum ActionMove {
    Slide(MoveSlide),
    Teleport(MoveTeleport),
}

#[derive(Debug, Clone)]
pub struct MoveSlide {
    pub delta: vec2<Coord>,
}

#[derive(Debug, Clone)]
pub struct MoveTeleport {
    pub target: vec2<Coord>,
}
