use geng::prelude::*;

use crate::{
    id::{Id, IdGenerator},
    sound::{BeatConfig, BeatController, MusicConfig, MusicController, SectionName, Synthesizer},
};

pub struct World {
    pub geng: Geng,
    pub player: Player,
    pub entities: Entities,
    pub items: Collection<Item>,
    pub beat_controller: BeatController,
    pub music_controller: MusicController,
}

#[derive(Debug)]
pub struct Player {
    pub entity: EntityId,
}

pub type PlayerAction = Action;

#[derive(Default)]
pub struct Entities {
    id_gen: IdGenerator,
    ids: HashSet<EntityId>,
    pub positions: HashMap<EntityId, vec2<Coord>>,
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

#[derive(Debug, Clone)]
pub enum Action {
    Move(ActionMove),
    UseItem(ActionUseItem),
}

#[derive(Debug, Clone)]
pub struct ActionUseItem {
    pub item: ItemId,
}

#[derive(Debug, Clone)]
pub enum ActionMove {
    Slide(MoveSlide),
    Teleport(MoveTeleport),
}

#[derive(Debug, Clone)]
pub struct MoveSlide {
    pub delta: vec2<Coord>,
}

#[derive(Debug, Clone)]
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
        let mut entities = Entities::new();
        let player = entities.spawn();
        entities.positions.insert(player, vec2::ZERO);
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

    pub fn player_action(&mut self, action: PlayerAction) {
        let ticks = self.beat_controller.player_beat();
        for _ in 0..ticks {
            self.music_controller.tick();
        }

        // TODO: validate action
        self.entity_action(self.player.entity, action);
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

    fn entity_action(&mut self, entity: EntityId, action: Action) {
        if !self.entities.ids.contains(&entity) {
            panic!("Unexistent entity tried to do an action: {entity:?} - {action:?}");
        }

        match action {
            Action::Move(action) => self.entity_move(entity, action),
            Action::UseItem(action) => self.entity_use_item(entity, action),
        }
    }

    fn entity_move(&mut self, entity: EntityId, move_action: ActionMove) {
        let pos = self.entities.positions.get_mut(&entity).unwrap_or_else(|| {
            panic!("Tried to move entity without position component: {entity:?} - {move_action:?}")
        });
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
    }

    fn entity_use_item(&mut self, entity: EntityId, use_action: ActionUseItem) {
        todo!()
    }
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> EntityId {
        let id = self.id_gen.next();
        self.ids.insert(id);
        id
    }
}

impl Player {
    pub fn new(entity_id: EntityId) -> Self {
        Self { entity: entity_id }
    }
}
