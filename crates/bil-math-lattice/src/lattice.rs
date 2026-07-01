use crate::{PartialOrder, PolicyState};

#[derive(Debug, Clone)]
pub struct FiniteLattice {
    pub states: Vec<PolicyState>,
}

impl FiniteLattice {
    pub fn new(states: Vec<PolicyState>) -> Self {
        Self { states }
    }
}