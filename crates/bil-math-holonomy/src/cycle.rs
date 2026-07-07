use bil_math_core::EdgeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub edges: Vec<EdgeId>,
}

impl Cycle {
    pub fn new(edges: Vec<EdgeId>) -> Self {
        Self { edges }
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn edges(&self) -> &[EdgeId] {
        &self.edges
    }
}
