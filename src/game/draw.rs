use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) -> SystemResult<()> {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);

        let screen_vs = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| draw_2d::Vertex { a_pos: vec2(x, y) })
            .collect();
        let screen_vs = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), screen_vs);

        // Grid
        ugli::draw(
            framebuffer,
            &self.assets.shaders.grid,
            ugli::DrawMode::TriangleFan,
            &screen_vs,
            (
                ugli::uniforms! {
                    u_grid_matrix: self.world.grid.matrix(),
                },
                geng::camera2d_uniforms(&self.camera, framebuffer_size),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );

        let radius = self.world.grid.cell_size.x.min(self.world.grid.cell_size.y) / 2.0;
        for (id, &pos) in self.world.entities.grid_position.iter() {
            let pos = self.world.grid.grid_to_world(pos) + self.world.grid.cell_size / 2.0;
            let color = if id == self.world.player.entity {
                Rgba::GREEN
            } else {
                Rgba::RED
            };
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Ellipse::circle(pos, radius, color),
            );
        }

        self.draw_particles(framebuffer)?;

        Ok(())
    }

    fn draw_particles(&self, framebuffer: &mut ugli::Framebuffer) -> SystemResult<()> {
        for (id, particle) in self.world.entities.particle.iter() {
            let pos = self
                .world
                .entities
                .world_position
                .get(id)?
                .map(FCoord::as_f32);
            let &color = self.world.entities.color.get(id)?;
            let radius = particle.size.as_f32() / 2.0 * particle.lifetime.get_ratio().as_f32();

            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Ellipse::circle(pos, radius, color),
            );
        }

        Ok(())
    }
}
