use super::*;

pub struct Grid {
    pub cell_size: vec2<FCoord>,
    pub offset: vec2<FCoord>,
}

impl Grid {
    pub fn matrix(&self) -> mat3<FCoord> {
        mat3::translate(self.offset) * mat3::scale(self.cell_size)
    }

    pub fn grid_to_world(&self, grid_pos: vec2<Coord>) -> vec2<FCoord> {
        // self.offset + self.cell_size * grid_pos.map(|x| x as f32)
        let pos = self.matrix().inverse() * grid_pos.extend(1).map(|x| FCoord::new(x as f32));
        pos.into_2d()
    }

    /// Returns the grid position and an in-cell offset from the cell pos to `world_pos`.
    pub fn world_to_grid(&self, world_pos: vec2<FCoord>) -> (vec2<Coord>, vec2<FCoord>) {
        // (world_pos / self.cell_size).map(|x| x.floor() as i64)
        let grid_pos = self.matrix() * world_pos.extend(FCoord::ONE);
        let mut offset = grid_pos.into_2d() + vec2(0.5, 0.5).map(FCoord::new);
        let cell_pos = vec2(
            offset.x.floor().as_f32() as _,
            offset.y.floor().as_f32() as _,
        );
        offset = vec2(offset.x.fract(), offset.y.fract());
        (cell_pos, offset)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cell_size: vec2(FCoord::ONE, FCoord::ONE),
            offset: vec2::ZERO,
        }
    }
}
