use super::*;

impl World {
    pub fn get_unit_at(&self, target_pos: vec2<Coord>) -> ContextResult<UnitId> {
        let target = self
            .units
            .grid_position
            .iter()
            .find(|(_, &pos)| pos == target_pos);
        target.map(|(id, _)| id).ok_or(ContextError::NoTarget) // TODO: better error
    }
}
