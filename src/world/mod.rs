use geng::prelude::*;

use crate::{
    id::{Id, IdGenerator},
    sound::{
        BeatConfig, BeatController, MusicConfig, MusicController, SectionName, Synthesizer, Ticks,
    },
};

mod action;
mod action_effect;
mod component;
mod context;
mod effect;
mod entity;
mod grid;
mod health;
mod item;
mod logic;
mod player;
mod target;
mod unit;

pub use action::*;
pub use action_effect::*;
pub use component::*;
pub use context::*;
pub use effect::*;
pub use entity::*;
pub use grid::*;
pub use health::*;
pub use item::*;
pub use logic::*;
pub use player::*;
pub use target::*;
pub use unit::*;

pub type Time = R32;
pub type Coord = i64;
pub type FCoord = R32;
pub type Color = Rgba<f32>;

pub struct World {
    pub geng: Geng,
    pub entities: Entities,
    pub grid: Grid,
    pub player: Player,
    pub beat_controller: BeatController,
    pub music_controller: MusicController,
}

pub type SystemResult<T> = Result<T, SystemError>;

#[derive(Debug, Clone)]
pub enum SystemError {
    Component(ComponentError),
    Context(ContextError),
    Behaviour(BehaviourError),
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub lifetime: Health,
    /// Diameter.
    pub size: FCoord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fraction {
    Player,
    Enemy,
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
            entities,
            player: Player::new(player),
            grid: Grid::default(),
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
        self.entities
            .grid_position
            .insert(player, vec2::ZERO)
            .unwrap();
        self.entities
            .health
            .insert(player, Health::new(Hp::new(10.0)))
            .unwrap();
        self.entities
            .fraction
            .insert(player, Fraction::Player)
            .unwrap();
        self.entities
            .held_items
            .insert(
                player,
                HeldItems {
                    left_hand: None,
                    right_hand: Some(Item {
                        on_use: ActionEffect::MeleeAttack {
                            damage: Hp::new(2.0),
                        },
                    }),
                },
            )
            .unwrap();

        let enemy = self.entities.spawn();
        self.entities
            .grid_position
            .insert(enemy, vec2(2, 1))
            .unwrap();
        self.entities
            .health
            .insert(enemy, Health::new(Hp::new(2.0)))
            .unwrap();
        self.entities
            .fraction
            .insert(enemy, Fraction::Enemy)
            .unwrap();
        self.entities
            .held_items
            .insert(
                enemy,
                HeldItems {
                    left_hand: None,
                    right_hand: Some(Item {
                        on_use: ActionEffect::MeleeAttack {
                            damage: Hp::new(1.0),
                        },
                    }),
                },
            )
            .unwrap();
        self.entities
            .unit
            .insert(
                enemy,
                Unit {
                    beat: UnitBeat::Independent { bpm: 100 },
                    next_beat: Time::ONE,
                    behaviour: UnitBehaviour::SelectTarget {
                        selector: TargetSelector {
                            filter: TargetFilter::Fraction(FractionFilter::Enemy),
                            fitness: TargetFitness::Negative(Box::new(TargetFitness::Distance)),
                        },
                        then_behave: Box::new(UnitBehaviour::If {
                            condition: BehaviourCondition::TargetInRange { distance: 1 },
                            then_behave: Box::new(UnitBehaviour::UseItemOnTarget {
                                item: ItemId::RightHand,
                            }),
                            else_behave: Box::new(UnitBehaviour::MoveToTarget),
                        }),
                    },
                },
            )
            .unwrap()
    }
}

impl From<ComponentError> for SystemError {
    fn from(value: ComponentError) -> Self {
        Self::Component(value)
    }
}

impl From<ContextError> for SystemError {
    fn from(value: ContextError) -> Self {
        Self::Context(value)
    }
}

impl From<BehaviourError> for SystemError {
    fn from(value: BehaviourError) -> Self {
        Self::Behaviour(value)
    }
}

impl Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::Component(error) => write!(f, "Component error: {error}"),
            SystemError::Context(error) => write!(f, "Context error: {error}"),
            SystemError::Behaviour(error) => write!(f, "Behaviour error: {error}"),
        }
    }
}
