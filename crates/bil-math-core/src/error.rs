use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("missing vertex: {0}")]
    MissingVertex(usize),

    #[error("missing edge: {0}")]
    MissingEdge(usize),

    #[error("missing restriction map for edge {edge:?} and vertex {vertex:?}")]
    MissingRestriction {
        edge: crate::EdgeId,
        vertex: crate::VertexId,
    },

    #[error("invalid cycle: {0}")]
    InvalidCycle(String),

    #[error("empty structure")]
    EmptyStructure,
}
