use ecs::prelude::*;
use geng::prelude::*;

use crate::{
    collection::{Collection, Id},
    sound::{
        BeatConfig, BeatController, MusicConfig, MusicController, SectionName, Synthesizer, Ticks,
    },
};

mod action;
mod action_effect;
mod component;
mod condition;
mod context;
mod effect;
mod grid;
mod health;
mod item;
mod logic;
mod player;
mod projectile;
mod target;
mod unit;

pub use action::*;
pub use action_effect::*;
pub use component::*;
pub use condition::*;
pub use context::*;
pub use effect::*;
pub use grid::*;
pub use health::*;
pub use item::*;
pub use logic::*;
pub use player::*;
pub use projectile::*;
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
    pub units: StructOf<Collection<Unit>>,
    pub projectiles: StructOf<Collection<Projectile>>,
    pub particles: StructOf<Vec<Particle>>,
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

#[derive(StructOf, Debug, Clone)]
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

        let mut units = StructOf::<Collection<Unit>>::new();

        let player_unit = units.insert(Unit {
            grid_position: vec2::ZERO,
            world_position: vec2::ZERO,
            unit: None,
            health: Health::new(Hp::new(10.0)),
            fraction: Fraction::Player,
            held_items: HeldItems {
                left_hand: Some(Item::bow(Hp::new(1.0), FCoord::new(3.0))),
                right_hand: Some(Item::sword(Hp::new(2.0))),
            },
        });

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
            projectiles: StructOf::new(),
            particles: StructOf::new(),
        };
        world.init();
        world
    }

    fn init(&mut self) {
        self.units.insert(Unit {
            unit: Some(UnitAI {
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
            }),
            fraction: Fraction::Enemy,
            grid_position: vec2(2, 1),
            world_position: vec2::ZERO,
            health: Health::new(Hp::new(2.0)),
            held_items: HeldItems {
                left_hand: None,
                right_hand: Some(Item::sword(Hp::new(1.0))),
            },
        });
    }
}
