use crate::CellularSheaf;
use bil_math_core::{MathError, Matrix};

pub fn coboundary_matrix(sheaf: &CellularSheaf) -> Result<Matrix, MathError> {
    let rows = sheaf.c1_dim()?;
    let cols = sheaf.c0_dim()?;

    let mut delta = Matrix::zeros(rows, cols);

    for edge in sheaf.graph.edges() {
        let edge_dim = *sheaf
            .edge_dims
            .get(&edge.id)
            .ok_or(MathError::MissingEdge(edge.id.0))?;

        let row_offset = sheaf.edge_offset(edge.id)?;

        let source_dim = *sheaf
            .vertex_dims
            .get(&edge.source)
            .ok_or(MathError::MissingVertex(edge.source.0))?;

        let target_dim = *sheaf
            .vertex_dims
            .get(&edge.target)
            .ok_or(MathError::MissingVertex(edge.target.0))?;

        let source_offset = sheaf.vertex_offset(edge.source)?;
        let target_offset = sheaf.vertex_offset(edge.target)?;

        let source_restriction = sheaf.restriction(edge.source, edge.id)?;
        let target_restriction = sheaf.restriction(edge.target, edge.id)?;

        if source_restriction.nrows() != edge_dim || source_restriction.ncols() != source_dim {
            return Err(MathError::DimensionMismatch {
                expected: edge_dim,
                actual: source_restriction.nrows(),
            });
        }

        if target_restriction.nrows() != edge_dim || target_restriction.ncols() != target_dim {
            return Err(MathError::DimensionMismatch {
                expected: edge_dim,
                actual: target_restriction.nrows(),
            });
        }

        for r in 0..edge_dim {
            for c in 0..source_dim {
                delta[(row_offset + r, source_offset + c)] = source_restriction[(r, c)];
            }

            for c in 0..target_dim {
                delta[(row_offset + r, target_offset + c)] = -target_restriction[(r, c)];
            }
        }
    }

    Ok(delta)
}
