use super::*;

#[derive(Debug, Clone)]
pub struct ActionEffect {
    pub aim: ActionAim,
    pub effect: Effect,
}

#[derive(Debug, Clone)]
pub enum ActionAim {
    InRange { distance: Coord },
}

impl ActionAim {
    /// Check whether `target` satisfies the constraints.
    pub fn check(&self, world: &World, unit: UnitId, target: EffectTarget) -> SystemResult<bool> {
        match self {
            &ActionAim::InRange { distance } => {
                let target_pos = target.find_pos(world)?;
                let &caster_pos = world.units.grid_position.get(unit).expect("Unit not found");
                let delta = target_pos - caster_pos;
                Ok(crate::util::king_distance(delta) <= distance)
            }
        }
    }

    pub fn assist(
        &self,
        world: &World,
        unit: UnitId,
        target: EffectTarget,
    ) -> SystemResult<Option<EffectTarget>> {
        // If possible, move to the valid area
        let target = match self {
            &ActionAim::InRange { distance } => match target {
                EffectTarget::Unit(_) => target,
                EffectTarget::Position(target_pos) => {
                    let &caster_pos = world.units.grid_position.get(unit).expect("Unit not found");
                    let mut delta = (target_pos - caster_pos).map(|x| r32(x as f32));
                    let len = crate::util::king_distance(delta);
                    let distance = r32(distance as f32);
                    if len > distance {
                        delta *= distance / len;
                    }
                    let delta = delta.map(|x| x.as_f32().round() as Coord);
                    let target_pos = caster_pos + delta;
                    EffectTarget::Position(target_pos)
                }
            },
        };

        // Validate
        let target = self.check(world, unit, target)?.then_some(target);
        Ok(target)
    }
}

impl ActionEffect {
    pub fn into_effect(
        self,
        world: &World,
        unit: UnitId,
        input: ActionInput,
    ) -> SystemResult<(Effect, EffectContext)> {
        let target = self.aim.assist(world, unit, input.target)?;
        let context = EffectContext {
            caster: Some(Caster { unit }),
            target,
        };
        Ok((self.effect, context))
    }
}
