use crate::DecisionEvent;
use bil_math_core::{EvidenceId, MathError};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct ProvenanceDag {
    pub events: BTreeMap<EvidenceId, DecisionEvent>,
    pub parents: BTreeMap<EvidenceId, BTreeSet<EvidenceId>>,
}

impl ProvenanceDag {
    pub fn new() -> Self {
        Self {
            events: BTreeMap::new(),
            parents: BTreeMap::new(),
        }
    }

    pub fn add_event(&mut self, event: DecisionEvent) {
        self.parents.entry(event.evidence_id).or_default();
        self.events.insert(event.evidence_id, event);
    }

    pub fn add_dependency(
        &mut self,
        child: EvidenceId,
        parent: EvidenceId,
    ) -> Result<(), MathError> {
        if !self.events.contains_key(&child) {
            return Err(MathError::MissingEdge(child.0));
        }

        if !self.events.contains_key(&parent) {
            return Err(MathError::MissingEdge(parent.0));
        }

        self.parents.entry(child).or_default().insert(parent);

        Ok(())
    }
}

impl Default for ProvenanceDag {
    fn default() -> Self {
        Self::new()
    }
}
