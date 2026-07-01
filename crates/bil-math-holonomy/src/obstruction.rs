use bil_math_core::{Matrix, Scalar};

#[derive(Debug, Clone)]
pub struct HolonomyObstruction {
    pub holonomy: Matrix,
    pub identity_deviation: Scalar,
}

impl HolonomyObstruction {
    pub fn from_holonomy(holonomy: Matrix) -> Self {
        let identity = Matrix::identity(holonomy.nrows(), holonomy.ncols());
        let deviation = &holonomy - identity;
        let identity_deviation = deviation.norm();

        Self {
            holonomy,
            identity_deviation,
        }
    }

    pub fn is_trivial(&self, tolerance: Scalar) -> bool {
        self.identity_deviation <= tolerance
    }
}