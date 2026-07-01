use bil_math_core::Vector;

#[derive(Debug, Clone)]
pub struct ZeroCochain {
    pub values: Vector,
}

impl ZeroCochain {
    pub fn new(values: Vector) -> Self {
        Self { values }
    }

    pub fn dim(&self) -> usize {
        self.values.len()
    }
}
