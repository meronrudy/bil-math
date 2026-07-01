use bil_math_core::EvidenceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceObject {
    pub id: EvidenceId,
    pub label: String,
}

impl EvidenceObject {
    pub fn new(id: EvidenceId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
        }
    }
}