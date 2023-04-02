use geng::prelude::*;

mod beat_controller;
mod config;
mod music_controller;
mod sound_queue;
mod source;
mod state;
mod synthesize;

#[derive(geng::Assets)]
pub struct Assets {}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Pseudo-Rhythm Game".to_string(),
        ..default()
    });

    geng.clone().run(state::run(&geng))
}
