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
    pub fn evaluate(&self, world: &World, unit: UnitId) -> ComponentResult<Option<EffectTarget>> {
        let mut best_target = None;

        let mut eval_unit = |id: UnitId| -> ComponentResult<()> {
            let fitness = self.fitness.evaluate(id, world, unit)?;
            let target = (id, fitness);
            best_target = Some(
                best_target
                    .map(|best| std::cmp::max_by_key(best, target, |&(_, fit)| fit))
                    .unwrap_or(target),
            );
            Ok(())
        };

        if let TargetFilter::Own = self.filter {
            eval_unit(unit)?;
        } else {
            for id in world.units.ids() {
                if id != unit
                    // && (id == world.player.unit || world.units.unit.contains(id))
                    && self.filter.check(id, world, unit)?
                {
                    eval_unit(id)?;
                }
            }
        }

        Ok(best_target.map(|(target, _)| EffectTarget::Unit(target)))
    }
}

impl TargetFilter {
    pub fn check(self, target: UnitId, world: &World, unit: UnitId) -> ComponentResult<bool> {
        match self {
            TargetFilter::Own => Ok(target == unit),
            TargetFilter::Fraction(filter) => filter.check_query(target, world, unit),
        }
    }
}

impl FractionFilter {
    pub fn check_query(self, target: UnitId, world: &World, unit: UnitId) -> ComponentResult<bool> {
        let &unit = world.units.fraction.get(unit).expect("Unit not found");
        let &target = world.units.fraction.get(target).expect("Unit not found");
        Ok(self.check(unit, target))
    }

    pub fn check(self, unit: Fraction, other: Fraction) -> bool {
        match self {
            FractionFilter::Any => true,
            FractionFilter::Ally => unit == other,
            FractionFilter::Enemy => unit != other,
        }
    }
}

impl TargetFitness {
    pub fn evaluate(
        &self,
        target: UnitId,
        world: &World,
        unit: UnitId,
    ) -> ComponentResult<Fitness> {
        match self {
            TargetFitness::Negative(fitness) => Ok(-fitness.evaluate(target, world, unit)?),
            TargetFitness::Distance => {
                let &pos = world.units.grid_position.get(unit).expect("Unit not found");
                let &target_pos = world
                    .units
                    .grid_position
                    .get(target)
                    .expect("Unit not found");
                let delta = target_pos - pos;
                let distance = crate::util::king_distance(delta);
                Ok(Fitness::new(distance as f32))
            }
        }
    }
}
