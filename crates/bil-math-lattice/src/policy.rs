use crate::PartialOrder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyLevel {
    Permissive = 0,
    Standard = 1,
    Restricted = 2,
    Locked = 3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyState {
    pub level: PolicyLevel,
}

impl PolicyState {
    pub fn new(level: PolicyLevel) -> Self {
        Self { level }
    }
}

impl PartialOrder for PolicyState {
    fn leq(&self, other: &Self) -> bool {
        self.level <= other.level
    }
}