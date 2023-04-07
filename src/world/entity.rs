use super::*;

pub type EntityId = Id;

pub struct Entities {
    id_gen: IdGenerator,
    ids: ComponentStorage<()>,
    pub unit: ComponentStorage<UnitAI>,
    pub fraction: ComponentStorage<Fraction>,
    pub grid_position: ComponentStorage<vec2<Coord>>,
    pub world_position: ComponentStorage<vec2<FCoord>>,
    pub velocity: ComponentStorage<vec2<FCoord>>,
    pub health: ComponentStorage<Health>,
    pub held_items: ComponentStorage<HeldItems>,
    pub color: ComponentStorage<Color>,
    pub particle: ComponentStorage<Particle>, // TODO: separate
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ids(&self) -> &ComponentStorage<()> {
        &self.ids
    }

    pub fn spawn(&mut self) -> EntityId {
        let id = self.id_gen.next();
        self.ids
            .insert(id, ())
            .expect("Failed to generate a unique id");
        id
    }

    pub fn remove(&mut self, id: EntityId) -> bool {
        let _ = self.grid_position.remove(id);
        let _ = self.world_position.remove(id);
        let _ = self.velocity.remove(id);
        let _ = self.health.remove(id);
        let _ = self.unit.remove(id);
        let _ = self.fraction.remove(id);
        let _ = self.held_items.remove(id);
        let _ = self.color.remove(id);
        let _ = self.particle.remove(id);
        self.ids.remove(id).is_ok()
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self {
            id_gen: default(),
            ids: ComponentStorage::new("Id"),
            grid_position: ComponentStorage::new("GridPosition"),
            world_position: ComponentStorage::new("WorldPosition"),
            velocity: ComponentStorage::new("Velocity"),
            health: ComponentStorage::new("Health"),
            unit: ComponentStorage::new("Unit"),
            fraction: ComponentStorage::new("Fraction"),
            held_items: ComponentStorage::new("HeldItems"),
            color: ComponentStorage::new("Color"),
            particle: ComponentStorage::new("Particle"),
        }
    }
}
