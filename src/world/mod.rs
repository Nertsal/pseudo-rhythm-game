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

        let mut world = Self {
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
        };
        world.init();
        world
    }

    fn init(&mut self) {
        let player = self.player.entity;
        self.entities.position.insert(player, vec2::ZERO).unwrap();
        self.entities
            .health
            .insert(player, Health::new(Hp::new(10.0)))
            .unwrap();

        let enemy = self.entities.spawn();
        self.entities.position.insert(enemy, vec2(2, 1)).unwrap();
        self.entities
            .health
            .insert(enemy, Health::new(Hp::new(2.0)))
            .unwrap();
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
        match move_action {
            ActionMove::Slide(slide) => self.entity_slide(entity, slide),
            ActionMove::Teleport(_tp) => {
                todo!()
            }
        }
    }

    fn entity_slide(&mut self, entity: EntityId, slide: MoveSlide) -> SystemResult<()> {
        if slide.delta.x.abs() + slide.delta.y.abs() != 1 {
            // TODO
            todo!("Only single-tile axis-aligned slide move implemented");
        }

        let &pos = self.entities.position.get(entity)?;

        let target = pos + slide.delta;

        let other = self
            .entities
            .position
            .iter()
            .find(|(_, &pos)| pos == target);
        if let Some((other, _)) = other {
            self.contact_damage(entity, other)?;
            return Ok(());
        }

        self.entities.position.update(entity, target)?;
        Ok(())
    }

    fn contact_damage(&mut self, entity_a: EntityId, entity_b: EntityId) -> SystemResult<()> {
        // TODO: customize damage
        self.entity_damage(entity_a, Hp::new(1.0))?;
        self.entity_damage(entity_b, Hp::new(1.0))?;
        Ok(())
    }

    fn entity_damage(&mut self, entity: EntityId, damage: Hp) -> SystemResult<()> {
        let health = self.entities.health.get_mut(entity)?;
        health.damage(damage);
        if health.is_dead() {
            // Entity died
            // TODO: death effect
            self.entities.remove(entity);
        }

        Ok(())
    }

    fn entity_use_item(&mut self, entity: EntityId, use_action: ActionUseItem) -> SystemResult<()> {
        todo!()
    }
}
