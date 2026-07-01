use bil_math_core::{
    EdgeId, InstitutionalGraph, MathError, Matrix, VertexId,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RestrictionKey {
    pub vertex: VertexId,
    pub edge: EdgeId,
}

#[derive(Debug, Clone)]
pub struct CellularSheaf {
    pub graph: InstitutionalGraph,
    pub vertex_dims: BTreeMap<VertexId, usize>,
    pub edge_dims: BTreeMap<EdgeId, usize>,
    pub restrictions: BTreeMap<RestrictionKey, Matrix>,
}

impl CellularSheaf {
    pub fn new(
        graph: InstitutionalGraph,
        vertex_dims: BTreeMap<VertexId, usize>,
        edge_dims: BTreeMap<EdgeId, usize>,
        restrictions: BTreeMap<RestrictionKey, Matrix>,
    ) -> Self {
        Self {
            graph,
            vertex_dims,
            edge_dims,
            restrictions,
        }
    }

    pub fn vertex_offset(&self, vertex: VertexId) -> Result<usize, MathError> {
        let mut offset = 0;

        for v in self.graph.vertices() {
            if *v == vertex {
                return Ok(offset);
            }

            offset += self
                .vertex_dims
                .get(v)
                .copied()
                .ok_or(MathError::MissingVertex(v.0))?;
        }

        Err(MathError::MissingVertex(vertex.0))
    }

    pub fn edge_offset(&self, edge: EdgeId) -> Result<usize, MathError> {
        let mut offset = 0;

        for e in self.graph.edges() {
            if e.id == edge {
                return Ok(offset);
            }

            offset += self
                .edge_dims
                .get(&e.id)
                .copied()
                .ok_or(MathError::MissingEdge(e.id.0))?;
        }

        Err(MathError::MissingEdge(edge.0))
    }

    pub fn c0_dim(&self) -> Result<usize, MathError> {
        self.graph
            .vertices()
            .iter()
            .map(|v| {
                self.vertex_dims
                    .get(v)
                    .copied()
                    .ok_or(MathError::MissingVertex(v.0))
            })
            .sum()
    }

    pub fn c1_dim(&self) -> Result<usize, MathError> {
        self.graph
            .edges()
            .iter()
            .map(|e| {
                self.edge_dims
                    .get(&e.id)
                    .copied()
                    .ok_or(MathError::MissingEdge(e.id.0))
            })
            .sum()
    }

    pub fn restriction(
        &self,
        vertex: VertexId,
        edge: EdgeId,
    ) -> Result<&Matrix, MathError> {
        self.restrictions
            .get(&RestrictionKey { vertex, edge })
            .ok_or(MathError::MissingRestriction { edge, vertex })
    }
}