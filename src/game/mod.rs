use geng::{prelude::*, Camera2d};

use crate::{
    assets::Assets,
    sound::{MusicConfig, SectionName, Synthesizer},
    world::*,
};

mod draw;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    world: World,
    camera: Camera2d,
    framebuffer_size: vec2<usize>,
    cursor_world_pos: vec2<f32>,
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
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
            framebuffer_size: vec2(1, 1),
            cursor_world_pos: vec2::ZERO,
        }
    }

    fn action(&mut self, action: PlayerAction) {
        let result = self.world.player_action(action, self.get_action_input());
        crate::util::report_err(result);
    }

    fn update_cursor(&mut self, cursor_pos: vec2<f64>) {
        self.cursor_world_pos = self.camera.screen_to_world(
            self.framebuffer_size.map(|x| x as f32),
            cursor_pos.map(|x| x as f32),
        );
    }

    fn get_action_input(&self) -> ActionInput {
        ActionInput {
            target_pos: self.world.grid.world_to_grid(self.cursor_world_pos).0,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        crate::util::report_err(self.draw(framebuffer));
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        let delta_time = crate::world::Time::new(delta_time);

        crate::util::report_err(self.world.update(delta_time));
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => {
                let delta = match key {
                    geng::Key::W => Some(vec2(0, 1)),
                    geng::Key::S => Some(vec2(0, -1)),
                    geng::Key::A => Some(vec2(-1, 0)),
                    geng::Key::D => Some(vec2(1, 0)),
                    _ => None,
                };
                if let Some(delta) = delta {
                    self.action(PlayerAction::Move(ActionMove::Slide(MoveSlide { delta })));
                }
            }
            geng::Event::MouseDown { button, .. } => {
                let item = match button {
                    geng::MouseButton::Left => ItemId::RightHand,
                    geng::MouseButton::Right => ItemId::LeftHand,
                    geng::MouseButton::Middle => return,
                };
                self.action(PlayerAction::UseItem(ActionUseItem { item }));
            }
            geng::Event::MouseMove { position, .. } => {
                self.update_cursor(position);
            }
            _ => (),
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
