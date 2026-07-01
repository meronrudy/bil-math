use crate::{residual::sheaf_residual, CellularSheaf, ZeroCochain};
use bil_math_core::{MathError, Scalar};

pub fn is_global_section(
    sheaf: &CellularSheaf,
    cochain: &ZeroCochain,
    tolerance: Scalar,
) -> Result<bool, MathError> {
    let residual = sheaf_residual(sheaf, cochain)?;
    Ok(residual.energy <= tolerance)
}