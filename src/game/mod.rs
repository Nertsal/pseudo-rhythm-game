use geng::{prelude::*, Camera2d};

use crate::{
    assets::Assets,
    sound::{MusicConfig, SectionName, Synthesizer},
    world::{ActionMove, MoveSlide, PlayerAction, World},
};

mod draw;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    world: World,
    grid: Grid,
    camera: Camera2d,
}

struct Grid {
    cell_size: vec2<f32>,
    offset: vec2<f32>,
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
            grid: Grid::default(),
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.draw(framebuffer)
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        let delta_time = crate::world::Time::new(delta_time);

        self.world.update(delta_time);
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown { key } = event {
            let delta = match key {
                geng::Key::W => Some(vec2(0, 1)),
                geng::Key::S => Some(vec2(0, -1)),
                geng::Key::A => Some(vec2(-1, 0)),
                geng::Key::D => Some(vec2(1, 0)),
                _ => None,
            };
            if let Some(delta) = delta {
                // TODO: handle error
                self.world
                    .player_action(PlayerAction::Move(ActionMove::Slide(MoveSlide { delta })))
                    .unwrap();
            }
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

impl Grid {
    pub fn matrix(&self) -> mat3<f32> {
        mat3::translate(self.offset) * mat3::scale(self.cell_size)
    }

    pub fn grid_to_world(&self, grid_pos: vec2<i64>) -> vec2<f32> {
        // self.offset + self.cell_size * grid_pos.map(|x| x as f32)
        let pos = self.matrix().inverse() * grid_pos.extend(1).map(|x| x as f32);
        pos.into_2d()
    }

    /// Returns the grid position and an in-cell offset from the cell pos to `world_pos`.
    pub fn world_to_grid(&self, world_pos: vec2<f32>) -> (vec2<i64>, vec2<f32>) {
        // (world_pos / self.cell_size).map(|x| x.floor() as i64)
        let grid_pos = self.matrix() * world_pos.extend(1.0);
        let mut offset = grid_pos.into_2d();
        let mut cell_pos = vec2(offset.x.trunc() as _, offset.y.trunc() as _);
        offset = vec2(offset.x.fract(), offset.y.fract());
        if offset.x < 0.0 {
            offset.x += 1.0;
            cell_pos.x -= 1;
        }
        if offset.y < 0.0 {
            offset.y += 1.0;
            cell_pos.y -= 1;
        }
        (cell_pos, offset)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cell_size: vec2(1.0, 1.0),
            offset: vec2::ZERO,
        }
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
