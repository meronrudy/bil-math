use crate::{EdgeId, VertexId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectedEdge {
    pub id: EdgeId,
    pub source: VertexId,
    pub target: VertexId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalGraph {
    vertices: Vec<VertexId>,
    edges: Vec<DirectedEdge>,
}

impl InstitutionalGraph {
    pub fn new(vertices: Vec<VertexId>, edges: Vec<DirectedEdge>) -> Self {
        Self { vertices, edges }
    }

    pub fn vertices(&self) -> &[VertexId] {
        &self.vertices
    }

    pub fn edges(&self) -> &[DirectedEdge] {
        &self.edges
    }

    pub fn edge(&self, id: EdgeId) -> Option<&DirectedEdge> {
        self.edges.iter().find(|edge| edge.id == id)
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
