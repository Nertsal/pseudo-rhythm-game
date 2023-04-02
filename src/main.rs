use geng::prelude::*;

mod beat_controller;

use beat_controller::BeatController;

pub struct State {
    geng: Geng,
    beat_controller: BeatController,
    music_tick: usize,
}

impl State {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            beat_controller: BeatController::new(default()),
            music_tick: 0,
        }
    }

    fn music_tick(&mut self) {
        self.music_tick += 1;
        // TODO
        debug!("Tick");
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        let ticks = self.beat_controller.update(delta_time);
        for _ in 0..ticks {
            self.music_tick();
        }
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown { key: geng::Key::S } = event {
            self.beat_controller.player_beat();
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Pseudo-Rhythm Game".to_string(),
        ..default()
    });

    geng.clone().run(State::new(&geng))
}
