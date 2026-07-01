use crate::CellularSheaf;
use crate::{coboundary::coboundary_matrix, ZeroCochain};
use bil_math_core::{MathError, Scalar, Vector};

#[derive(Debug, Clone)]
pub struct SheafResidual {
    pub residual_vector: Vector,
    pub energy: Scalar,
}

pub fn sheaf_residual(
    sheaf: &CellularSheaf,
    cochain: &ZeroCochain,
) -> Result<SheafResidual, MathError> {
    let delta = coboundary_matrix(sheaf)?;

    if delta.ncols() != cochain.dim() {
        return Err(MathError::DimensionMismatch {
            expected: delta.ncols(),
            actual: cochain.dim(),
        });
    }

    let residual_vector = delta * &cochain.values;
    let energy = residual_vector.dot(&residual_vector);

    Ok(SheafResidual {
        residual_vector,
        energy,
    })
}
