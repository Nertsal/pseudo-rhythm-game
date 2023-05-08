use std::collections::VecDeque;

use super::*;

mod particles;
mod projectiles;
mod units;
mod util;

pub struct Logic<'a> {
    pub world: &'a mut World,
    delta_time: Time,
    queued_effects: VecDeque<QueuedEffect>,
}

struct QueuedEffect {
    pub effect: Effect,
    pub context: EffectContext,
}

impl World {
    pub fn update(
        &mut self,
        player_action: Option<(PlayerAction, ActionInput)>,
        delta_time: Time,
    ) -> SystemResult<()> {
        let beat_time = Time::new(60.0 / self.beat_controller.get_bpm());
        self.player_beat_time += delta_time / beat_time;

        let mut logic = Logic {
            world: self,
            delta_time,
            queued_effects: default(),
        };
        logic.process(player_action)?;

        // Update music
        let ticks = self.beat_controller.update(delta_time.as_f32());
        self.music_controller
            .set_bpm(self.beat_controller.get_bpm());
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        // Play music
        let audio = self.geng.audio();
        for sound in self.music_controller.update(delta_time.as_f32()) {
            let sample_rate = rodio::Source::sample_rate(&sound) as f32;
            let data = sound.data().as_ref().to_owned();
            audio.from_raw(data, sample_rate).play();
        }

        Ok(())
    }
}

impl Logic<'_> {
    pub fn process(
        &mut self,
        player_action: Option<(PlayerAction, ActionInput)>,
    ) -> SystemResult<()> {
        if let Some((action, input)) = player_action {
            self.player_action(action, input)?;
        }

        self.process_projectiles_move()?;
        self.process_projectiles_collide()?;

        self.process_units_pos()?;
        self.process_units_ai()?;

        self.process_particles();

        self.process_effects()?;
        Ok(())
    }

    pub fn process_effects(&mut self) -> SystemResult<()> {
        while let Some(effect) = self.queued_effects.pop_front() {
            effect.effect.apply(self, effect.context)?;
        }

        Ok(())
    }

    pub fn player_action(&mut self, action: PlayerAction, input: ActionInput) -> SystemResult<()> {
        self.world.player_beat_time = Time::ZERO;
        let ticks = self.world.beat_controller.player_beat();
        for _ in 0..ticks {
            self.world.music_controller.tick();
        }

        // TODO: validate action
        self.unit_action(self.world.player.unit, action, input)?;

        // Synchronize units
        for (_, unit) in self.world.units.unit.iter_mut() {
            let Some(unit) = unit else {
                continue;
            };

            if let UnitBeat::Synchronized {
                player: player_beats,
                current_beat,
                ..
            } = &mut unit.beat
            {
                *current_beat = (*current_beat + 1) % *player_beats;
                if *current_beat % *player_beats == 0 {
                    unit.next_beat = Time::ZERO;
                }
            }
        }

        Ok(())
    }

    pub fn unit_action(
        &mut self,
        unit: UnitId,
        action: Action,
        input: ActionInput,
    ) -> SystemResult<()> {
        log::debug!("Unit {unit:?} executing action {action:?} with input {input:?}");
        match action {
            Action::Move(action) => self.unit_move(unit, action),
            Action::UseItem(action) => self.unit_use_item(unit, action, input),
        }
    }

    pub fn unit_move(&mut self, unit: UnitId, action: ActionMove) -> SystemResult<()> {
        let &pos = self
            .world
            .units
            .grid_position
            .get(unit)
            .expect("Unit not found");
        match action {
            ActionMove::Slide(slide) => self.unit_slide(unit, slide)?,
            ActionMove::Teleport(_tp) => {
                todo!()
            }
        }

        if Some(&pos) != self.world.units.grid_position.get(unit) {
            // Unit actually moved
            self.world.spawn_particles(pos, Color::BLUE)?;
        }

        Ok(())
    }

    pub fn unit_slide(&mut self, unit: UnitId, slide: MoveSlide) -> SystemResult<()> {
        if slide.delta.x.abs() > 1 || slide.delta.y.abs() > 1 {
            // TODO
            todo!("Only single-tile slide move implemented");
        }

        let &pos = self
            .world
            .units
            .grid_position
            .get(unit)
            .expect("Unit not found");

        let target = pos + slide.delta;

        let other = self
            .world
            .units
            .grid_position
            .iter()
            .find(|(_, &pos)| pos == target);
        if let Some((other, _)) = other {
            self.contact_damage(unit, other)?;
            return Ok(());
        }

        *self
            .world
            .units
            .grid_position
            .get_mut(unit)
            .expect("Unit not found") = target;
        Ok(())
    }

    pub fn contact_damage(&mut self, unit_a: UnitId, unit_b: UnitId) -> SystemResult<()> {
        // TODO: customize damage
        self.unit_damage(unit_a, Hp::new(1.0))?;
        self.unit_damage(unit_b, Hp::new(1.0))?;
        Ok(())
    }

    pub fn unit_damage(&mut self, unit: UnitId, damage: Hp) -> SystemResult<()> {
        let &pos = self
            .world
            .units
            .grid_position
            .get(unit)
            .expect("Unit not found");

        let health = self
            .world
            .units
            .health
            .get_mut(unit)
            .expect("Unit not found");
        health.damage(damage);
        if health.is_dead() {
            // Unit died
            // TODO: death effect
            self.world.units.remove(unit);
        }

        self.world.spawn_particles(pos, Color::WHITE)?;
        Ok(())
    }

    pub fn unit_use_item(
        &mut self,
        unit: UnitId,
        action: ActionUseItem,
        input: ActionInput,
    ) -> SystemResult<()> {
        let items = self
            .world
            .units
            .held_items
            .get(unit)
            .expect("Unit not found");
        let Some(item) = items.get_item(action.item) else {
            log::debug!("Tried using item from an empty hand");
            return Ok(());
        };

        let action = item.on_use.clone();
        let (effect, context) = action.into_effect(self.world, unit, input)?;
        effect.apply(self, context)?;

        Ok(())
    }
}
