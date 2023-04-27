use geng::prelude::*;

#[derive(geng::Load)]
pub struct Assets {
    pub shaders: Shaders,
}

#[derive(geng::Load)]
pub struct Shaders {
    pub color: ugli::Program,
    pub grid: ugli::Program,
    pub arc: ugli::Program,
}
