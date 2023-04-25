use super::*;

#[derive(Debug, Clone)]
pub enum Effect {
    Noop,
    If(Box<EffectIf>),
    Damage(Box<EffectDamage>),
    Particles(Box<EffectParticles>),
}

#[derive(Debug, Clone)]
pub struct EffectIf {
    pub condition: Condition,
    pub then: Effect,
    pub otherwise: Effect,
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
        debug!("Applying effect {self:?} with context {context:?}");
        match self {
            Effect::Noop => Ok(()),
            Effect::If(effect) => effect.apply(world, context),
            Effect::Damage(effect) => effect.apply(world, context),
            Effect::Particles(effect) => effect.apply(world, context),
        }
    }
}

impl EffectIf {
    pub fn apply(self, world: &mut World, context: EffectContext) -> SystemResult<()> {
        let condition = self.condition.evaluate(world, &context)?;
        if condition {
            self.then.apply(world, context)
        } else {
            self.otherwise.apply(world, context)
        }
    }
}

impl EffectDamage {
    pub fn apply(self, world: &mut World, context: EffectContext) -> SystemResult<()> {
        let target = context.expect_target()?;
        match target.find_unit(world) {
            Ok(unit) => {
                world.unit_damage(unit, self.value)?;
            }
            Err(_) => {
                let pos = target.find_pos(world)?;
                world.spawn_particles(pos, Color::WHITE)?;
            }
        }
        Ok(())
    }
}

impl EffectParticles {
    pub fn apply(self, world: &mut World, _context: EffectContext) -> SystemResult<()> {
        world.spawn_particles(self.pos, self.color)?;
        Ok(())
    }
}
