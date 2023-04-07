use geng::prelude::*;

#[derive(geng::Assets)]
pub struct Assets {
    pub shaders: Shaders,
}

#[derive(geng::Assets)]
pub struct Shaders {
    pub color: ugli::Program,
    pub grid: ugli::Program,
    pub arc: ugli::Program,
}
