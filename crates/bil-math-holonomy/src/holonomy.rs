use crate::{Cycle, DiscreteConnection};
use bil_math_core::{MathError, Matrix};

pub fn cycle_holonomy(connection: &DiscreteConnection, cycle: &Cycle) -> Result<Matrix, MathError> {
    if cycle.is_empty() {
        return Err(MathError::InvalidCycle(
            "cannot compute holonomy over an empty cycle".to_string(),
        ));
    }

    let first = cycle.edges[0];
    let first_map = connection
        .transport(first)
        .ok_or(MathError::MissingEdge(first.0))?;

    let mut accumulated = Matrix::identity(first_map.nrows(), first_map.ncols());

    for edge in &cycle.edges {
        let map = connection
            .transport(*edge)
            .ok_or(MathError::MissingEdge(edge.0))?;

        if map.ncols() != accumulated.nrows() {
            return Err(MathError::DimensionMismatch {
                expected: accumulated.nrows(),
                actual: map.ncols(),
            });
        }

        accumulated = map * accumulated;
    }

    Ok(accumulated)
}
