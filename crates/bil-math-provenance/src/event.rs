use bil_math_core::{EvidenceId, PolicyId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    InputObserved,
    ModelInvoked,
    PolicyApplied,
    HumanApproved,
    DecisionEmitted,
    ReceiptIssued,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEvent {
    pub evidence_id: EvidenceId,
    pub kind: EventKind,
    pub policy_context: Option<PolicyId>,
    pub logical_time: u64,
}

impl DecisionEvent {
    pub fn new(
        evidence_id: EvidenceId,
        kind: EventKind,
        policy_context: Option<PolicyId>,
        logical_time: u64,
    ) -> Self {
        Self {
            evidence_id,
            kind,
            policy_context,
            logical_time,
        }
    }
}