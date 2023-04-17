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
    pub grid: Grid,
    pub player: Player,
    pub beat_controller: BeatController,
    pub music_controller: MusicController,
    /// Normalized (in range 0..1) time since the last player's beat.
    pub player_beat_time: Time,
    pub units: Units,
    pub particles: Vec<Particle>,
}

pub type SystemResult<T> = Result<T, SystemError>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum SystemError {
    #[error("Component error: {0}")]
    Component(#[from] ComponentError),
    #[error("Context error: {0}")]
    Context(#[from] ContextError),
    #[error("Behaviour error: {0}")]
    Behaviour(#[from] BehaviourError),
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: vec2<FCoord>,
    pub velocity: vec2<FCoord>,
    pub lifetime: Health,
    /// Diameter.
    pub size: FCoord,
    pub color: Color,
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
        let mut units = Units::new();

        let player_unit = units.spawn();

        let mut world = Self {
            geng: geng.clone(),
            player: Player::new(player_unit),
            grid: Grid::default(),
            music_controller: MusicController::new(
                music_config,
                beat_config.bpm_min as f32,
                synthesizers,
            ),
            beat_controller: BeatController::new(beat_config),
            player_beat_time: Time::ZERO,
            units,
            particles: Vec::new(),
        };
        world.init();
        world
    }

    fn init(&mut self) {
        let player = self.player.unit;
        self.units.grid_position.insert(player, vec2::ZERO).unwrap();
        self.units
            .health
            .insert(player, Health::new(Hp::new(10.0)))
            .unwrap();
        self.units
            .fraction
            .insert(player, Fraction::Player)
            .unwrap();
        self.units
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

        let enemy = self.units.spawn();
        self.units.grid_position.insert(enemy, vec2(2, 1)).unwrap();
        self.units
            .health
            .insert(enemy, Health::new(Hp::new(2.0)))
            .unwrap();
        self.units.fraction.insert(enemy, Fraction::Enemy).unwrap();
        self.units
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
        self.units
            .unit
            .insert(
                enemy,
                UnitAI {
                    beat: UnitBeat::Synchronized {
                        unit: 1,
                        player: 2,
                        current_beat: 0,
                    },
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
