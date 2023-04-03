use geng::prelude::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdGenerator {
    next_id: Id,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: Id(0) }
    }

    pub fn next(&mut self) -> Id {
        let id = self.next_id;
        self.next_id.0 += 1;
        id
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}
