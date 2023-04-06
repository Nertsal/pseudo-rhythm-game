use super::*;

#[derive(Debug, Clone)]
pub enum ActionEffect {
    MeleeAttack { damage: Hp },
}

impl ActionEffect {
    pub fn into_effect(
        self,
        world: &World,
        entity: EntityId,
        input: ActionInput,
    ) -> SystemResult<(Effect, EffectContext)> {
        let mut context = EffectContext {
            caster: Some(Caster { entity }),
            target: None,
        };

        match self {
            ActionEffect::MeleeAttack { damage } => {
                let player = world.player.entity;
                let &player_pos = world.entities.grid_position.get(player)?;
                let delta = input.target_pos - player_pos;
                let target_pos = player_pos + crate::util::vec_to_dir(delta.map(|x| x as f32));

                let target = world
                    .entities
                    .grid_position
                    .iter()
                    .find(|(_, &pos)| pos == target_pos);
                if let Some((target, _)) = target {
                    let effect = EffectDamage { value: damage };
                    context.target = Some(Target::Entity(target));
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
