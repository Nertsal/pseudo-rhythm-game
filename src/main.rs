#![allow(dead_code)] // TODO: remove

use geng::prelude::*;

mod assets;
mod collection;
mod game;
mod sound;
mod util;
mod world;

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Pseudo-Rhythm Game".to_string(),
        ..default()
    });

    geng.clone().run(game::run(&geng))
}
