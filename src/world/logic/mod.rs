use super::*;

mod particles;

struct Logic<'a> {
    world: &'a mut World,
    delta_time: Time,
}

impl Logic<'_> {
    pub fn process(&mut self) -> SystemResult<()> {
        self.process_units()?;
        self.process_particles();
        Ok(())
    }

    fn process_units(&mut self) -> SystemResult<()> {
        let mut actions = Vec::new();
        for (id, unit) in self.world.units.unit.iter_mut() {
            let bpm = unit.beat.calc_bpm(self.world.beat_controller.get_bpm());
            let beat_time = Time::new(60.0 / bpm);

            if let UnitBeat::Synchronized { .. } = unit.beat {
                if self.world.player_beat_time >= Time::ONE {
                    // Wait for player beat
                    continue;
                }
            }

            unit.next_beat -= self.delta_time / beat_time;
            if unit.next_beat < Time::ZERO {
                actions.push(id);
                unit.next_beat += Time::ONE;
            }
        }

        for id in actions {
            let unit = self.world.units.unit.get(id)?;
            if let Some((action, input)) = unit.behaviour.evaluate(self.world, id)? {
                self.world.unit_action(id, action, input)?;
            }
        }

        Ok(())
    }
}

impl World {
    pub fn player_action(&mut self, action: PlayerAction, input: ActionInput) -> SystemResult<()> {
        self.player_beat_time = Time::ZERO;
        let ticks = self.beat_controller.player_beat();
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        // TODO: validate action
        self.unit_action(self.player.unit, action, input)?;

        // Synchronize units
        for (_, unit) in self.units.unit.iter_mut() {
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

    pub fn update(&mut self, delta_time: Time) -> SystemResult<()> {
        let beat_time = Time::new(60.0 / self.beat_controller.get_bpm());
        self.player_beat_time += delta_time / beat_time;

        let mut logic = Logic {
            world: self,
            delta_time,
        };
        logic.process()?;

        // Update music
        let ticks = self.beat_controller.update(delta_time.as_f32());
        self.music_controller
            .set_bpm(self.beat_controller.get_bpm());
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        // Play music
        for sound in self.music_controller.update(delta_time.as_f32()) {
            geng::SoundEffect::from_source(&self.geng, &sound).play();
        }

        Ok(())
    }

    fn unit_action(
        &mut self,
        unit: UnitId,
        action: Action,
        input: ActionInput,
    ) -> SystemResult<()> {
        debug!("Unit {unit:?} executing action {action:?} with input {input:?}");
        match action {
            Action::Move(action) => self.unit_move(unit, action),
            Action::UseItem(action) => self.unit_use_item(unit, action, input),
        }
    }

    fn unit_move(&mut self, unit: UnitId, action: ActionMove) -> SystemResult<()> {
        let &pos = self.units.grid_position.get(unit)?;
        match action {
            ActionMove::Slide(slide) => self.unit_slide(unit, slide)?,
            ActionMove::Teleport(_tp) => {
                todo!()
            }
        }

        if Ok(&pos) != self.units.grid_position.get(unit) {
            // Unit actually moved
            self.spawn_particles(pos, Color::BLUE)?;
        }

        Ok(())
    }

    fn unit_slide(&mut self, unit: UnitId, slide: MoveSlide) -> SystemResult<()> {
        if slide.delta.x.abs() > 1 || slide.delta.y.abs() > 1 {
            // TODO
            todo!("Only single-tile slide move implemented");
        }

        let &pos = self.units.grid_position.get(unit)?;

        let target = pos + slide.delta;

        let other = self
            .units
            .grid_position
            .iter()
            .find(|(_, &pos)| pos == target);
        if let Some((other, _)) = other {
            self.contact_damage(unit, other)?;
            return Ok(());
        }

        self.units.grid_position.update(unit, target)?;
        Ok(())
    }

    pub fn contact_damage(&mut self, unit_a: UnitId, unit_b: UnitId) -> SystemResult<()> {
        // TODO: customize damage
        self.unit_damage(unit_a, Hp::new(1.0))?;
        self.unit_damage(unit_b, Hp::new(1.0))?;
        Ok(())
    }

    pub fn unit_damage(&mut self, unit: UnitId, damage: Hp) -> SystemResult<()> {
        let &pos = self.units.grid_position.get(unit)?;

        let health = self.units.health.get_mut(unit)?;
        health.damage(damage);
        if health.is_dead() {
            // Unit died
            // TODO: death effect
            self.units.remove(unit);
        }

        self.spawn_particles(pos, Color::WHITE)?;
        Ok(())
    }

    fn unit_use_item(
        &mut self,
        unit: UnitId,
        action: ActionUseItem,
        input: ActionInput,
    ) -> SystemResult<()> {
        let items = self.units.held_items.get(unit)?;
        let Some(item) = items.get_item(action.item) else {
            debug!("Tried using item from an empty hand");
            return Ok(());
        };

        let action = item.on_use.clone();
        let (effect, context) = action.into_effect(self, unit, input)?;
        effect.apply(self, context)?;

        Ok(())
    }
}
