use super::*;

#[derive(Debug, Clone)]
pub struct TargetSelector {
    pub filter: TargetFilter,
    pub fitness: TargetFitness,
}

#[derive(Debug, Clone, Copy)]
pub enum TargetFilter {
    Own,
    Fraction(FractionFilter),
}

#[derive(Debug, Clone, Copy)]
pub enum FractionFilter {
    Any,
    Ally,
    Enemy,
}

#[derive(Debug, Clone)]
pub enum TargetFitness {
    Negative(Box<TargetFitness>),
    Distance,
}

pub type Fitness = R32;

impl TargetSelector {
    pub fn evaluate(
        &self,
        world: &World,
        entity: EntityId,
    ) -> ComponentResult<Option<EffectTarget>> {
        let mut best_target = None;

        let mut eval_entity = |id: EntityId| -> ComponentResult<()> {
            let fitness = self.fitness.evaluate(id, world, entity)?;
            let target = (id, fitness);
            best_target = Some(
                best_target
                    .map(|best| std::cmp::max_by_key(best, target, |&(_, fit)| fit))
                    .unwrap_or(target),
            );
            Ok(())
        };

        if let TargetFilter::Own = self.filter {
            eval_entity(entity)?;
        } else {
            for (id, ()) in world.entities.ids().iter() {
                if id != entity
                    && (id == world.player.entity || world.entities.unit.contains(id))
                    && self.filter.check(id, world, entity)?
                {
                    eval_entity(id)?;
                }
            }
        }

        Ok(best_target.map(|(target, _)| EffectTarget::Entity(target)))
    }
}

impl TargetFilter {
    pub fn check(self, target: EntityId, world: &World, entity: EntityId) -> ComponentResult<bool> {
        match self {
            TargetFilter::Own => Ok(target == entity),
            TargetFilter::Fraction(filter) => filter.check_query(target, world, entity),
        }
    }
}

impl FractionFilter {
    pub fn check_query(
        self,
        target: EntityId,
        world: &World,
        entity: EntityId,
    ) -> ComponentResult<bool> {
        let &entity = world.entities.fraction.get(entity)?;
        let &target = world.entities.fraction.get(target)?;
        Ok(self.check(entity, target))
    }

    pub fn check(self, entity: Fraction, other: Fraction) -> bool {
        match self {
            FractionFilter::Any => true,
            FractionFilter::Ally => entity == other,
            FractionFilter::Enemy => entity != other,
        }
    }
}

impl TargetFitness {
    pub fn evaluate(
        &self,
        target: EntityId,
        world: &World,
        entity: EntityId,
    ) -> ComponentResult<Fitness> {
        match self {
            TargetFitness::Negative(fitness) => Ok(-fitness.evaluate(target, world, entity)?),
            TargetFitness::Distance => {
                let &pos = world.entities.grid_position.get(entity)?;
                let &target_pos = world.entities.grid_position.get(target)?;
                let delta = target_pos - pos;
                let distance = crate::util::king_distance(delta);
                Ok(Fitness::new(distance as f32))
            }
        }
    }
}
