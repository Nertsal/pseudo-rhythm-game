use super::*;

pub type EntityId = Id;

pub struct Entities {
    id_gen: IdGenerator,
    ids: ComponentStorage<()>,
    pub position: ComponentStorage<vec2<Coord>>,
    pub health: ComponentStorage<Health>,
    pub held_items: ComponentStorage<HeldItems>,
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> EntityId {
        let id = self.id_gen.next();
        self.ids
            .insert(id, ())
            .expect("Failed to generate a unique id");
        id
    }

    pub fn remove(&mut self, id: EntityId) -> bool {
        let _ = self.position.remove(id);
        let _ = self.health.remove(id);
        let _ = self.held_items.remove(id);
        self.ids.remove(id).is_ok()
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self {
            id_gen: default(),
            ids: ComponentStorage::new("Id"),
            position: ComponentStorage::new("Position"),
            health: ComponentStorage::new("Health"),
            held_items: ComponentStorage::new("HeldItems"),
        }
    }
}
