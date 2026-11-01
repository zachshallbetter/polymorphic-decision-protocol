use crate::{EvidenceClass, Vendor};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const RECORD_SCHEMA_VERSION: u16 = 1;
pub const PROTOCOL_VERSION: &str = "1.3.1";
pub const PANEL_SCHEMA_VERSION: u16 = 3;
pub const ELICITATION_SCHEMA_VERSION: u16 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleFit {
    Supported {
        mappings: BTreeMap<EvidenceClass, String>,
    },
    Unsupported {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub schema_version: u16,
    pub protocol_version: String,
    pub id: String,
    pub title: String,
    pub question: String,
    pub domain: String,
    pub options: Vec<String>,
    pub role_fit: RoleFit,
    pub stage: DecisionStage,
    pub human_actor: String,
}

impl DecisionRecord {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != RECORD_SCHEMA_VERSION {
            return Err(ValidationError::UnsupportedSchema(self.schema_version));
        }
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(ValidationError::UnsupportedProtocol(
                self.protocol_version.clone(),
            ));
        }
        for (name, value) in [
            ("id", self.id.as_str()),
            ("title", self.title.as_str()),
            ("question", self.question.as_str()),
            ("domain", self.domain.as_str()),
            ("human_actor", self.human_actor.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::MissingField(name));
            }
        }
        let unique_options: BTreeSet<&str> = self.options.iter().map(String::as_str).collect();
        if self.options.len() < 2 || unique_options.len() != self.options.len() {
            return Err(ValidationError::InvalidOptions);
        }
        if self.options.iter().any(|option| option.trim().is_empty()) {
            return Err(ValidationError::InvalidOptions);
        }
        if self.id.len() > 128
            || self.title.len() > 300
            || self.question.len() > 8_000
            || self.domain.len() > 300
            || self.human_actor.len() > 200
            || self.options.len() > 32
            || self.options.iter().any(|option| option.len() > 300)
        {
            return Err(ValidationError::InvalidOptions);
        }
        if let RoleFit::Supported { mappings } = &self.role_fit {
            let required = [
                EvidenceClass::Quant,
                EvidenceClass::Market,
                EvidenceClass::Structural,
                EvidenceClass::Contrarian,
            ];
            if required.iter().any(|role| {
                mappings
                    .get(role)
                    .map(|mapping| mapping.trim().is_empty())
                    .unwrap_or(true)
            }) {
                return Err(ValidationError::IncompleteRoleMapping);
            }
        } else if matches!(&self.role_fit, RoleFit::Unsupported { reason } if reason.trim().is_empty() || reason.len() > 2_000)
        {
            return Err(ValidationError::MissingField("unsupported domain reason"));
        }
        Ok(())
    }

    pub fn supports_conformant_run(&self) -> bool {
        matches!(&self.role_fit, RoleFit::Supported { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub id: String,
    pub decision_id: String,
    pub title: String,
    pub source_url: String,
    pub publisher: String,
    pub accessed_at: String,
    pub summary: String,
    pub source_kind: EvidenceSourceKind,
    pub roles: BTreeSet<EvidenceClass>,
}

impl EvidenceRecord {
    pub fn validate(&self) -> Result<(), ValidationError> {
        for (name, value) in [
            ("evidence id", self.id.as_str()),
            ("decision id", self.decision_id.as_str()),
            ("evidence title", self.title.as_str()),
            ("source URL", self.source_url.as_str()),
            ("publisher", self.publisher.as_str()),
            ("access time", self.accessed_at.as_str()),
            ("evidence summary", self.summary.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::MissingField(name));
            }
        }
        if self.roles.is_empty()
            || self.id.len() > 128
            || self.decision_id.len() > 128
            || self.title.len() > 500
            || self.publisher.len() > 300
            || self.summary.len() > 20_000
            || self.source_url.len() > 2_048
            || !valid_source_url(&self.source_url)
        {
            return Err(ValidationError::InvalidEvidence);
        }
        Ok(())
    }
}

fn valid_source_url(value: &str) -> bool {
    let Some((scheme, remainder)) = value.split_once("://") else {
        return false;
    };
    let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default();
    matches!(scheme, "http" | "https")
        && !authority.is_empty()
        && !value.chars().any(char::is_whitespace)
        && !value.chars().any(char::is_control)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSourceKind {
    Curated,
    LicensedSearch,
    ManualTranscript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAdapter {
    OpenAiResponses,
    AnthropicMessages,
    GoogleGenerateContent,
    OllamaChat,
    Manual,
}

impl ProviderAdapter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAiResponses => "open_ai_responses",
            Self::AnthropicMessages => "anthropic_messages",
            Self::GoogleGenerateContent => "google_generate_content",
            Self::OllamaChat => "ollama_chat",
            Self::Manual => "manual",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelPin {
    pub vendor: Vendor,
    pub provider_id: ProviderAdapter,
    pub model_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PanelDefinition {
    pub schema_version: u16,
    pub run_id: String,
    pub decision_id: String,
    pub models: Vec<ModelPin>,
    pub per_run_budget_micros: u64,
}

#[derive(Deserialize)]
struct PanelDefinitionWire {
    schema_version: Option<u16>,
    run_id: String,
    decision_id: String,
    models: Vec<ModelPinWire>,
    per_run_budget_micros: u64,
}

#[derive(Deserialize)]
struct ModelPinWire {
    vendor: Vendor,
    provider_id: Option<String>,
    model_id: String,
}

impl<'de> Deserialize<'de> for PanelDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = PanelDefinitionWire::deserialize(deserializer)?;
        let version = wire.schema_version.unwrap_or(1);
        let models =
            wire.models
                .into_iter()
                .map(|model| {
                    let provider_id =
                        match (version, model.provider_id) {
                            (1, None) => normalize_legacy_v1(&model.vendor)
                                .map_err(serde::de::Error::custom)?,
                            (2, Some(value)) => normalize_panel_v2_adapter(&value)
                                .map_err(serde::de::Error::custom)?,
                            (PANEL_SCHEMA_VERSION, Some(value)) => {
                                parse_provider_adapter(&value).map_err(serde::de::Error::custom)?
                            }
                            (1, Some(_)) => {
                                return Err(serde::de::Error::custom(
                                    "panel schema v1 does not define provider_id",
                                ))
                            }
                            (2, None) | (PANEL_SCHEMA_VERSION, None) => {
                                return Err(serde::de::Error::custom(
                                    "panel schema requires provider_id",
                                ))
                            }
                            (other, _) => {
                                return Err(serde::de::Error::custom(format!(
                                    "unsupported panel schema version {other}"
                                )))
                            }
                        };
                    Ok(ModelPin {
                        vendor: model.vendor,
                        provider_id,
                        model_id: model.model_id,
                    })
                })
                .collect::<Result<Vec<_>, D::Error>>()?;
        if version != 1 && version != 2 && version != PANEL_SCHEMA_VERSION {
            return Err(serde::de::Error::custom(format!(
                "unsupported panel schema version {version}"
            )));
        }
        Ok(Self {
            schema_version: PANEL_SCHEMA_VERSION,
            run_id: wire.run_id,
            decision_id: wire.decision_id,
            models,
            per_run_budget_micros: wire.per_run_budget_micros,
        })
    }
}

fn normalize_legacy_v1(vendor: &Vendor) -> Result<ProviderAdapter, &'static str> {
    match vendor.as_str() {
        "anthropic" => Ok(ProviderAdapter::AnthropicMessages),
        "open_ai" => Ok(ProviderAdapter::OpenAiResponses),
        "google" => Ok(ProviderAdapter::GoogleGenerateContent),
        _ => Err("legacy vendor ID cannot be normalized"),
    }
}

fn normalize_panel_v2_adapter(value: &str) -> Result<ProviderAdapter, &'static str> {
    match value {
        "openai" | "open_ai_responses" => Ok(ProviderAdapter::OpenAiResponses),
        "anthropic" | "anthropic_messages" => Ok(ProviderAdapter::AnthropicMessages),
        "google" | "google_generate_content" => Ok(ProviderAdapter::GoogleGenerateContent),
        "ollama" | "ollama_chat" => Ok(ProviderAdapter::OllamaChat),
        "manual" => Ok(ProviderAdapter::Manual),
        _ => Err("unknown panel v2 provider adapter"),
    }
}

