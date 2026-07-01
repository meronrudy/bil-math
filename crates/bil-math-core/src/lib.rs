pub mod error;
pub mod graph;
pub mod ids;
pub mod scalar;

pub use error::MathError;
pub use graph::{DirectedEdge, InstitutionalGraph};
pub use ids::{CycleId, EdgeId, EvidenceId, InstitutionId, PolicyId, VertexId};
pub use scalar::{Matrix, Scalar, Vector};