use super::*;

mod particles;

struct Logic<'a> {
    world: &'a mut World,
    delta_time: Time,
}

impl Logic<'_> {
    pub fn process(&mut self) -> SystemResult<()> {
        self.movement()?;
        self.process_particles();
        Ok(())
    }

    fn movement(&mut self) -> SystemResult<()> {
        for (entity, &velocity) in self.world.entities.velocity.iter() {
            let position = self.world.entities.world_position.get_mut(entity)?;
            *position += velocity * self.delta_time;
        }
        Ok(())
    }
}

impl World {
    pub fn player_action(&mut self, action: PlayerAction, input: ActionInput) -> SystemResult<()> {
        let ticks = self.beat_controller.player_beat();
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        // TODO: validate action
        self.entity_action(self.player.entity, action, input)
    }

    pub fn update(&mut self, delta_time: Time) -> SystemResult<()> {
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
            geng::SoundEffect::from_source(&self.geng, sound).play();
        }

        Ok(())
    }

    fn entity_action(
        &mut self,
        entity: EntityId,
        action: Action,
        input: ActionInput,
    ) -> SystemResult<()> {
        debug!("Entity {entity:?} executing action {action:?} with input {input:?}");
        // self.entities.ids.get(entity)?;

        match action {
            Action::Move(action) => self.entity_move(entity, action),
            Action::UseItem(action) => self.entity_use_item(entity, action, input),
        }
    }

    fn entity_move(&mut self, entity: EntityId, action: ActionMove) -> SystemResult<()> {
        let &pos = self.entities.grid_position.get(entity)?;
        match action {
            ActionMove::Slide(slide) => self.entity_slide(entity, slide)?,
            ActionMove::Teleport(_tp) => {
                todo!()
            }
        }
        let world_pos = (self.grid.grid_to_world(pos) + self.grid.cell_size / 2.0).map(FCoord::new);
        self.spawn_particles(world_pos)?;
        Ok(())
    }

    fn entity_slide(&mut self, entity: EntityId, slide: MoveSlide) -> SystemResult<()> {
        if slide.delta.x.abs() + slide.delta.y.abs() != 1 {
            // TODO
            todo!("Only single-tile axis-aligned slide move implemented");
        }

        let &pos = self.entities.grid_position.get(entity)?;

        let target = pos + slide.delta;

        let other = self
            .entities
            .grid_position
            .iter()
            .find(|(_, &pos)| pos == target);
        if let Some((other, _)) = other {
            self.contact_damage(entity, other)?;
            return Ok(());
        }

        self.entities.grid_position.update(entity, target)?;
        Ok(())
    }

    pub fn contact_damage(&mut self, entity_a: EntityId, entity_b: EntityId) -> SystemResult<()> {
        // TODO: customize damage
        self.entity_damage(entity_a, Hp::new(1.0))?;
        self.entity_damage(entity_b, Hp::new(1.0))?;
        Ok(())
    }

    pub fn entity_damage(&mut self, entity: EntityId, damage: Hp) -> SystemResult<()> {
        let health = self.entities.health.get_mut(entity)?;
        health.damage(damage);
        if health.is_dead() {
            // Entity died
            // TODO: death effect
            self.entities.remove(entity);
        }

        Ok(())
    }

    fn entity_use_item(
        &mut self,
        entity: EntityId,
        action: ActionUseItem,
        input: ActionInput,
    ) -> SystemResult<()> {
        let items = self.entities.held_items.get(entity)?;
        let Some(item) = items.get_item(action.item) else {
            debug!("Tried using item from an empty hand");
            return Ok(());
        };

        let action = item.on_use.clone();
        let (effect, context) = action.to_effect(self, entity, input)?;
        effect.apply(self, context)?;

        Ok(())
    }
}
