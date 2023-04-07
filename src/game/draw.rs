use super::*;

const HOVERED_COLOR: Rgba<f32> = Rgba {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const CLAMPED_COLOR: Rgba<f32> = Rgba {
    r: 0.2,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

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

        // Hovered cells
        let &player_pos = self.world.units.grid_position.get(self.world.player.unit)?;
        let hovered = self.world.grid.world_to_grid(self.cursor_world_pos).0;
        let delta = hovered - player_pos;
        let clamped = player_pos + crate::util::vec_to_dir(delta.map(|x| x as f32));
        for (mesh, color) in [(hovered, HOVERED_COLOR), (clamped, CLAMPED_COLOR)]
            .into_iter()
            .map(|(pos, color)| (cell_mesh(pos, &self.world.grid), color))
        {
            let mesh = mesh
                .into_iter()
                .map(|a_pos| draw_2d::Vertex { a_pos })
                .collect();
            let geometry = ugli::VertexBuffer::new_dynamic(self.geng.ugli(), mesh);
            ugli::draw(
                framebuffer,
                &self.assets.shaders.color,
                ugli::DrawMode::Triangles,
                &geometry,
                (
                    ugli::uniforms! {
                        u_model_matrix: mat3::identity(),
                        u_color: color,
                    },
                    geng::camera2d_uniforms(&self.camera, framebuffer_size),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            )
        }

        // Units
        let radius = 0.9 * self.world.grid.cell_size.x.min(self.world.grid.cell_size.y) / 2.0;
        for (id, &pos) in self.world.units.grid_position.iter() {
            let pos = self.world.grid.grid_to_world(pos) + self.world.grid.cell_size / 2.0;
            let color = if id == self.world.player.unit {
                Rgba::BLUE
            } else {
                let fraction = self.world.units.fraction.get(id)?;
                match fraction {
                    Fraction::Player => Rgba::GREEN,
                    Fraction::Enemy => Rgba::RED,
                }
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
        for particle in self.world.particles.iter() {
            let pos = particle.position.map(FCoord::as_f32);
            let color = particle.color;
            let t = particle.lifetime.get_ratio().as_f32();
            let t = crate::util::smooth_step(t);
            let radius = particle.size.as_f32() / 2.0 * t;

            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Ellipse::circle(pos, radius, color),
            );
        }

        Ok(())
    }
}

fn cell_mesh(pos: vec2<Coord>, grid: &Grid) -> Vec<vec2<f32>> {
    let pos = grid.grid_to_world(pos);
    let size = grid.cell_size;
    vec![
        pos,
        pos + vec2(size.x, 0.0),
        pos + vec2(0.0, size.y),
        pos + vec2(size.x, 0.0),
        pos + size,
        pos + vec2(0.0, size.y),
    ]
}
