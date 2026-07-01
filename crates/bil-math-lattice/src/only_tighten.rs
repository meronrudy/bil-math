use crate::{PartialOrder, PolicyState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TighteningResult {
    Accepted,
    RejectedWouldLoosen,
}

pub trait OnlyTighten {
    fn only_tighten_from(&self, previous: &Self) -> TighteningResult;
}

impl OnlyTighten for PolicyState {
    fn only_tighten_from(&self, previous: &Self) -> TighteningResult {
        if previous.leq(self) {
            TighteningResult::Accepted
        } else {
            TighteningResult::RejectedWouldLoosen
        }
    }
}