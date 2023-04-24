use super::*;

#[derive(Debug, Clone)]
pub enum ActionEffect {
    MeleeAttack { damage: Hp },
}

impl ActionEffect {
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

        match self {
            ActionEffect::MeleeAttack { damage } => {
                let player = world.player.unit;

                let (target, target_pos) = match input.target {
                    EffectTarget::Unit(unit) => {
                        // Check validity
                        (
                            Some(unit),
                            *world.units.grid_position.get(unit).expect("Unit not found"),
                        )
                    }
                    EffectTarget::Position(target_pos) => {
                        let &player_pos = world
                            .units
                            .grid_position
                            .get(player)
                            .expect("Unit not found");
                        let delta = target_pos - player_pos;
                        let target_pos =
                            player_pos + crate::util::vec_to_dir(delta.map(|x| x as f32));

                        let target = world
                            .units
                            .grid_position
                            .iter()
                            .find(|(_, &pos)| pos == target_pos);

                        (target.map(|(id, _)| id), target_pos)
                    }
                };

                if let Some(target) = target {
                    let effect = EffectDamage { value: damage };
                    context.target = Some(EffectTarget::Unit(target));
                    return Ok((Effect::Damage(Box::new(effect)), context));
                }

                let effect = EffectParticles {
                    pos: target_pos,
                    color: Color::GRAY,
                };
                Ok((Effect::Particles(Box::new(effect)), context))
            }
        }
    }
}
