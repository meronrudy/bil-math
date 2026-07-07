use bil_math_core::{MathError, Matrix, Scalar};

#[derive(Debug, Clone)]
pub struct HolonomyObstruction {
    pub holonomy: Matrix,
    pub identity_deviation: Scalar,
}

impl HolonomyObstruction {
    pub fn try_from_holonomy(holonomy: Matrix) -> Result<Self, MathError> {
        if holonomy.nrows() != holonomy.ncols() {
            return Err(MathError::NonSquareMatrix {
                rows: holonomy.nrows(),
                cols: holonomy.ncols(),
            });
        }

        Ok(Self::from_square_holonomy(holonomy))
    }

    pub fn from_holonomy(holonomy: Matrix) -> Self {
        assert_eq!(
            holonomy.nrows(),
            holonomy.ncols(),
            "holonomy obstruction requires a square holonomy matrix"
        );

        Self::from_square_holonomy(holonomy)
    }

    fn from_square_holonomy(holonomy: Matrix) -> Self {
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
