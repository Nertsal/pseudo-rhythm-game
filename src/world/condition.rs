use super::*;

#[derive(Debug, Clone)]
pub enum Condition {
    TargetIsUnit,
}

impl Condition {
    pub fn evaluate(&self, world: &World, context: &EffectContext) -> SystemResult<bool> {
        match self {
            Condition::TargetIsUnit => match context.target {
                Some(EffectTarget::Unit(_)) => Ok(true),
                _ => Ok(false),
            },
        }
    }
}