fn parse_provider_adapter(value: &str) -> Result<ProviderAdapter, &'static str> {
    match value {
        "open_ai_responses" => Ok(ProviderAdapter::OpenAiResponses),
        "anthropic_messages" => Ok(ProviderAdapter::AnthropicMessages),
        "google_generate_content" => Ok(ProviderAdapter::GoogleGenerateContent),
        "ollama_chat" => Ok(ProviderAdapter::OllamaChat),
        "manual" => Ok(ProviderAdapter::Manual),
        _ => Err("unknown provider adapter"),
    }
}

impl PanelDefinition {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.run_id.trim().is_empty() || self.decision_id.trim().is_empty() {
            return Err(ValidationError::MissingField("panel identity"));
        }
        if self.run_id.len() > 128
            || self.decision_id.len() > 128
            || self.per_run_budget_micros == 0
            || self.per_run_budget_micros > i64::MAX as u64
        {
            return Err(ValidationError::MissingField("per-run budget"));
        }
        let present: BTreeSet<Vendor> = self
            .models
            .iter()
            .map(|model| model.vendor.clone())
            .collect();
        if self.models.len() != 3
            || present.len() != self.models.len()
            || self
                .models
                .iter()
                .any(|model| model.model_id.trim().is_empty() || model.model_id.len() > 300)
        {
            return Err(ValidationError::InvalidPanel);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElicitationRecord {
    pub id: String,
    pub decision_id: String,
    pub run_id: String,
    pub vendor: Vendor,
    pub model_id: String,
    pub role: EvidenceClass,
    pub pick: String,
    pub probability: f64,
    pub alternative: String,
    pub alternative_probability: f64,
    pub load_bearing_facts: Vec<String>,
    pub searched_versus_known: String,
    pub raw_response: String,
    pub manually_imported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CritiqueSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CritiqueFactor {
    pub description: String,
    pub severity: CritiqueSeverity,
    pub priced_into_confidence: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureClassification {
    Robust,
    EvidenceDrivenFlip,
    PressureDrivenFlip,
    Collapse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfCritiqueRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub vendor: Vendor,
    pub model_id: String,
    pub role: EvidenceClass,
    pub factors: Vec<CritiqueFactor>,
    pub flip_scenario: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormalModelRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub prepared_by: String,
    pub frozen_at: String,
    pub dimensions: Vec<crate::ScoreDimension>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressureTestRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub vendor: Vendor,
    pub model_id: String,
    pub role: EvidenceClass,
    pub prompt: String,
    pub response: String,
    pub classification: PressureClassification,
    pub new_evidence: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G2Review {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub reviewed_by: String,
    pub observable: String,
    pub precommitted_exit: String,
    pub distinct_from_variance: String,
    pub pressure_asymmetry: String,
    pub disposition: G1Disposition,
    pub rationale: String,
    pub reviewed_at: String,
}

impl G2Review {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != RECORD_SCHEMA_VERSION {
            return Err(ValidationError::UnsupportedSchema(self.schema_version));
        }
        for (name, value) in [
            ("decision id", self.decision_id.as_str()),
            ("run id", self.run_id.as_str()),
            ("reviewer", self.reviewed_by.as_str()),
            ("observable invalidation", self.observable.as_str()),
            ("precommitted exit", self.precommitted_exit.as_str()),
            ("variance distinction", self.distinct_from_variance.as_str()),
            ("pressure asymmetry", self.pressure_asymmetry.as_str()),
            ("rationale", self.rationale.as_str()),
            ("review time", self.reviewed_at.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::MissingField(name));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitmentRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub proposed_by: String,
    pub p: f64,
    pub value_if_right: f64,
    pub cost_if_wrong: f64,
    pub quarter_kelly: f64,
    pub matrix_multiplier: f64,
    pub bankroll: f64,
    pub ruin_tolerance: f64,
    pub committed_amount: f64,
    pub terminal_fraction: f64,
    pub checkpoint_fraction: f64,
    pub single_event_fraction: f64,
    pub rationale: String,
    pub proposed_at: String,
}

impl CommitmentRecord {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != RECORD_SCHEMA_VERSION {
            return Err(ValidationError::UnsupportedSchema(self.schema_version));
        }
        for (name, value) in [
            ("decision id", self.decision_id.as_str()),
            ("run id", self.run_id.as_str()),
            ("proposer", self.proposed_by.as_str()),
            ("rationale", self.rationale.as_str()),
            ("proposal time", self.proposed_at.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::MissingField(name));
            }
        }
        let values = [
            self.p,
            self.value_if_right,
            self.cost_if_wrong,
            self.quarter_kelly,
            self.matrix_multiplier,
            self.bankroll,
            self.ruin_tolerance,
            self.committed_amount,
            self.terminal_fraction,
            self.checkpoint_fraction,
            self.single_event_fraction,
        ];
        if values
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
            || !(0.0..=1.0).contains(&self.p)
            || self.value_if_right <= 0.0
            || self.cost_if_wrong <= 0.0
            || self.quarter_kelly != 0.25
            || self.matrix_multiplier > 1.0
            || self.bankroll <= 0.0
            || self.ruin_tolerance > self.bankroll
            || self.committed_amount > self.ruin_tolerance
            || !(0.4..=0.6).contains(&self.terminal_fraction)
            || !(0.4..=0.6).contains(&self.checkpoint_fraction)
            || self.single_event_fraction > 0.1
            || (self.terminal_fraction + self.checkpoint_fraction + self.single_event_fraction
                - 1.0)
                .abs()
                > 1e-6
        {
            return Err(ValidationError::InvalidCommitment);
        }
        let odds = self.value_if_right / self.cost_if_wrong;
        let full_kelly = ((self.p * odds - (1.0 - self.p)) / odds).max(0.0);
        if self.committed_amount
            > self.bankroll * full_kelly * self.quarter_kelly * self.matrix_multiplier + 1e-8
        {
            return Err(ValidationError::InvalidCommitment);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionLeg {
    pub instrument: String,
    pub direction: String,
    pub size: f64,
    pub price: f64,
    pub platform_confirmation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryExecutionRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub executed_by: String,
    pub entry_window_started: String,
    pub entry_window_ended: String,
    pub legs: Vec<ExecutionLeg>,
    pub reconciliation: String,
    pub reconciled_by: String,
    pub reconciled_at: String,
}

impl EntryExecutionRecord {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != RECORD_SCHEMA_VERSION {
            return Err(ValidationError::UnsupportedSchema(self.schema_version));
        }
        for (name, value) in [
            ("decision id", self.decision_id.as_str()),
            ("run id", self.run_id.as_str()),
            ("executor", self.executed_by.as_str()),
            ("entry window start", self.entry_window_started.as_str()),
            ("entry window end", self.entry_window_ended.as_str()),
            ("reconciliation", self.reconciliation.as_str()),
            ("reconciler", self.reconciled_by.as_str()),
            ("reconciled time", self.reconciled_at.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::MissingField(name));
            }
        }
        if self.entry_window_ended < self.entry_window_started
            || self.legs.is_empty()
            || self.legs.len() > 32
            || self.legs.iter().any(|leg| {
                leg.instrument.trim().is_empty()
                    || leg.direction.trim().is_empty()
                    || leg.platform_confirmation.trim().is_empty()
                    || !leg.size.is_finite()
                    || leg.size <= 0.0
                    || !leg.price.is_finite()
                    || leg.price < 0.0
            })
        {
            return Err(ValidationError::InvalidExecution);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub run_id: String,
    pub recorded_by: String,
    pub expected: String,
    pub actual: String,
    pub thesis_impact: String,
    pub external_source: String,
    pub matched_invalidation_condition: bool,
    pub recorded_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettlementRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub settled_by: String,
    pub realized_outcome: String,
    pub thesis_probability: f64,
    pub realized_return: f64,
    pub luck_decomposition: String,
    pub checkpoint_hit_rate: f64,
    pub invalidation_triggered: bool,
    pub settled_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub schema_version: u16,
    pub decision_id: String,
    pub auditor: String,
    pub outcome_hidden_during_scoring: bool,
    pub criteria: BTreeMap<String, u8>,
    pub evidence: String,
    pub calibration_note: String,
    pub audited_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "record_type", content = "record", rename_all = "snake_case")]
pub enum WorkflowRecord {
    SelfCritique(SelfCritiqueRecord),
    FormalModel(FormalModelRecord),
    PressureTest(PressureTestRecord),
    G2Review(G2Review),
    Commitment(CommitmentRecord),
    EntryExecution(EntryExecutionRecord),
    Monitoring(MonitoringRecord),
    Settlement(SettlementRecord),
    Audit(AuditRecord),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum G1Disposition {
    Approve,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct G1Review {
    pub schema_version: u16,
    pub protocol_version: String,
    pub decision_id: String,
    pub run_id: String,
    pub reviewed_by: String,
    pub modal_pick: String,
    pub acs: f64,
    pub agreeing_agent_ids: Vec<String>,
    pub total_agents: u16,
    pub sizing_band: String,
    pub factual_conflicts: String,
    pub weighting_conflicts: String,
    pub evidence_gaps: String,
    pub rationale: String,
    pub disposition: G1Disposition,
    pub reviewed_at: String,
}

impl G1Review {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != RECORD_SCHEMA_VERSION {
            return Err(ValidationError::UnsupportedSchema(self.schema_version));
        }
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(ValidationError::UnsupportedProtocol(
                self.protocol_version.clone(),
            ));
        }
        for (name, value) in [
            ("decision id", self.decision_id.as_str()),
            ("run id", self.run_id.as_str()),
            ("reviewer", self.reviewed_by.as_str()),
            ("modal pick", self.modal_pick.as_str()),
            ("sizing band", self.sizing_band.as_str()),
            ("factual conflict review", self.factual_conflicts.as_str()),
            (
                "weighting conflict review",
                self.weighting_conflicts.as_str(),
            ),
            ("evidence gap review", self.evidence_gaps.as_str()),
            ("review rationale", self.rationale.as_str()),
            ("review time", self.reviewed_at.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::MissingField(name));
            }
        }
        if !self.acs.is_finite()
            || !(0.0..=1.0).contains(&self.acs)
            || self.total_agents != 12
            || self.agreeing_agent_ids.is_empty()
            || self.agreeing_agent_ids.len() > usize::from(self.total_agents)
            || self.decision_id.len() > 128
            || self.run_id.len() > 128
            || self.reviewed_by.len() > 200
            || self.modal_pick.len() > 300
            || self.sizing_band.len() > 100
            || self.reviewed_at.len() > 64
            || [
                self.factual_conflicts.as_str(),
                self.weighting_conflicts.as_str(),
                self.evidence_gaps.as_str(),
                self.rationale.as_str(),
            ]
            .iter()
            .any(|value| value.len() > 2_000)
        {
            return Err(ValidationError::InvalidG1Review);
        }
        if self.disposition == G1Disposition::Approve
            && self.acs < 0.40
            && (self.factual_conflicts.trim().is_empty()
                || self.weighting_conflicts.trim().is_empty())
        {
            return Err(ValidationError::UnresolvedDivergence);
        }
        Ok(())
    }
}

impl ElicitationRecord {
    pub fn validate(&self, allowed_options: &[String]) -> Result<(), ValidationError> {
        if self.id.trim().is_empty()
            || self.decision_id.trim().is_empty()
            || self.run_id.trim().is_empty()
            || self.model_id.trim().is_empty()
            || self.pick.trim().is_empty()
            || self.alternative.trim().is_empty()
            || self.searched_versus_known.trim().is_empty()
            || self.raw_response.trim().is_empty()
            || self.id.len() > 128
            || self.decision_id.len() > 128
            || self.run_id.len() > 128
            || self.model_id.len() > 300
            || self.pick.len() > 300
            || self.alternative.len() > 300
            || self.searched_versus_known.len() > 2_000
            || self.raw_response.len() > 50_000
        {
            return Err(ValidationError::InvalidElicitation);
        }
        if !self.probability.is_finite()
            || !self.alternative_probability.is_finite()
            || !(0.0..=1.0).contains(&self.probability)
            || !(0.0..=1.0).contains(&self.alternative_probability)
        {
            return Err(ValidationError::InvalidProbability);
        }
        if !allowed_options.iter().any(|option| option == &self.pick)
            || !allowed_options
                .iter()
                .any(|option| option == &self.alternative)
            || self.pick == self.alternative
            || self.load_bearing_facts.len() != 3
            || self
                .load_bearing_facts
                .iter()
                .any(|fact| fact.trim().is_empty() || fact.len() > 2_000)
            || self
                .load_bearing_facts
                .iter()
                .collect::<BTreeSet<_>>()
                .len()
                != 3
        {
            return Err(ValidationError::InvalidElicitation);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionStage {
    Draft,
    EvidenceReady,
    Eliciting,
    G1Pending,
    StressTesting,
    G2Pending,
    G3Pending,
    Locked,
    Monitoring,
    Settled,
    Audited,
    Killed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum WorkflowAction {
    EvidenceReady,
    StartPanel,
    PanelCompleted,
    ApproveG1,
    RejectG1 { reason: String },
    StressCompleted,
    ApproveG2,
    RejectG2 { reason: String },
    CommitmentProposed,
    ApproveG3,
    RejectG3 { reason: String },
    StartMonitoring,
    RecordCheckpoint,
    TriggerInvalidation,
    RecordSettlement,
    RecordAudit,
    Kill { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionError {
    Invalid {
        from: DecisionStage,
        action: WorkflowAction,
    },
    EmptyReason,
}

impl DecisionStage {
    pub fn apply(self, action: &WorkflowAction) -> Result<Self, TransitionError> {
        use DecisionStage as S;
        use WorkflowAction as A;
        let next = match (self, action) {
            (S::Draft, A::EvidenceReady) => S::EvidenceReady,
            (S::EvidenceReady, A::StartPanel) => S::Eliciting,
            (S::Eliciting, A::PanelCompleted) => S::G1Pending,
            (S::G1Pending, A::ApproveG1) => S::StressTesting,
            (S::G1Pending, A::RejectG1 { .. }) => S::Killed,
            (S::StressTesting, A::StressCompleted) => S::G2Pending,
            (S::G2Pending, A::ApproveG2) => S::G3Pending,
            (S::G2Pending, A::RejectG2 { .. }) => S::Killed,
            (S::G3Pending, A::CommitmentProposed) => S::G3Pending,
            (S::G3Pending, A::ApproveG3) => S::Locked,
            (S::G3Pending, A::RejectG3 { .. }) => S::Killed,
            (S::Locked, A::StartMonitoring) => S::Monitoring,
            (S::Monitoring, A::RecordCheckpoint) => S::Monitoring,
            (S::Monitoring, A::TriggerInvalidation) => S::Monitoring,
            (S::Monitoring, A::RecordSettlement) => S::Settled,
            (S::Settled, A::RecordAudit) => S::Audited,
            (
                S::Draft
                | S::EvidenceReady
                | S::Eliciting
                | S::G1Pending
                | S::StressTesting
                | S::G2Pending
                | S::G3Pending,
                A::Kill { .. },
            ) => S::Killed,
            _ => {
                return Err(TransitionError::Invalid {
                    from: self,
                    action: action.clone(),
                })
            }
        };
        match action {
            A::RejectG1 { reason }
            | A::RejectG2 { reason }
            | A::RejectG3 { reason }
            | A::Kill { reason }
                if reason.trim().is_empty() =>
            {
                Err(TransitionError::EmptyReason)
            }
            _ => Ok(next),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowEvent {
    pub schema_version: u16,
    pub decision_id: String,
    pub sequence: u64,
    pub kind: String,
    pub actor: String,
    pub occurred_at: String,
    pub fields: Vec<CanonicalField>,
    pub previous_hash: Option<String>,
}

impl WorkflowEvent {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema_version != RECORD_SCHEMA_VERSION {
            return Err(ValidationError::UnsupportedSchema(self.schema_version));
        }
        if self.decision_id.trim().is_empty()
            || self.kind.trim().is_empty()
            || self.actor.trim().is_empty()
            || self.occurred_at.trim().is_empty()
        {
            return Err(ValidationError::MissingField("event identity"));
        }
        if self.sequence == 1 && self.previous_hash.is_some() {
            return Err(ValidationError::InvalidEventChain);
        }
        if self.sequence == 0
            || (self.sequence > 1 && self.previous_hash.as_deref().unwrap_or_default().is_empty())
        {
            return Err(ValidationError::InvalidEventChain);
        }
        let reserved = [
            "actor",
            "occurred_at",
            "decision_id",
            "kind",
            "previous_hash",
            "schema_version",
            "sequence",
        ];
        let names = self
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<BTreeSet<_>>();
        if names.len() != self.fields.len()
            || self.fields.iter().any(|field| {
                field.name.trim().is_empty()
                    || field.name.len() > 128
                    || field.value.len() > 20_000
                    || reserved.contains(&field.name.as_str())
            })
        {
            return Err(ValidationError::InvalidEventChain);
        }
        Ok(())
    }
}

pub fn verify_event_chain(events: &[(WorkflowEvent, String)]) -> bool {
    let mut prior: Option<String> = None;
    for (index, (event, expected_hash)) in events.iter().enumerate() {
        if event.sequence != index as u64 + 1
            || event.previous_hash != prior
            || event.validate().is_err()
            || event.content_hash() != *expected_hash
        {
            return false;
        }
        prior = Some(expected_hash.clone());
    }
    true
}

pub trait CanonicalRecord {
    fn canonical_domain(&self) -> &'static str;
    fn canonical_fields(&self) -> Vec<CanonicalField>;

    fn content_hash(&self) -> String {
        canonical_hash(self.canonical_domain(), self.canonical_fields())
    }
}

impl CanonicalRecord for crate::InvalidationCondition {
    fn canonical_domain(&self) -> &'static str {
        "pdp.invalidation.v1"
    }

    fn canonical_fields(&self) -> Vec<CanonicalField> {
        let mut fields = vec![
            field("condition_text", &self.condition_text),
            field("exit_action", &self.exit_action),
            field("status", &self.status),
        ];
        let mut inputs = self.observable_inputs.clone();
        inputs.sort();
        fields.extend(inputs.iter().map(|input| field("observable_input", input)));
        fields
    }
}

impl CanonicalRecord for WorkflowEvent {
    fn canonical_domain(&self) -> &'static str {
        "pdp.event.v1"
    }

    fn canonical_fields(&self) -> Vec<CanonicalField> {
        let mut fields = vec![
            field("actor", &self.actor),
            field("occurred_at", &self.occurred_at),
            field("decision_id", &self.decision_id),
            field("kind", &self.kind),
            field("previous_hash", self.previous_hash.as_deref().unwrap_or("")),
            field("schema_version", &self.schema_version.to_string()),
            field("sequence", &self.sequence.to_string()),
        ];
        fields.extend(self.fields.iter().cloned());
        fields
    }
}

pub fn canonical_hash(domain: &str, mut fields: Vec<CanonicalField>) -> String {
    fields.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.value.cmp(&right.value))
    });
    let mut hasher = Sha256::new();
    frame(&mut hasher, domain.as_bytes());
    for item in fields {
        frame(&mut hasher, item.name.as_bytes());
        frame(&mut hasher, item.value.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn field(name: &str, value: &str) -> CanonicalField {
    CanonicalField {
        name: name.to_owned(),
        value: value.to_owned(),
    }
}

fn frame(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    UnsupportedSchema(u16),
    UnsupportedProtocol(String),
    MissingField(&'static str),
    InvalidOptions,
    IncompleteRoleMapping,
    InvalidPanel,
    InvalidElicitation,
    InvalidG1Review,
    UnresolvedDivergence,
    InvalidProbability,
    InvalidEventChain,
    InvalidEvidence,
    InvalidStressRecord,
    InvalidCommitment,
    InvalidExecution,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decision() -> DecisionRecord {
        let mappings = [
            (EvidenceClass::Quant, "historical metrics"),
            (EvidenceClass::Market, "current pricing and consensus"),
            (EvidenceClass::Structural, "qualitative evidence"),
            (
                EvidenceClass::Contrarian,
                "evidence against the leading option",
            ),
        ]
        .into_iter()
        .map(|(role, value)| (role, value.to_owned()))
        .collect();
        DecisionRecord {
            schema_version: RECORD_SCHEMA_VERSION,
            protocol_version: PROTOCOL_VERSION.to_owned(),
            id: "d1".to_owned(),
            title: "Decision".to_owned(),
            question: "Which option?".to_owned(),
            domain: "general".to_owned(),
            options: vec!["A".to_owned(), "B".to_owned()],
            role_fit: RoleFit::Supported { mappings },
            stage: DecisionStage::Draft,
            human_actor: "local-human".to_owned(),
        }
    }

    #[test]
    fn strict_role_mapping_is_required_for_conformant_run() {
        let mut incomplete = decision();
        incomplete.role_fit = RoleFit::Supported {
            mappings: BTreeMap::new(),
        };
        assert!(incomplete.validate().is_err());
        let mut unsupported = decision();
        unsupported.role_fit = RoleFit::Unsupported {
            reason: "no market evidence".into(),
        };
        assert!(unsupported.validate().is_ok());
        assert!(!unsupported.supports_conformant_run());
    }

    #[test]
    fn role_fit_json_matches_the_local_ui_discriminators() {
        let supported = r#"{"supported":{"mappings":{"quant":"q","market":"m","structural":"s","contrarian":"c"}}}"#;
        let parsed: RoleFit = serde_json::from_str(supported).unwrap();
        assert!(matches!(parsed, RoleFit::Supported { .. }));
        let unsupported = r#"{"unsupported":{"reason":"domain not mapped"}}"#;
        let parsed: RoleFit = serde_json::from_str(unsupported).unwrap();
        assert!(matches!(parsed, RoleFit::Unsupported { .. }));
    }

    #[test]
    fn panel_requires_three_distinct_vendors_and_a_budget() {
        let panel = PanelDefinition {
            schema_version: PANEL_SCHEMA_VERSION,
            run_id: "r1".into(),
            decision_id: "d1".into(),
            per_run_budget_micros: 1,
            models: vec![
                ModelPin {
                    vendor: Vendor::new("anthropic").unwrap(),
                    provider_id: ProviderAdapter::AnthropicMessages,
                    model_id: "a".into(),
                },
                ModelPin {
                    vendor: Vendor::new("open_ai").unwrap(),
                    provider_id: ProviderAdapter::OpenAiResponses,
                    model_id: "b".into(),
                },
                ModelPin {
                    vendor: Vendor::new("google").unwrap(),
                    provider_id: ProviderAdapter::GoogleGenerateContent,
                    model_id: "c".into(),
                },
            ],
        };
        assert!(panel.validate().is_ok());
        let mut no_budget = panel.clone();
        no_budget.per_run_budget_micros = 0;
        assert!(no_budget.validate().is_err());
        assert_eq!(
            serde_json::to_string(&Vendor::new("open_ai").unwrap()).unwrap(),
            "\"open_ai\""
        );

        let mut chosen = panel;
        chosen.models = vec![
            ModelPin {
                vendor: Vendor::new("open_ai").unwrap(),
                provider_id: ProviderAdapter::OpenAiResponses,
                model_id: "gpt-6-luna".into(),
            },
            ModelPin {
                vendor: Vendor::new("liquid_ai").unwrap(),
                provider_id: ProviderAdapter::OllamaChat,
                model_id: "lfm3:1.7b".into(),
            },
            ModelPin {
                vendor: Vendor::new("alibaba").unwrap(),
                provider_id: ProviderAdapter::Manual,
                model_id: "Qwen 3.5 4B Vision Q4".into(),
            },
        ];
        assert!(chosen.validate().is_ok());
        chosen.models[2].vendor = Vendor::new("liquid_ai").unwrap();
        assert!(chosen.validate().is_err());
    }

    #[test]
    fn vendor_identifiers_preserve_legacy_wire_values_and_reject_noncanonical_ids() {
        let legacy: Vendor = serde_json::from_str("\"open_ai\"").unwrap();
        assert_eq!(legacy.as_str(), "open_ai");
        assert_eq!(serde_json::to_string(&legacy).unwrap(), "\"open_ai\"");
        assert_eq!(Vendor::new("bonzai_web").unwrap().as_str(), "bonzai_web");
        assert!(serde_json::from_str::<Vendor>("\"OpenAI\"").is_err());
        assert_eq!(Vendor::new("meta-llama").unwrap().as_str(), "meta-llama");
        assert!(serde_json::from_str::<Vendor>("\"vendor--id\"").is_err());
    }

    #[test]
    fn legacy_panel_adapter_ids_normalize_only_at_deserialization_boundary() {
        let raw = r#"{"run_id":"r","decision_id":"d","per_run_budget_micros":10000,"models":[{"vendor":"anthropic","model_id":"claude-haiku"},{"vendor":"open_ai","model_id":"gpt-6-luna"},{"vendor":"google","model_id":"gemini-flash"}]}"#;
        let panel: PanelDefinition = serde_json::from_str(raw).unwrap();
        assert_eq!(panel.schema_version, PANEL_SCHEMA_VERSION);
        assert_eq!(
            panel.models[0].provider_id,
            ProviderAdapter::AnthropicMessages
        );
        assert_eq!(
            panel.models[1].provider_id,
            ProviderAdapter::OpenAiResponses
        );
        assert_eq!(panel.models[1].vendor.as_str(), "open_ai");
    }

    #[test]
    fn panel_v2_adapter_strings_migrate_and_unknown_v3_discriminator_fails() {
        let v2 = r#"{"schema_version":2,"run_id":"r","decision_id":"d","per_run_budget_micros":10000,"models":[{"vendor":"vendor-a","provider_id":"openai","model_id":"m1"},{"vendor":"vendor-b","provider_id":"ollama","model_id":"m2"},{"vendor":"vendor-c","provider_id":"manual","model_id":"m3"}]}"#;
        let panel: PanelDefinition = serde_json::from_str(v2).unwrap();
        assert_eq!(
            panel.models[0].provider_id,
            ProviderAdapter::OpenAiResponses
        );
        let v3 = r#"{"schema_version":3,"run_id":"r","decision_id":"d","per_run_budget_micros":10000,"models":[{"vendor":"vendor-a","provider_id":"mystery","model_id":"m1"},{"vendor":"vendor-b","provider_id":"ollama_chat","model_id":"m2"},{"vendor":"vendor-c","provider_id":"manual","model_id":"m3"}]}"#;
        assert!(serde_json::from_str::<PanelDefinition>(v3).is_err());
    }

    #[test]
    fn commitment_requires_protocol_matrix_kelly_ruin_and_ladder_bounds() {
        let record = CommitmentRecord {
            schema_version: 1,
            decision_id: "d1".into(),
            run_id: "r1".into(),
            proposed_by: "human".into(),
            p: 0.6,
            value_if_right: 2.0,
            cost_if_wrong: 1.0,
            quarter_kelly: 0.25,
            matrix_multiplier: 0.5,
            bankroll: 100.0,
            ruin_tolerance: 5.0,
            committed_amount: 1.0,
            terminal_fraction: 0.45,
            checkpoint_fraction: 0.45,
            single_event_fraction: 0.10,
            rationale: "bounded test proposal".into(),
            proposed_at: "2026-10-31T00:00:00Z".into(),
        };
        assert!(record.validate().is_ok());
        let mut over_cap = record.clone();
        over_cap.committed_amount = 10.0;
        assert!(over_cap.validate().is_err());
        let mut overallocated = record;
        overallocated.single_event_fraction = 0.2;
        assert!(overallocated.validate().is_err());
    }

    #[test]
    fn entry_execution_requires_one_bounded_window_and_reconciliation() {
        let record = EntryExecutionRecord {
            schema_version: 1,
            decision_id: "d1".into(),
            run_id: "r1".into(),
            executed_by: "human".into(),
            entry_window_started: "2026-10-31T10:00:00Z".into(),
            entry_window_ended: "2026-10-31T10:05:00Z".into(),
            legs: vec![ExecutionLeg {
                instrument: "contract".into(),
                direction: "buy".into(),
                size: 1.0,
                price: 0.25,
                platform_confirmation: "confirmation-1".into(),
            }],
            reconciliation: "matched planned legs to fills".into(),
            reconciled_by: "human".into(),
            reconciled_at: "2026-10-31T10:10:00Z".into(),
        };
        assert!(record.validate().is_ok());
        let mut invalid = record;
        invalid.entry_window_ended = "2026-10-31T09:59:00Z".into();
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn g1_review_requires_review_fields_and_resolves_low_acs_divergence() {
        let review = G1Review {
            schema_version: RECORD_SCHEMA_VERSION,
            protocol_version: PROTOCOL_VERSION.into(),
            decision_id: "d1".into(),
            run_id: "r1".into(),
            reviewed_by: "human".into(),
            modal_pick: "A".into(),
            acs: 0.3,
            agreeing_agent_ids: vec!["open_ai:quant".into()],
            total_agents: 12,
            sizing_band: "divergence_resolution_required".into(),
            factual_conflicts: "A and B differ on the base rate".into(),
            weighting_conflicts: "The panel weights cost differently".into(),
            evidence_gaps: "No primary cost record".into(),
            rationale: "Proceed only after human review".into(),
            disposition: G1Disposition::Approve,
            reviewed_at: "2026-10-31T00:00:00Z".into(),
        };
        assert!(review.validate().is_ok());
        let mut unresolved = review.clone();
        unresolved.factual_conflicts.clear();
        assert_eq!(
            unresolved.validate(),
            Err(ValidationError::MissingField("factual conflict review"))
        );
        let mut not_resolved = review;
        not_resolved.weighting_conflicts = " ".into();
        assert_eq!(
            not_resolved.validate(),
            Err(ValidationError::MissingField("weighting conflict review"))
        );
    }

    #[test]
    fn evidence_and_manual_transcript_records_reject_unbounded_or_invalid_values() {
        let evidence = EvidenceRecord {
            id: "e1".into(),
            decision_id: "d1".into(),
            title: "Source".into(),
            source_url: "https://example.test/source".into(),
            publisher: "Publisher".into(),
            accessed_at: "2026-01-01T00:00:00Z".into(),
            summary: "Summary".into(),
            source_kind: EvidenceSourceKind::Curated,
            roles: BTreeSet::from([EvidenceClass::Market]),
        };
        assert!(evidence.validate().is_ok());
        let mut unsafe_url = evidence.clone();
        unsafe_url.source_url = "javascript:alert(1)".into();
        assert!(unsafe_url.validate().is_err());
        let transcript = ElicitationRecord {
            id: "t1".into(),
            decision_id: "d1".into(),
            run_id: "r1".into(),
            vendor: Vendor::new("open_ai").unwrap(),
            model_id: "pinned-model".into(),
            role: EvidenceClass::Market,
            pick: "A".into(),
            probability: 0.6,
            alternative: "B".into(),
            alternative_probability: 0.4,
            load_bearing_facts: vec!["one".into(), "two".into(), "three".into()],
            searched_versus_known: "manual import".into(),
            raw_response: "verbatim".into(),
            manually_imported: true,
        };
        assert!(transcript.validate(&["A".into(), "B".into()]).is_ok());
        let mut invalid_probability = transcript;
        invalid_probability.probability = f64::NAN;
        assert!(invalid_probability
            .validate(&["A".into(), "B".into()])
            .is_err());
    }

    #[test]
    fn workflow_does_not_skip_human_gates_or_exit_lock() {
        use DecisionStage as S;
        use WorkflowAction as A;
        assert!(matches!(
            S::Draft.apply(&A::StartPanel),
            Err(TransitionError::Invalid { .. })
        ));
        assert_eq!(S::G1Pending.apply(&A::ApproveG1).unwrap(), S::StressTesting);
        assert_eq!(S::G2Pending.apply(&A::ApproveG2).unwrap(), S::G3Pending);
        assert_eq!(S::G3Pending.apply(&A::ApproveG3).unwrap(), S::Locked);
        assert!(S::Locked
            .apply(&A::RejectG3 {
                reason: "late".into()
            })
            .is_err());
        assert!(S::G1Pending
            .apply(&A::RejectG1 {
                reason: "  ".into()
            })
            .is_err());
    }

    #[test]
    fn event_hash_is_order_independent_for_fields_and_chained_to_prior_event() {
        let a = WorkflowEvent {
            schema_version: 1,
            decision_id: "d".into(),
            sequence: 0,
            kind: "created".into(),
            actor: "human".into(),
            occurred_at: "2026-10-31T00:00:00Z".into(),
            fields: vec![field("question", "q"), field("title", "t")],
            previous_hash: None,
        };
        let mut b = a.clone();
        b.fields.reverse();
        assert_eq!(a.content_hash(), b.content_hash());
        b.previous_hash = Some(a.content_hash());
        b.sequence = 1;
        assert_ne!(a.content_hash(), b.content_hash());
    }

    #[test]
    fn event_chain_rejects_modified_or_reordered_records() {
        let event = WorkflowEvent {
            schema_version: RECORD_SCHEMA_VERSION,
            decision_id: "d1".into(),
            sequence: 1,
            kind: "created".into(),
            actor: "human".into(),
            occurred_at: "unix:1".into(),
            fields: vec![CanonicalField {
                name: "title".into(),
                value: "test".into(),
            }],
            previous_hash: None,
        };
        let valid = vec![(event.clone(), event.content_hash())];
        assert!(verify_event_chain(&valid));
        let mut changed = event;
        changed.fields[0].value = "changed".into();
        assert!(!verify_event_chain(&[(changed, valid[0].1.clone())]));
    }
}
