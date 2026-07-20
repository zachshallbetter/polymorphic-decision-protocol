use pdp_core::{
    math::{acs, committed_fraction, flip_distance, matrix_cell, score_model, allocate_ladder, AllocError},
    AgentVerdict, EvidenceClass, LegSpec, LegType, ScoreDimension, SizingBand, Vendor,
    cents::Cents,
};
use std::collections::HashMap;

#[test]
fn test_acs_unanimous_cross_vendor_same_evidence() {
    // 3 agents, different vendors, same evidence class
    let agents = vec![
        AgentVerdict {
            agent_id: "claude-1".to_string(),
            vendor: Vendor::Anthropic,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
        AgentVerdict {
            agent_id: "gemini-1".to_string(),
            vendor: Vendor::Google,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
        AgentVerdict {
            agent_id: "gpt-1".to_string(),
            vendor: Vendor::OpenAI,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
    ];

    let result = acs(&agents);
    assert!((result.score - 0.5).abs() < 1e-9); // (0.5 + 0.5 + 0.5) / 3 = 0.5
    assert_eq!(result.agreeing_agents.len(), 3);
}

#[test]
fn test_acs_unanimous_cross_vendor_cross_evidence() {
    // 3 agents, different vendors, different evidence classes
    let agents = vec![
        AgentVerdict {
            agent_id: "claude-1".to_string(),
            vendor: Vendor::Anthropic,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
        AgentVerdict {
            agent_id: "gemini-1".to_string(),
            vendor: Vendor::Google,
            evidence_class: EvidenceClass::Market,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
        AgentVerdict {
            agent_id: "gpt-1".to_string(),
            vendor: Vendor::OpenAI,
            evidence_class: EvidenceClass::Structural,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
    ];

    let result = acs(&agents);
    assert!((result.score - 1.0).abs() < 1e-9); // (1.0 + 1.0 + 1.0) / 3 = 1.0
}

#[test]
fn test_acs_one_analyst_n_hats() {
    // 3 agents, same vendor, same evidence class
    let agents = vec![
        AgentVerdict {
            agent_id: "gpt-1".to_string(),
            vendor: Vendor::OpenAI,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
        AgentVerdict {
            agent_id: "gpt-2".to_string(),
            vendor: Vendor::OpenAI,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
        AgentVerdict {
            agent_id: "gpt-3".to_string(),
            vendor: Vendor::OpenAI,
            evidence_class: EvidenceClass::Quant,
            pick: "Spain".to_string(),
            stated_p: 0.20,
            alternatives: vec![],
            load_bearing_facts: vec![],
            searched: "".to_string(),
        },
    ];

    let result = acs(&agents);
    assert!((result.score - 0.2).abs() < 1e-9); // (0.2 + 0.2 + 0.2) / 3 = 0.2
}

#[test]
fn test_flip_distance_spain_france() {
    // Spain/France World Cup case
    let mut scores1 = HashMap::new();
    scores1.insert("Spain".to_string(), 8.0);
    scores1.insert("France".to_string(), 10.0);

    let mut scores2 = HashMap::new();
    scores2.insert("Spain".to_string(), 9.0);
    scores2.insert("France".to_string(), 8.0);

    let mut scores3 = HashMap::new();
    scores3.insert("Spain".to_string(), 10.0);
    scores3.insert("France".to_string(), 7.0);

    let mut scores4 = HashMap::new();
    scores4.insert("Spain".to_string(), 9.0);
    scores4.insert("France".to_string(), 8.0);

    let mut scores5 = HashMap::new();
    scores5.insert("Spain".to_string(), 5.0);
    scores5.insert("France".to_string(), 8.0);

    let mut scores6 = HashMap::new();
    scores6.insert("Spain".to_string(), 10.0);
    scores6.insert("France".to_string(), 8.0);

    let mut scores7 = HashMap::new();
    scores7.insert("Spain".to_string(), 9.0);
    scores7.insert("France".to_string(), 6.0);

    let mut scores8 = HashMap::new();
    scores8.insert("Spain".to_string(), 7.0);
    scores8.insert("France".to_string(), 9.0);

    let dimensions = vec![
        ScoreDimension {
            name: "Attack Peak".to_string(),
            weight: 0.15,
            scores: scores1,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "Attack Depth".to_string(),
            weight: 0.15,
            scores: scores2,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "Defense".to_string(),
            weight: 0.20,
            scores: scores3,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "Recent Form".to_string(),
            weight: 0.15,
            scores: scores4,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "WC Pedigree".to_string(),
            weight: 0.10,
            scores: scores5,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "Cohesion".to_string(),
            weight: 0.10,
            scores: scores6,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "Injury Risk".to_string(),
            weight: 0.10,
            scores: scores7,
            justification: "".to_string(),
        },
        ScoreDimension {
            name: "Experience".to_string(),
            weight: 0.05,
            scores: scores8,
            justification: "".to_string(),
        },
    ];

    let model = score_model(&dimensions);
    
    // Spain overall: 8.65, France overall: 7.95
    assert!((model.option_scores.get("Spain").unwrap() - 8.65).abs() < 1e-9);
    assert!((model.option_scores.get("France").unwrap() - 7.95).abs() < 1e-9);

    let fd = flip_distance(&model).unwrap();
    
    // Minimum delta should be in "WC Pedigree", which is ~0.17027
    assert_eq!(fd.sensitive_dimension.as_deref(), Some("WC Pedigree"));
    assert!((fd.distance - 0.17027027).abs() < 1e-6);
}

#[test]
fn test_matrix_cell_lookup() {
    // Robust, ACS strong
    assert_eq!(matrix_cell(0.80, 0.20), SizingBand::Full);
    // Robust, ACS single-axis
    assert_eq!(matrix_cell(0.50, 0.20), SizingBand::Half);
    // Robust, ACS weak
    assert_eq!(matrix_cell(0.30, 0.20), SizingBand::Quarter);

    // Sensitive, ACS strong
    assert_eq!(matrix_cell(0.80, 0.10), SizingBand::Half);
    // Sensitive, ACS weak
    assert_eq!(matrix_cell(0.30, 0.10), SizingBand::Kill);

    // Fragile, ACS strong
    assert_eq!(matrix_cell(0.80, 0.02), SizingBand::Quarter);
    // Fragile, ACS weak
    assert_eq!(matrix_cell(0.30, 0.02), SizingBand::Kill);
}

#[test]
fn test_kelly_sizing_cap() {
    let p = 0.20;
    let b = 4.9; // 5.9x blended return means net odds = 4.9
    let k = 0.25; // Quarter Kelly
    let m = SizingBand::Quarter; // Quarter multiplier (0.25)
    let ruin_cap = Cents::from_dollars(5.0); // limit commitment to $5
    let bankroll = Cents::from_dollars(1000.0);

    // f* = (0.20 * 4.9 - 0.8) / 4.9 = (0.98 - 0.8) / 4.9 = 0.18 / 4.9 = 0.03673
    // sizing_fraction = 0.03673 * 0.25 * 0.25 = 0.0022959
    // proposed_amount = 0.0022959 * 1000 = 2.2959
    // Since proposed_amount < ruin_cap, it should return ~2.2959
    let amount = committed_fraction(p, b, k, m, ruin_cap, bankroll);
    assert_eq!(amount, Cents(230)); // 229.59 cents rounded to 230 cents ($2.30)

    // If we decrease ruin_cap to 1.0, it should be clamped to 1.0
    let amount_capped = committed_fraction(p, b, k, m, Cents::from_dollars(1.0), bankroll);
    assert_eq!(amount_capped, Cents::from_dollars(1.0));
}

#[test]
fn test_allocate_ladder_success() {
    let total = Cents::from_dollars(10.00);
    let legs = vec![
        LegSpec {
            name: "Spain Winner".to_string(),
            leg_type: LegType::Terminal,
            proportion: 0.50,
            allocated_amount: None,
        },
        LegSpec {
            name: "Spain Semis".to_string(),
            leg_type: LegType::Checkpoint,
            proportion: 0.40,
            allocated_amount: None,
        },
        LegSpec {
            name: "ESP vs CPV".to_string(),
            leg_type: LegType::SingleEvent,
            proportion: 0.10,
            allocated_amount: None,
        },
    ];

    let ladder = allocate_ladder(total, &legs).unwrap();
    assert_eq!(ladder.total, Cents::from_dollars(10.00));
    assert_eq!(ladder.legs[0].allocated_amount, Some(Cents::from_dollars(5.00)));
    assert_eq!(ladder.legs[1].allocated_amount, Some(Cents::from_dollars(4.00)));
    assert_eq!(ladder.legs[2].allocated_amount, Some(Cents::from_dollars(1.00)));
}

#[test]
fn test_allocate_ladder_rounding() {
    let total = Cents::from_dollars(10.00);
    // Let's create proportions that require rounding distribution
    let legs = vec![
        LegSpec {
            name: "Spain Winner".to_string(),
            leg_type: LegType::Terminal,
            proportion: 0.501,
            allocated_amount: None,
        },
        LegSpec {
            name: "Spain Semis".to_string(),
            leg_type: LegType::Checkpoint,
            proportion: 0.401,
            allocated_amount: None,
        },
        LegSpec {
            name: "ESP vs CPV".to_string(),
            leg_type: LegType::SingleEvent,
            proportion: 0.098,
            allocated_amount: None,
        },
    ];

    let ladder = allocate_ladder(total, &legs).unwrap();
    let mut sum = Cents::ZERO;
    for leg in &ladder.legs {
        sum = sum + leg.allocated_amount.unwrap();
    }
    assert_eq!(sum, Cents::from_dollars(10.00));
}

#[test]
fn test_allocate_ladder_errors() {
    let total = Cents::from_dollars(10.00);
    
    // 1. Single-event leg > 10% of total
    let legs1 = vec![
        LegSpec {
            name: "Spain Winner".to_string(),
            leg_type: LegType::Terminal,
            proportion: 0.45,
            allocated_amount: None,
        },
        LegSpec {
            name: "Spain Semis".to_string(),
            leg_type: LegType::Checkpoint,
            proportion: 0.40,
            allocated_amount: None,
        },
        LegSpec {
            name: "ESP vs CPV".to_string(),
            leg_type: LegType::SingleEvent,
            proportion: 0.15, // 15% > 10%
            allocated_amount: None,
        },
    ];
    let err1 = allocate_ladder(total, &legs1).unwrap_err();
    assert!(matches!(err1, AllocError::SingleEventBoundViolated { .. }));

    // 2. Terminal sum < 40% of total
    let legs2 = vec![
        LegSpec {
            name: "Spain Winner".to_string(),
            leg_type: LegType::Terminal,
            proportion: 0.35, // 35% < 40%
            allocated_amount: None,
        },
        LegSpec {
            name: "Spain Semis".to_string(),
            leg_type: LegType::Checkpoint,
            proportion: 0.55,
            allocated_amount: None,
        },
        LegSpec {
            name: "ESP vs CPV".to_string(),
            leg_type: LegType::SingleEvent,
            proportion: 0.10,
            allocated_amount: None,
        },
    ];
    let err2 = allocate_ladder(total, &legs2).unwrap_err();
    assert!(matches!(err2, AllocError::TerminalBoundViolated { .. }));
}

#[test]
fn test_content_hashing() {
    use pdp_core::{calculate_hash, InvalidationCondition};
    let condition1 = InvalidationCondition {
        condition_text: "Yamal AND Nico Williams both miss game time".to_string(),
        observable_inputs: vec!["yamal_injury".to_string(), "williams_injury".to_string()],
        exit_action: "France becomes pick".to_string(),
        status: "Armed".to_string(),
    };
    let condition2 = condition1.clone();
    
    let hash1 = calculate_hash(&condition1).unwrap();
    let hash2 = calculate_hash(&condition2).unwrap();
    assert_eq!(hash1, hash2);

    let mut condition3 = condition1.clone();
    condition3.status = "Triggered".to_string();
    let hash3 = calculate_hash(&condition3).unwrap();
    assert_ne!(hash1, hash3);
}
