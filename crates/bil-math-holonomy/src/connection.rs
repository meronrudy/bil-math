use bil_math_core::{EdgeId, Matrix};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct DiscreteConnection {
    pub transports: BTreeMap<EdgeId, Matrix>,
}

impl DiscreteConnection {
    pub fn new(transports: BTreeMap<EdgeId, Matrix>) -> Self {
        Self { transports }
    }

    pub fn transport(&self, edge: EdgeId) -> Option<&Matrix> {
        self.transports.get(&edge)
    }
}