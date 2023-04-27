use super::*;

mod particles;
mod util;

struct Logic<'a> {
    world: &'a mut World,
    delta_time: Time,
}

impl Logic<'_> {
    pub fn process(&mut self) -> SystemResult<()> {
        self.process_projectiles()?;

        self.process_units_pos()?;
        self.process_units_ai()?;

        self.process_particles();
        Ok(())
    }

    fn process_projectiles(&mut self) -> SystemResult<()> {
        #[derive(StructQuery)]
        struct Item<'a> {
            world_position: &'a mut vec2<FCoord>,
            velocity: &'a vec2<FCoord>,
        }

        let mut query = query_item!(self.world.projectiles);
        let mut iter = query.iter_mut();
        while let Some((_, item)) = iter.next() {
            *item.world_position += *item.velocity * self.delta_time;
        }

        Ok(())
    }

    fn process_units_pos(&mut self) -> SystemResult<()> {
        #[derive(StructQuery)]
        struct Item<'a> {
            grid_position: &'a vec2<Coord>,
            world_position: &'a mut vec2<FCoord>,
        }

        let mut query = query_item!(self.world.units);
        let mut iter = query.iter_mut();
        while let Some((_, item)) = iter.next() {
            // TODO: interpolate
            *item.world_position = self.world.grid.grid_to_world(*item.grid_position);
        }

        Ok(())
    }

    fn process_units_ai(&mut self) -> SystemResult<()> {
        let mut actions = Vec::new();
        for (id, unit) in self.world.units.unit.iter_mut() {
            let Some(unit) = unit else {
                continue;
            };

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
            let unit = self
                .world
                .units
                .unit
                .get(id)
                .expect("Unit not found")
                .as_ref()
                .expect("Unit AI not found");
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
        let audio = self.geng.audio();
        for sound in self.music_controller.update(delta_time.as_f32()) {
            let sample_rate = rodio::Source::sample_rate(&sound) as f32;
            let data = sound.data().as_ref().to_owned();
            audio.from_raw(data, sample_rate).play();
        }

        Ok(())
    }

    fn unit_action(
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

    fn unit_move(&mut self, unit: UnitId, action: ActionMove) -> SystemResult<()> {
        let &pos = self.units.grid_position.get(unit).expect("Unit not found");
        match action {
            ActionMove::Slide(slide) => self.unit_slide(unit, slide)?,
            ActionMove::Teleport(_tp) => {
                todo!()
            }
        }

        if Some(&pos) != self.units.grid_position.get(unit) {
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

        let &pos = self.units.grid_position.get(unit).expect("Unit not found");

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

        *self
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
        let &pos = self.units.grid_position.get(unit).expect("Unit not found");

        let health = self.units.health.get_mut(unit).expect("Unit not found");
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
        let items = self.units.held_items.get(unit).expect("Unit not found");
        let Some(item) = items.get_item(action.item) else {
            log::debug!("Tried using item from an empty hand");
            return Ok(());
        };

        let action = item.on_use.clone();
        let (effect, context) = action.into_effect(self, unit, input)?;
        effect.apply(self, context)?;

        Ok(())
    }
}
