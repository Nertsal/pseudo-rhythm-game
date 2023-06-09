use super::*;

#[derive(Debug, Clone)]
pub struct EffectContext {
    pub caster: Option<Caster>,
    pub target: Option<EffectTarget>,
}

#[derive(Debug, Clone)]
pub struct Caster {
    pub unit: UnitId,
    // pub item: Option<ItemId>,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectTarget {
    Unit(UnitId),
    Position(vec2<Coord>),
}

pub type ContextResult<T> = Result<T, ContextError>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ContextError {
    #[error("Caster expected but not found in effect context")]
    NoCaster,
    #[error("Target expected but not found in effect context")]
    NoTarget,
}

impl EffectTarget {
    pub fn expect_unit(self) -> ContextResult<UnitId> {
        match self {
            EffectTarget::Unit(unit) => Ok(unit),
            EffectTarget::Position(_) => Err(ContextError::NoTarget), // TODO: better error
        }
    }

    pub fn find_unit(self, world: &World) -> ContextResult<UnitId> {
        match self {
            EffectTarget::Unit(unit) => Ok(unit),
            EffectTarget::Position(target_pos) => world.get_unit_at(target_pos),
        }
    }

    pub fn find_pos(self, world: &World) -> ComponentResult<vec2<Coord>> {
        match self {
            EffectTarget::Unit(unit) => {
                Ok(*world.units.grid_position.get(unit).expect("Unit not found"))
            }
            EffectTarget::Position(pos) => Ok(pos),
        }
    }

    pub fn find_world_pos(self, world: &World) -> ComponentResult<vec2<FCoord>> {
        match self {
            EffectTarget::Unit(unit) => Ok(*world
                .units
                .world_position
                .get(unit)
                .expect("Unit not found")),
            EffectTarget::Position(pos) => Ok(world.grid.grid_to_world(pos)),
        }
    }
}

impl EffectContext {
    pub fn expect_caster(&self) -> ContextResult<Caster> {
        self.caster.clone().ok_or(ContextError::NoCaster)
    }

    pub fn expect_target(&self) -> ContextResult<EffectTarget> {
        self.target.ok_or(ContextError::NoTarget)
    }
}
