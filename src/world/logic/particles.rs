use super::*;

impl World {
    pub fn spawn_particles(&mut self, pos: vec2<Coord>, color: Color) -> SystemResult<()> {
        let world_pos = (self.grid.grid_to_world(pos) + self.grid.cell_size / 2.0).map(FCoord::new);
        self.spawn_particles_world(world_pos, color)
    }

    pub fn spawn_particles_world(&mut self, pos: vec2<FCoord>, color: Color) -> SystemResult<()> {
        let amount = 5;
        for i in 0..amount {
            let speed = FCoord::new(1.5);

            let angle = i as f32 * f32::PI * 2.0 / amount as f32;
            let (sin, cos) = angle.sin_cos();
            let velocity = vec2(cos, sin).map(FCoord::new) * speed;

            let id = self.entities.spawn();
            self.entities.world_position.insert(id, pos).unwrap();
            self.entities.velocity.insert(id, velocity).unwrap();
            self.entities.color.insert(id, color).unwrap();
            self.entities
                .particle
                .insert(
                    id,
                    Particle {
                        lifetime: Health::new(Time::new(0.3)),
                        size: FCoord::new(0.3),
                    },
                )
                .unwrap();
        }

        Ok(())
    }
}

impl Logic<'_> {
    pub fn process_particles(&mut self) {
        let mut dead = Vec::new();
        for (id, particle) in self.world.entities.particle.iter_mut() {
            particle.lifetime.damage(self.delta_time);
            if particle.lifetime.is_dead() {
                dead.push(id);
                continue;
            }

            // particle.size = particle.lifetime.get_ratio();
        }

        for id in dead {
            self.world.entities.remove(id);
        }
    }
}
