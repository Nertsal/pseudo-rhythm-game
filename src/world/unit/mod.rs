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

pub struct Units {
    id_gen: IdGenerator,
    ids: ComponentStorage<()>,
    pub unit: ComponentStorage<UnitAI>,
    pub fraction: ComponentStorage<Fraction>,
    pub grid_position: ComponentStorage<vec2<Coord>>,
    pub world_position: ComponentStorage<vec2<FCoord>>,
    pub velocity: ComponentStorage<vec2<FCoord>>,
    pub health: ComponentStorage<Health>,
    pub held_items: ComponentStorage<HeldItems>,
}

impl Units {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ids(&self) -> &ComponentStorage<()> {
        &self.ids
    }

    pub fn spawn(&mut self) -> UnitId {
        let id = self.id_gen.next();
        self.ids
            .insert(id, ())
            .expect("Failed to generate a unique id");
        id
    }

    pub fn remove(&mut self, id: UnitId) -> bool {
        let _ = self.unit.remove(id);
        let _ = self.fraction.remove(id);
        let _ = self.grid_position.remove(id);
        let _ = self.world_position.remove(id);
        let _ = self.velocity.remove(id);
        let _ = self.health.remove(id);
        let _ = self.held_items.remove(id);
        self.ids.remove(id).is_ok()
    }
}

impl Default for Units {
    fn default() -> Self {
        Self {
            id_gen: default(),
            ids: ComponentStorage::new("UnitId"),
            unit: ComponentStorage::new("Unit"),
            fraction: ComponentStorage::new("Fraction"),
            grid_position: ComponentStorage::new("GridPosition"),
            world_position: ComponentStorage::new("WorldPosition"),
            velocity: ComponentStorage::new("Velocity"),
            health: ComponentStorage::new("Health"),
            held_items: ComponentStorage::new("HeldItems"),
        }
    }
}
