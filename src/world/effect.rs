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
    Noop,
    Damage(Box<EffectDamage>),
    Particles(Box<EffectParticles>),
}

#[derive(Debug, Clone)]
pub struct EffectDamage {
    pub value: Hp,
}

#[derive(Debug, Clone)]
pub struct EffectParticles {
    pub pos: vec2<Coord>,
    pub color: Color,
}

impl Effect {
    pub fn apply(self, world: &mut World, context: EffectContext) -> SystemResult<()> {
        match self {
            Effect::Noop => Ok(()),
            Effect::Damage(effect) => effect.apply(world, context),
            Effect::Particles(effect) => effect.apply(world, context),
        }
    }
}

impl EffectDamage {
    pub fn apply(self, world: &mut World, context: EffectContext) -> SystemResult<()> {
        let target = context.expect_target()?;
        let Target::Entity(target) = target;

        // let &pos = world.entities.grid_position.get(target)?;
        world.entity_damage(target, self.value)?;
        // world.spawn_particles(pos, Color::WHITE)?;
        Ok(())
    }
}

impl EffectParticles {
    pub fn apply(self, world: &mut World, _context: EffectContext) -> SystemResult<()> {
        world.spawn_particles(self.pos, self.color)?;
        Ok(())
    }
}
