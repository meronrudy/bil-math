pub trait PartialOrder {
    fn leq(&self, other: &Self) -> bool;

    fn geq(&self, other: &Self) -> bool {
        other.leq(self)
    }

    fn comparable(&self, other: &Self) -> bool {
        self.leq(other) || other.leq(self)
    }
}
