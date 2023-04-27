use super::*;

impl World {
    pub fn spawn_particles(&mut self, pos: vec2<Coord>, color: Color) -> SystemResult<()> {
        let world_pos = self.grid.grid_to_world(pos);
        self.spawn_particles_world(world_pos, color)
    }

    pub fn spawn_particles_world(
        &mut self,
        position: vec2<FCoord>,
        color: Color,
    ) -> SystemResult<()> {
        let amount = 5;
        for i in 0..amount {
            let speed = FCoord::new(1.5);

            let angle = i as f32 * f32::PI * 2.0 / amount as f32;
            let (sin, cos) = angle.sin_cos();
            let velocity = vec2(cos, sin).map(FCoord::new) * speed;

            let particle = Particle {
                position,
                velocity,
                lifetime: Health::new(Time::new(0.3)),
                size: FCoord::new(0.3),
                color,
            };
            self.particles.insert(particle);
        }

        Ok(())
    }
}

impl Logic<'_> {
    pub fn process_particles(&mut self) {
        #[derive(StructQuery)]
        struct Item<'a> {
            lifetime: &'a mut Health,
            position: &'a mut vec2<FCoord>,
            velocity: &'a vec2<FCoord>,
        }

        let mut dead = Vec::new();
        let mut query = query_item!(self.world.particles);
        let mut iter = query.iter_mut();
        while let Some((id, particle)) = iter.next() {
            particle.lifetime.damage(self.delta_time);
            if particle.lifetime.is_dead() {
                dead.push(id);
                continue;
            }

            // particle.size = particle.lifetime.get_ratio();
            *particle.position += *particle.velocity * self.delta_time;
        }

        for id in dead.into_iter().rev() {
            self.world.particles.remove(id);
        }
    }
}
