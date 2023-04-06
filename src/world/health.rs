use super::*;

pub type Hp = R32;

#[derive(Debug, Clone)]
pub struct Health {
    max_hp: Hp,
    hp: Hp,
}

impl Health {
    pub fn new(max_hp: Hp) -> Self {
        Self { hp: max_hp, max_hp }
    }

    pub fn get(&self) -> Hp {
        self.hp
    }

    pub fn get_max(&self) -> Hp {
        self.max_hp
    }

    pub fn get_ratio(&self) -> R32 {
        self.hp / self.max_hp
    }

    pub fn is_alive(&self) -> bool {
        self.hp > Hp::ZERO
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    pub fn heal(&mut self, hp: Hp) {
        self.change(hp)
    }

    pub fn damage(&mut self, damage: Hp) {
        self.change(-damage)
    }

    pub fn change(&mut self, hp: Hp) {
        let target = self.hp + hp;
        self.hp = target.clamp(Hp::ZERO, self.max_hp); // TODO: overheal
    }

    pub fn change_max(&mut self, delta: Hp) {
        self.max_hp += delta; // TODO: update `hp` accordingly (e.g. remain at constant %)
    }
}
