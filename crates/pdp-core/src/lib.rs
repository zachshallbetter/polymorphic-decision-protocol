pub mod cents;
pub mod math;
pub mod workflow;

use cents::Cents;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;

pub use workflow::{CanonicalRecord, WorkflowEvent};

pub fn calculate_hash<T: CanonicalRecord>(value: &T) -> String {
    value.content_hash()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Vendor(String);

impl Vendor {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        let valid = !value.is_empty()
            && value.len() <= 64
            && value.as_bytes()[0].is_ascii_lowercase()
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
            })
            && !value.ends_with('_')
            && !value.ends_with('-')
            && !value.contains("__")
            && !value.contains("--")
            && !value.contains("_-")
            && !value.contains("-_");
        if valid {
            Ok(Self(value))
        } else {
            Err("vendor ID must be a lowercase canonical identifier")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Vendor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    Quant,
    Market,
    Structural,
    Contrarian,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentVerdict {
    pub agent_id: String,
    pub vendor: Vendor,
    pub evidence_class: EvidenceClass,
    pub pick: String,
    pub stated_p: f64,
    pub alternatives: Vec<(String, f64)>,
    pub load_bearing_facts: Vec<String>,
    pub searched: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Acs {
    pub score: f64,
    pub agreeing_agents: Vec<String>,
    pub total_agents: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlipDistance {
    pub distance: f64,
    pub sensitive_dimension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreDimension {
    pub name: String,
    pub weight: f64,
    pub scores: BTreeMap<String, f64>, // option_name -> score (0.0 to 10.0)
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoredModel {
    pub dimensions: Vec<ScoreDimension>,
    pub option_scores: BTreeMap<String, f64>, // option_name -> overall weighted score
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizingBand {
    Full,
    Half,
    Quarter,
    Kill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegType {
    Terminal,
    Checkpoint,
    SingleEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegSpec {
    pub name: String,
    pub leg_type: LegType,
    pub proportion: f64,
    pub allocated_amount: Option<Cents>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ladder {
    pub total: Cents,
    pub legs: Vec<LegSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvalidationCondition {
    pub condition_text: String,
    pub observable_inputs: Vec<String>,
    pub exit_action: String,
    pub status: String, // "Armed", "Triggered", "Expired"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KellyInputs {
    pub p: f64,
    pub b: f64,
    pub k: f64,
    pub bankroll: Cents,
    pub ruin_cap: Cents,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitmentStructure {
    pub ladder: Ladder,
    pub total_committed: Cents,
    pub matrix_cell: SizingBand,
    pub kelly_inputs: KellyInputs,
    pub confirmed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreCommittedExit {
    pub action_text: String,
    pub target_asset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Outcome {
    pub winning_pick: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessScore {
    pub panel_decorrelation: u8,
    pub elicitation_quality: u8,
    pub self_critique_before_pressure: u8,
    pub weights_frozen: u8,
    pub sensitivity_analysis: u8,
    pub invalidation_quality: u8,
    pub entry_discipline: u8,
    pub lock_integrity: u8,
    pub total_score: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gate {
    G1,
    G2,
    G3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionState {
    Draft,
    Eliciting,
    G1Pending {
        acs: Acs,
    },
    StressTesting,
    G2Pending {
        flip_distance: FlipDistance,
        invalidation: InvalidationCondition,
    },
    ComposingStructure,
    G3Pending {
        proposal: CommitmentStructure,
    },
    Locked {
        structure: CommitmentStructure,
        invalidation: InvalidationCondition,
    },
    InvalidationTriggered {
        exit: PreCommittedExit,
    },
    Settled {
        outcome: Outcome,
    },
    Audited {
        process_score: ProcessScore,
    },
    Killed {
        at_gate: Gate,
        reason: String,
    },
}
