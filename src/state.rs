use super::*;

use beat_controller::BeatController;
use config::MusicConfig;
use music_controller::MusicController;

pub struct State {
    geng: Geng,
    assets: Rc<Assets>,
    beat_controller: BeatController,
    music_controller: MusicController,
}

impl State {
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        config: MusicConfig,
        synthesizers: HashMap<config::SectionName, rustysynth::Synthesizer>,
    ) -> Self {
        let beat_config = beat_controller::BeatControllerConfig {
            ticks_per_beat: config.ticks_per_beat,
            ..default()
        };
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            beat_controller: BeatController::new(beat_config),
            music_controller: MusicController::new(config, 50.0, synthesizers),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        let ticks = self.beat_controller.update(delta_time);
        self.music_controller
            .set_bpm(self.beat_controller.get_bpm());
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        for sound in self.music_controller.update(delta_time) {
            geng::SoundEffect::from_source(&self.geng, sound).play();
        }
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown { key: geng::Key::S } = event {
            let ticks = self.beat_controller.player_beat();
            for _ in 0..ticks {
                self.music_controller.tick();
            }
        }
    }

    fn ui<'a>(&mut self, _cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        geng::ui::stack![geng::ui::Text::new(
            format!("BPM: {:.0}", self.beat_controller.get_bpm()),
            self.geng.default_font().clone(),
            10.0,
            Rgba::WHITE
        )
        .fixed_size(vec2(100.0, 100.0))
        .align(vec2(0.0, 1.0))]
        .boxed()
    }
}

pub fn run(geng: &Geng) -> impl geng::State {
    let future = {
        let geng = geng.clone();
        async move {
            let assets: Rc<Assets> = geng::LoadAsset::load(&geng, &run_dir().join("assets"))
                .await
                .expect("Failed to load assets");

            let config: config::MusicConfig =
                geng::LoadAsset::load(&geng, &run_dir().join("assets").join("config.json"))
                    .await
                    .expect("Failed to load music config");

            let mut soundfonts = HashMap::new();
            for (sf_name, path) in &config.soundfonts {
                let bytes = file::load_bytes(run_dir().join("assets").join(path))
                    .await
                    .expect("Failed to load soundfont");
                let mut reader = std::io::BufReader::new(&bytes[..]);
                let soundfont =
                    rustysynth::SoundFont::new(&mut reader).expect("Failed to parse soundfont");
                soundfonts.insert(sf_name.to_owned(), Arc::new(soundfont));
            }

            let mut synthesizers = HashMap::new();
            let settings = rustysynth::SynthesizerSettings::new(44000);
            for (section_name, section) in &config.sections {
                let soundfont = soundfonts
                    .get(&section.soundfont.name)
                    .expect("Unknown soundfont");
                let synthesizer = rustysynth::Synthesizer::new(soundfont, &settings)
                    .expect("Failed to create a synthesizer");
                synthesizers.insert(section_name.to_owned(), synthesizer);
            }

            State::new(&geng, &assets, config, synthesizers)
        }
    };
    geng::LoadingScreen::new(geng, geng::EmptyLoadingScreen::new(geng), future)
}
