use super::*;

#[derive(Debug, Clone)]
pub struct EffectContext {
    pub caster: Option<EffectCaster>,
    pub target: Option<EffectTarget>,
}

#[derive(Debug, Clone)]
pub struct EffectCaster {
    pub entity: EntityId,
    // pub item: Option<ItemId>,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectTarget {
    Entity(EntityId),
    Position(vec2<Coord>),
}

pub type ContextResult<T> = Result<T, ContextError>;

#[derive(Debug, Clone)]
pub enum ContextError {
    NoCaster,
    NoTarget,
}

impl EffectTarget {
    pub fn expect_entity(self) -> ContextResult<EntityId> {
        match self {
            EffectTarget::Entity(entity) => Ok(entity),
            EffectTarget::Position(_) => Err(ContextError::NoTarget), // TODO: better error
        }
    }

    pub fn find_pos(self, world: &World) -> ComponentResult<vec2<Coord>> {
        match self {
            EffectTarget::Entity(entity) => world.entities.grid_position.get(entity).copied(),
            EffectTarget::Position(pos) => Ok(pos),
        }
    }
}

impl EffectContext {
    pub fn expect_caster(&self) -> ContextResult<EffectCaster> {
        self.caster.clone().ok_or(ContextError::NoCaster)
    }

    pub fn expect_target(&self) -> ContextResult<EffectTarget> {
        self.target.ok_or(ContextError::NoTarget)
    }
}

impl Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::NoCaster => write!(f, "Caster expected but not found in effect context"),
            ContextError::NoTarget => write!(f, "Target expected but not found in effect context"),
        }
    }
}
