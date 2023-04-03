use geng::prelude::*;

use crate::{
    assets::Assets,
    sound::{MusicConfig, SectionName, Synthesizer},
    world::World,
};

pub struct Game {
    pub geng: Geng,
    pub assets: Rc<Assets>,
    pub world: World,
}

impl Game {
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        music_config: MusicConfig,
        synthesizers: HashMap<SectionName, Synthesizer>,
    ) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: World::new(geng, music_config, synthesizers),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        let delta_time = crate::world::Time::new(delta_time);

        self.world.update(delta_time);
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown { key: geng::Key::S } = event {
            self.world.player_beat();
        }
    }

    fn ui<'a>(&mut self, _cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        geng::ui::stack![geng::ui::Text::new(
            format!("BPM: {:.0}", self.world.beat_controller.get_bpm()),
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

            let config: MusicConfig =
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

            Game::new(&geng, &assets, config, synthesizers)
        }
    };
    geng::LoadingScreen::new(geng, geng::EmptyLoadingScreen::new(geng), future)
}
