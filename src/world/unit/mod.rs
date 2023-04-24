use super::*;

mod behaviour;

pub use behaviour::*;

pub type UnitId = Id;

#[derive(Debug, Clone)]
pub struct UnitAI {
    pub beat: UnitBeat,
    /// Normalized (in range 0..1) time until the next beat.
    pub next_beat: Time,
    pub behaviour: UnitBehaviour,
}

/// Describes how often the unit makes decisions.
#[derive(Debug, Clone)]
pub enum UnitBeat {
    /// Beats once every `player / unit` player's beats.
    Synchronized {
        /// How many times should the unit beat.
        unit: Ticks,
        /// How many player's beats should pass.
        player: Ticks,
        /// Current beat index modulo `player`.
        current_beat: Ticks,
    },
    Independent {
        bpm: Ticks,
    },
}

#[derive(StructOf)]
pub struct Unit {
    pub unit: Option<UnitAI>,
    pub fraction: Fraction,
    pub grid_position: vec2<Coord>,
    pub world_position: vec2<FCoord>,
    pub health: Health,
    pub held_items: HeldItems,
}
