pub mod cents;
pub mod math;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use cents::Cents;
use sha2::{Sha256, Digest};

pub fn calculate_hash<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let serialized = serde_json::to_string(value)?;
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vendor {
    Anthropic,
    OpenAI,
    Google,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub scores: HashMap<String, f64>, // option_name -> score (0.0 to 10.0)
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoredModel {
    pub dimensions: Vec<ScoreDimension>,
    pub option_scores: HashMap<String, f64>, // option_name -> overall weighted score
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
