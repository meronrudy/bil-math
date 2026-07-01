use bil_math_core::{EdgeId, Matrix};

#[derive(Debug, Clone)]
pub struct TransportMap {
    pub edge: EdgeId,
    pub map: Matrix,
}

impl TransportMap {
    pub fn new(edge: EdgeId, map: Matrix) -> Self {
        Self { edge, map }
    }
}
