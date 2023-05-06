use super::*;

impl Logic<'_> {
    pub fn process_units_pos(&mut self) -> SystemResult<()> {
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

    pub fn process_units_ai(&mut self) -> SystemResult<()> {
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
