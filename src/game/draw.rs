use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
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
                    u_grid_matrix: self.grid.matrix(),
                },
                geng::camera2d_uniforms(&self.camera, framebuffer_size),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );

        let radius = self.grid.cell_size.x.min(self.grid.cell_size.y) / 2.0;
        for (&id, &pos) in &self.world.entities.position {
            let pos = self.grid.grid_to_world(pos);
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
    }
}
