use crate::ProvenanceDag;

#[derive(Debug, Clone)]
pub struct ReplayInvariant {
    pub event_count: usize,
    pub dependency_count: usize,
}

impl ReplayInvariant {
    pub fn from_dag(dag: &ProvenanceDag) -> Self {
        let event_count = dag.events.len();
        let dependency_count = dag.parents.values().map(|parents| parents.len()).sum();

        Self {
            event_count,
            dependency_count,
        }
    }
}