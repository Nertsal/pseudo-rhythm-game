use super::*;

pub type UnitAction = Action;

pub type BehaviourResult<T> = Result<T, BehaviourError>;

#[derive(Debug, Clone)]
pub enum BehaviourError {
    Component(ComponentError),
    Context(BehaviourContextError),
}

#[derive(Debug, Clone)]
pub enum BehaviourContextError {
    NoTarget,
    NoInput,
}

/// Basically the decision tree of the unit.
#[derive(Debug, Clone)]
pub enum UnitBehaviour {
    Act(UnitAction),
    SelectTarget {
        selector: TargetSelector,
        then_behave: Box<UnitBehaviour>,
    },
    MoveToTarget,
    UseItemOnTarget {
        item: ItemId,
    },
    If {
        condition: BehaviourCondition,
        then_behave: Box<UnitBehaviour>,
        else_behave: Box<UnitBehaviour>,
    },
}

#[derive(Debug, Clone)]
pub enum BehaviourCondition {
    TargetInRange { distance: Coord },
}

#[derive(Debug, Clone)]
enum BehaviourContext {
    None,
    Target(EffectTarget),
    Input(ActionInput),
}

impl UnitBeat {
    pub fn calc_bpm(&self, player_bpm: f32) -> f32 {
        match *self {
            UnitBeat::Synchronized { unit, player } => player_bpm * unit as f32 / player as f32,
            UnitBeat::Independent { bpm } => bpm as f32,
        }
    }
}

impl UnitBehaviour {
    pub fn evaluate(
        &self,
        world: &World,
        unit: UnitId,
    ) -> BehaviourResult<Option<(UnitAction, ActionInput)>> {
        let context = BehaviourContext::None;
        self.evaluate_with_context(world, unit, context)
    }

    fn evaluate_with_context(
        &self,
        world: &World,
        unit: UnitId,
        context: BehaviourContext,
    ) -> BehaviourResult<Option<(UnitAction, ActionInput)>> {
        match self {
            Self::Act(action) => Ok(Some((action.clone(), context.expect_input()?))),
            Self::SelectTarget {
                selector,
                then_behave,
            } => {
                match selector.evaluate(world, unit)? {
                    None => {
                        // No target found
                        Ok(None)
                    }
                    Some(target) => {
                        let context = BehaviourContext::Target(target);
                        then_behave.evaluate_with_context(world, unit, context)
                    }
                }
            }
            Self::MoveToTarget => {
                let target = context.expect_target()?;
                let target_pos = target.find_pos(world)?;
                let &pos = world.units.grid_position.get(unit)?;
                let delta = target_pos - pos;
                let move_delta = crate::util::vec_to_dir(delta.map(|x| x as f32));
                Ok(Some((
                    Action::Move(ActionMove::Slide(MoveSlide { delta: move_delta })),
                    ActionInput { target },
                )))
            }
            &Self::UseItemOnTarget { item } => {
                let target = context.expect_target()?;
                Ok(Some((
                    Action::UseItem(ActionUseItem { item }),
                    ActionInput { target },
                )))
            }
            Self::If {
                condition,
                then_behave,
                else_behave,
            } => {
                let behave = if condition.evaluate(world, unit, &context)? {
                    then_behave
                } else {
                    else_behave
                };
                behave.evaluate_with_context(world, unit, context)
            }
        }
    }
}

impl BehaviourCondition {
    fn evaluate(
        &self,
        world: &World,
        unit: UnitId,
        context: &BehaviourContext,
    ) -> BehaviourResult<bool> {
        match self {
            &BehaviourCondition::TargetInRange { distance } => match context {
                BehaviourContext::Target(target) => {
                    let &pos = world.units.grid_position.get(unit)?;
                    let target_pos = target.find_pos(world)?;
                    Ok(crate::util::king_distance(target_pos - pos) <= distance)
                }
                _ => Err(BehaviourError::Context(BehaviourContextError::NoTarget)),
            },
        }
    }
}

impl BehaviourContext {
    pub fn expect_target(self) -> Result<EffectTarget, BehaviourContextError> {
        match self {
            Self::Target(target) => Ok(target),
            _ => Err(BehaviourContextError::NoTarget),
        }
    }

    pub fn expect_input(self) -> Result<ActionInput, BehaviourContextError> {
        match self {
            Self::Input(input) => Ok(input),
            _ => Err(BehaviourContextError::NoInput),
        }
    }
}

impl From<ComponentError> for BehaviourError {
    fn from(value: ComponentError) -> Self {
        Self::Component(value)
    }
}

impl From<BehaviourContextError> for BehaviourError {
    fn from(value: BehaviourContextError) -> Self {
        Self::Context(value)
    }
}

impl Display for BehaviourError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BehaviourError::Component(error) => write!(f, "Component error: {error}"),
            BehaviourError::Context(error) => write!(f, "Context error: {error}"),
        }
    }
}

impl Display for BehaviourContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BehaviourContextError::NoTarget => write!(f, "behaviour context missing target"),
            BehaviourContextError::NoInput => write!(f, "behaviour context missing action input"),
        }
    }
}
