use geng::prelude::*;

pub struct State {
    geng: Geng,
}

impl State {
    pub fn new(geng: &Geng) -> Self {
        Self { geng: geng.clone() }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Pseudo-Rhytmic Game".to_string(),
        ..default()
    });

    geng.clone().run(State::new(&geng))
}
