use super::*;

#[derive(Debug, Clone)]
pub struct Caster {
    pub entity: EntityId,
    // pub item: Option<ItemId>,
}

#[derive(Debug, Clone)]
pub enum Target {
    Entity(EntityId),
}

#[derive(Debug, Clone)]
pub enum EffectError {}

#[derive(Debug, Clone)]
pub enum Effect {
    Damage(Box<EffectDamage>),
    Noop,
}

#[derive(Debug, Clone)]
pub struct EffectDamage {
    pub value: Hp,
}

impl Effect {
    pub fn apply(self, world: &mut World, context: EffectContext) -> SystemResult<()> {
        match self {
            Effect::Damage(effect) => effect.apply(world, context),
            Effect::Noop => Ok(()),
        }
    }
}

impl EffectDamage {
    pub fn apply(self, world: &mut World, context: EffectContext) -> SystemResult<()> {
        let target = context.expect_target()?;
        let Target::Entity(target) = target;
        world.entity_damage(target, self.value)
    }
}
