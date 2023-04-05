use super::*;

#[derive(Debug, Clone)]
pub struct EffectContext {
    pub caster: Option<Caster>,
    pub target: Option<Target>,
}

pub type ContextResult<T> = Result<T, ContextError>;

#[derive(Debug, Clone)]
pub enum ContextError {
    NoCaster,
    NoTarget,
}

impl EffectContext {
    pub fn expect_caster(&self) -> ContextResult<Caster> {
        self.caster.clone().ok_or(ContextError::NoCaster)
    }

    pub fn expect_target(&self) -> ContextResult<Target> {
        self.target.clone().ok_or(ContextError::NoTarget)
    }
}

impl From<ContextError> for SystemError {
    fn from(value: ContextError) -> Self {
        Self::Context(value)
    }
}

impl Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::NoCaster => write!(f, "Caster expected but not found in effect context"),
            ContextError::NoTarget => write!(f, "Target expected but not found in effect context"),
        }
    }
}
