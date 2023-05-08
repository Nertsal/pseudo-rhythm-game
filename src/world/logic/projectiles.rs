use super::*;

impl Logic<'_> {
    pub fn process_projectiles_move(&mut self) -> SystemResult<()> {
        #[derive(StructQuery)]
        struct MoveItem<'a> {
            world_position: &'a mut vec2<FCoord>,
            velocity: &'a vec2<FCoord>,
        }

        let from_pos = self.world.projectiles.world_position.clone();

        let mut query = query_move_item!(self.world.projectiles);
        let mut iter = query.iter_mut();
        while let Some((_, item)) = iter.next() {
            *item.world_position += *item.velocity * self.delta_time;
        }

        // Update target
        #[derive(StructQuery)]
        struct Item<'a> {
            world_position: &'a vec2<FCoord>,
            target: &'a EffectTarget,
        }

        let mut dead = Vec::new();
        for (id, item) in &query_item!(self.world.projectiles) {
            let from = *from_pos.get(id).unwrap();
            let to = *item.world_position;
            let target = item.target.find_world_pos(self.world)?;
            let dist = crate::util::dist_to_segment(target, Segment(from, to));
            if dist.as_f32() < 0.1 {
                // Reached target
                dead.push(id);
            }
        }

        dead.sort();
        for id in dead.into_iter().rev() {
            let proj = self.world.projectiles.remove(id).unwrap();
            self.world
                .spawn_particles_world(proj.world_position, Rgba::opaque(0.4, 0.4, 0.4))?;
        }

        Ok(())
    }

    pub fn process_projectiles_collide(&mut self) -> SystemResult<()> {
        #[derive(StructQuery)]
        struct Proj<'a> {
            world_position: &'a vec2<FCoord>,
            caster: &'a Option<Caster>,
            fraction: &'a Fraction,
            target_filter: &'a FractionFilter,
            on_contact: &'a Effect,
        }

        #[derive(StructQuery)]
        struct Unit<'a> {
            world_position: &'a vec2<FCoord>,
            fraction: &'a Fraction,
        }

        let mut hits = Vec::new();
        for (proj_id, proj) in &query_proj!(self.world.projectiles) {
            let query = query_unit!(self.world.units);
            let target = query
                .iter()
                .filter(|(_, unit)| proj.target_filter.check(*proj.fraction, *unit.fraction))
                .find(|(_, unit)| {
                    let delta = *unit.world_position - *proj.world_position;
                    let dist = delta.len();
                    dist.as_f32() < 0.5
                });
            if let Some((unit_id, _)) = target {
                self.queued_effects.push_back(QueuedEffect {
                    effect: proj.on_contact.clone(),
                    context: EffectContext {
                        caster: proj.caster.clone(),
                        target: Some(EffectTarget::Unit(unit_id)),
                    },
                });
                hits.push(proj_id);
            }
        }
        for id in hits {
            self.world
                .projectiles
                .remove(id)
                .expect("Tried to remove a nonexistent projectile");
        }

        Ok(())
    }
}
