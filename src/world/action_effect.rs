use super::*;

#[derive(Debug, Clone)]
pub enum ActionEffect {
    MeleeAttack { damage: Hp },
    // RangeAttack { projectile: Projectile },
}

impl ActionEffect {
    pub fn aim_assist(&self, world: &World, unit: UnitId, input: ActionInput) -> EffectTarget {
        match self {
            ActionEffect::MeleeAttack { .. } => {
                // Clamp target to be in range
                match input.target {
                    target @ EffectTarget::Unit(_) => target,
                    EffectTarget::Position(target_pos) => {
                        let &caster_pos =
                            world.units.grid_position.get(unit).expect("Unit not found");
                        let delta = target_pos - caster_pos;
                        let target_pos =
                            caster_pos + crate::util::vec_to_dir(delta.map(|x| x as f32));
                        EffectTarget::Position(target_pos)
                    }
                }
            }
        }
    }

    pub fn into_effect(
        self,
        world: &World,
        unit: UnitId,
        input: ActionInput,
    ) -> SystemResult<(Effect, EffectContext)> {
        let mut context = EffectContext {
            caster: Some(EffectCaster { unit }),
            target: None,
        };

        let target = self.aim_assist(world, unit, input);

        let (target_id, target_pos) = match target {
            EffectTarget::Unit(unit) => (
                Some(unit),
                *world.units.grid_position.get(unit).expect("Unit not found"),
            ),
            EffectTarget::Position(pos) => (world.get_unit_at(pos).ok(), pos),
        };

        match self {
            ActionEffect::MeleeAttack { damage } => {
                if let Some(target) = target_id {
                    let effect = EffectDamage { value: damage };
                    context.target = Some(EffectTarget::Unit(target));
                    return Ok((Effect::Damage(Box::new(effect)), context));
                }

                // Miss effect
                let effect = EffectParticles {
                    pos: target_pos,
                    color: Color::GRAY,
                };
                Ok((Effect::Particles(Box::new(effect)), context))
            } // ActionEffect::RangeAttack { projectile } => {}
        }
    }
}
