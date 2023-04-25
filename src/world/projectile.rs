use super::*;

#[derive(StructOf, Debug, Clone)]
pub struct Projectile {
    // pub grid_position: vec2<Coord>,
    pub world_position: vec2<FCoord>,
    pub velocity: vec2<FCoord>,
    pub fraction: Fraction,
    pub target_filter: FractionFilter,
    pub on_contact: Effect,
}

#[derive(Debug, Clone)]
pub struct ProjectilePrefab {
    pub on_contact: Effect,
}

#[derive(Debug, Clone)]
pub struct ProjectileInst {
    // pub grid_position: vec2<Coord>,
    pub world_position: vec2<FCoord>,
    pub velocity: vec2<FCoord>,
    pub fraction: Fraction,
    pub target_filter: FractionFilter,
}

impl ProjectilePrefab {
    pub fn instantiate(self, inst: ProjectileInst) -> Projectile {
        Projectile {
            // grid_position: inst.grid_position,
            world_position: inst.world_position,
            velocity: inst.velocity,
            fraction: inst.fraction,
            target_filter: inst.target_filter,
            on_contact: self.on_contact,
        }
    }
}
