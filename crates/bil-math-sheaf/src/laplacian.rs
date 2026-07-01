use crate::{coboundary::coboundary_matrix, CellularSheaf};
use bil_math_core::{MathError, Matrix};

pub fn sheaf_laplacian(sheaf: &CellularSheaf) -> Result<Matrix, MathError> {
    let delta = coboundary_matrix(sheaf)?;
    Ok(delta.transpose() * delta)
}