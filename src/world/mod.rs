use geng::prelude::*;

use crate::{
    id::{Id, IdGenerator},
    sound::{BeatConfig, BeatController, MusicConfig, MusicController, SectionName, Synthesizer},
};

pub struct World {
    geng: Geng,
    id_gen: IdGenerator,
    pub entities: Entities,
    items: Collection<Item>,
    pub beat_controller: BeatController,
    pub music_controller: MusicController,
}

#[derive(Default)]
pub struct Entities {
    ids: HashSet<EntityId>,
}

pub type Time = R32;
pub type EntityId = Id;
pub type ItemId = Id;
pub type Coord = i64;

#[derive(HasId)]
pub struct Item {
    pub id: ItemId,
    pub on_use: Effect,
}

pub enum Effect {
    Noop,
}

pub enum Action {
    Move(ActionMove),
    UseItem(ActionUseItem),
}

pub struct ActionUseItem {
    pub item: ItemId,
}

pub enum ActionMove {
    Slide(MoveSlide),
    Teleport(MoveTeleport),
}

pub struct MoveSlide {
    pub delta: vec2<Coord>,
}

pub struct MoveTeleport {
    pub target: vec2<Coord>,
}

impl World {
    pub fn new(
        geng: &Geng,
        music_config: MusicConfig,
        synthesizers: HashMap<SectionName, Synthesizer>,
    ) -> Self {
        let beat_config = BeatConfig {
            ticks_per_beat: music_config.ticks_per_beat,
            ..default()
        };
        Self {
            geng: geng.clone(),
            id_gen: IdGenerator::new(),
            entities: Entities::new(),
            items: Collection::new(),
            music_controller: MusicController::new(
                music_config,
                beat_config.bpm_min as f32,
                synthesizers,
            ),
            beat_controller: BeatController::new(beat_config),
        }
    }

    pub fn player_beat(&mut self) {
        let ticks = self.beat_controller.player_beat();
        for _ in 0..ticks {
            self.music_controller.tick();
        }
    }

    pub fn update(&mut self, delta_time: Time) {
        let ticks = self.beat_controller.update(delta_time.as_f32());
        self.music_controller
            .set_bpm(self.beat_controller.get_bpm());
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        for sound in self.music_controller.update(delta_time.as_f32()) {
            geng::SoundEffect::from_source(&self.geng, sound).play();
        }
    }

    fn entity_action(&self, entity: EntityId) -> Action {
        todo!()
    }

    fn move_entity(&mut self, entity: EntityId, move_action: ActionMove) {
        todo!()
    }
}

impl Entities {
    fn new() -> Self {
        Self::default()
    }
}
