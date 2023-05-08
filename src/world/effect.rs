use super::*;

#[derive(Debug, Clone)]
pub enum Effect {
    Noop,
    If(Box<EffectIf>),
    Damage(Box<EffectDamage>),
    Projectile(Box<EffectProjectile>),
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
pub struct EffectProjectile {
    pub projectile: ProjectilePrefab,
    pub speed: FCoord,
}

#[derive(Debug, Clone)]
pub struct EffectParticles {
    pub pos: vec2<Coord>,
    pub color: Color,
}

impl Effect {
    pub fn apply(self, logic: &mut Logic<'_>, context: EffectContext) -> SystemResult<()> {
        log::debug!("Applying effect {self:?} with context {context:?}");
        match self {
            Effect::Noop => Ok(()),
            Effect::If(effect) => effect.apply(logic, context),
            Effect::Damage(effect) => effect.apply(logic, context),
            Effect::Projectile(effect) => effect.apply(logic, context),
            Effect::Particles(effect) => effect.apply(logic, context),
        }
    }
}

impl EffectIf {
    pub fn apply(self, logic: &mut Logic<'_>, context: EffectContext) -> SystemResult<()> {
        let condition = self.condition.evaluate(logic.world, &context)?;
        if condition {
            self.then.apply(logic, context)
        } else {
            self.otherwise.apply(logic, context)
        }
    }
}

impl EffectDamage {
    pub fn apply(self, logic: &mut Logic<'_>, context: EffectContext) -> SystemResult<()> {
        let target = context.expect_target()?;
        match target.find_unit(logic.world) {
            Ok(unit) => {
                logic.unit_damage(unit, self.value)?;
            }
            Err(_) => {
                let pos = target.find_pos(logic.world)?;
                logic.world.spawn_particles(pos, Color::WHITE)?;
            }
        }
        Ok(())
    }
}

impl EffectProjectile {
    pub fn apply(self, logic: &mut Logic<'_>, context: EffectContext) -> SystemResult<()> {
        let caster = context.expect_caster()?;
        // let &grid_position = world
        //     .units
        //     .grid_position
        //     .get(caster.unit)
        //     .expect("Unit not found");
        let &world_position = logic
            .world
            .units
            .world_position
            .get(caster.unit)
            .expect("Unit not found");
        let &fraction = logic
            .world
            .units
            .fraction
            .get(caster.unit)
            .expect("Unit not found");

        let target = context.expect_target()?;
        let target_pos = target.find_world_pos(logic.world)?;

        let delta = target_pos - world_position;
        let dir = delta.normalize_or_zero();
        let velocity = dir * self.speed;

        let inst = ProjectileInst {
            // grid_position,
            world_position,
            velocity,
            caster: context.caster,
            fraction,
        };
        let projectile = self.projectile.instantiate(inst);
        logic.world.projectiles.insert(projectile);
        Ok(())
    }
}

impl EffectParticles {
    pub fn apply(self, logic: &mut Logic<'_>, _context: EffectContext) -> SystemResult<()> {
        logic.world.spawn_particles(self.pos, self.color)?;
        Ok(())
    }
}
