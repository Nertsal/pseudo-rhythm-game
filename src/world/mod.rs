use geng::prelude::*;

use crate::{
    id::{Id, IdGenerator},
    sound::{BeatConfig, BeatController, MusicConfig, MusicController, SectionName, Synthesizer},
};

mod action;
mod component;
mod entity;
mod health;
mod player;

pub use action::*;
pub use component::*;
pub use entity::*;
pub use health::*;
pub use player::*;

pub type Time = R32;
pub type ItemId = Id;
pub type Coord = i64;

pub struct World {
    pub geng: Geng,
    pub player: Player,
    pub entities: Entities,
    pub items: Collection<Item>,
    pub beat_controller: BeatController,
    pub music_controller: MusicController,
}

#[derive(HasId)]
pub struct Item {
    pub id: ItemId,
    pub on_use: Effect,
}

pub enum Effect {
    Noop,
}

pub type SystemResult<T> = Result<T, SystemError>;

#[derive(Debug, Clone)]
pub enum SystemError {
    Component(ComponentError),
}

impl From<ComponentError> for SystemError {
    fn from(value: ComponentError) -> Self {
        Self::Component(value)
    }
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
        let mut entities = Entities::new();

        let player = entities.spawn();
        entities.position.insert(player, vec2::ZERO).unwrap();

        Self {
            geng: geng.clone(),
            player: Player::new(player),
            entities,
            items: Collection::new(),
            music_controller: MusicController::new(
                music_config,
                beat_config.bpm_min as f32,
                synthesizers,
            ),
            beat_controller: BeatController::new(beat_config),
        }
    }

    pub fn player_action(&mut self, action: PlayerAction) -> SystemResult<()> {
        let ticks = self.beat_controller.player_beat();
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        // TODO: validate action
        self.entity_action(self.player.entity, action)
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

    fn entity_action(&mut self, entity: EntityId, action: Action) -> SystemResult<()> {
        // self.entities.ids.get(entity)?;

        match action {
            Action::Move(action) => self.entity_move(entity, action),
            Action::UseItem(action) => self.entity_use_item(entity, action),
        }
    }

    fn entity_move(&mut self, entity: EntityId, move_action: ActionMove) -> SystemResult<()> {
        let pos = self.entities.position.get_mut(entity)?;
        match move_action {
            ActionMove::Slide(slide) => {
                // TODO: check collision
                *pos += slide.delta;
            }
            ActionMove::Teleport(tp) => {
                // TODO: check collision
                *pos = tp.target;
            }
        }
        Ok(())
    }

    fn entity_use_item(&mut self, entity: EntityId, use_action: ActionUseItem) -> SystemResult<()> {
        todo!()
    }
}
