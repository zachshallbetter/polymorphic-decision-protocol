use crate::{Acs, AgentVerdict, FlipDistance, ScoredModel, SizingBand, LegSpec, LegType, Ladder, cents::Cents};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AllocError {
    SumMismatch { expected: f64, actual: f64 },
    TerminalBoundViolated { sum: f64, min: f64, max: f64 },
    CheckpointBoundViolated { sum: f64, min: f64, max: f64 },
    SingleEventBoundViolated { name: String, amount: f64, max: f64 },
}

impl std::fmt::Display for AllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocError::SumMismatch { expected, actual } => {
                write!(f, "Allocated sum mismatch: expected {}, got {}", expected, actual)
            }
            AllocError::TerminalBoundViolated { sum, min, max } => {
                write!(f, "Terminal allocation bound violated: sum {}, must be between {} and {}", sum, min, max)
            }
            AllocError::CheckpointBoundViolated { sum, min, max } => {
                write!(f, "Checkpoint allocation bound violated: sum {}, must be between {} and {}", sum, min, max)
            }
            AllocError::SingleEventBoundViolated { name, amount, max } => {
                write!(f, "Single-event leg '{}' allocation ({}) exceeds max allowed ({})", name, amount, max)
            }
        }
    }
}

impl std::error::Error for AllocError {}

/// Calculates the Adjusted Convergence Score (ACS) for a panel of agents.
/// Under PDP v1.3:
/// ACS = Σ_i (agreement_i * decorrelation_weight_i) / N
/// where N is the total number of agents in the panel.
pub fn acs(agents: &[AgentVerdict]) -> Acs {
    if agents.is_empty() {
        return Acs {
            score: 0.0,
            agreeing_agents: Vec::new(),
            total_agents: 0,
        };
    }

    // 1. Count picks to find the modal pick
    let mut pick_counts = HashMap::new();
    for agent in agents {
        *pick_counts.entry(&agent.pick).or_insert(0) += 1;
    }

    let mut modal_pick = String::new();
    let mut max_count = 0;
    for (pick, count) in pick_counts {
        if count > max_count {
            max_count = count;
            modal_pick = pick.clone();
        }
    }

    // 2. Identify agreeing agents
    let agreeing_agents: Vec<&AgentVerdict> = agents
        .iter()
        .filter(|a| a.pick == modal_pick)
        .collect();

    let agreeing_ids: Vec<String> = agreeing_agents.iter().map(|a| a.agent_id.clone()).collect();

    // 3. Compute weight for each agreeing agent
    let mut numerator = 0.0;
    for agent in &agreeing_agents {
        let mut shares_both = false;
        let mut shares_one = false;

        for other in &agreeing_agents {
            if agent.agent_id == other.agent_id {
                continue;
            }
            let same_vendor = agent.vendor == other.vendor;
            let same_evidence = agent.evidence_class == other.evidence_class;

            if same_vendor && same_evidence {
                shares_both = true;
            } else if same_vendor || same_evidence {
                shares_one = true;
            }
        }

        let weight = if shares_both {
            0.2
        } else if shares_one {
            0.5
        } else {
            1.0
        };

        numerator += weight;
    }

    let score = numerator / agents.len() as f64;

    Acs {
        score,
        agreeing_agents: agreeing_ids,
        total_agents: agents.len(),
    }
}

/// Computes the scored model from dimensions.
pub fn score_model(dimensions: &[crate::ScoreDimension]) -> ScoredModel {
    let mut option_scores = HashMap::new();
    let mut options = std::collections::HashSet::new();

    for dim in dimensions {
        for option in dim.scores.keys() {
            options.insert(option.clone());
        }
    }

    for option in options {
        let mut total_score = 0.0;
        for dim in dimensions {
            let score = dim.scores.get(&option).cloned().unwrap_or(0.0);
            total_score += dim.weight * score;
        }
        option_scores.insert(option, total_score);
    }

    ScoredModel {
        dimensions: dimensions.to_vec(),
        option_scores,
    }
}

/// Calculates the Flip Distance of a ScoredModel.
/// Flip Distance = min single-weight perturbation (redistributed pro-rata) reversing the top-2 ranking.
pub fn flip_distance(model: &ScoredModel) -> Option<FlipDistance> {
    if model.option_scores.len() < 2 {
        return None;
    }

    // Sort options to find top-2
    let mut sorted_options: Vec<(&String, &f64)> = model.option_scores.iter().collect();
    sorted_options.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    let top_option = sorted_options[0].0;
    let second_option = sorted_options[1].0;

    let score_a = *sorted_options[0].1;
    let score_b = *sorted_options[1].1;
    let d_overall = score_a - score_b;

    if d_overall <= 0.0 {
        // Already flipped or tied
        return Some(FlipDistance {
            distance: 0.0,
            sensitive_dimension: None,
        });
    }

    let mut min_abs_delta = f64::INFINITY;
    let mut best_dim = None;

    for dim in &model.dimensions {
        let w_k = dim.weight;
        if w_k >= 1.0 {
            continue;
        }

        let s_a = dim.scores.get(top_option).cloned().unwrap_or(0.0);
        let s_b = dim.scores.get(second_option).cloned().unwrap_or(0.0);
        let d_k = s_a - s_b;

        // C = (d_k - d_overall) / (1.0 - w_k)
        let c = (d_k - d_overall) / (1.0 - w_k);

        if c > 0.0 {
            // we need delta <= -d_overall / c
            let required_delta = -d_overall / c;
            if required_delta >= -w_k {
                let abs_delta = required_delta.abs();
                if abs_delta < min_abs_delta {
                    min_abs_delta = abs_delta;
                    best_dim = Some(dim.name.clone());
                }
            }
        } else if c < 0.0 {
            // we need delta >= -d_overall / c
            let required_delta = -d_overall / c;
            if required_delta <= 1.0 - w_k {
                let abs_delta = required_delta.abs();
                if abs_delta < min_abs_delta {
                    min_abs_delta = abs_delta;
                    best_dim = Some(dim.name.clone());
                }
            }
        }
    }

    if min_abs_delta.is_infinite() {
        None
    } else {
        Some(FlipDistance {
            distance: min_abs_delta,
            sensitive_dimension: best_dim,
        })
    }
}

/// Matches the ACS and Flip Distance values against the decision matrix to return the SizingBand.
pub fn matrix_cell(acs_val: f64, fd_val: f64) -> SizingBand {
    if fd_val > 0.15 {
        // Robust
        if acs_val >= 0.75 {
            SizingBand::Full
        } else if acs_val >= 0.40 {
            SizingBand::Half
        } else {
            SizingBand::Quarter
        }
    } else if fd_val >= 0.05 {
        // Sensitive
        if acs_val >= 0.75 {
            SizingBand::Half
        } else if acs_val >= 0.40 {
            SizingBand::Quarter
        } else {
            SizingBand::Kill
        }
    } else {
        // Fragile
        if acs_val >= 0.75 {
            SizingBand::Quarter
        } else {
            SizingBand::Kill
        }
    }
}

/// Computes the committed fraction (monetary amount) based on Fractional Kelly with ruin cap.
pub fn committed_fraction(
    p: f64,
    b: f64,
    k: f64,
    m: SizingBand,
    ruin_cap: Cents,
    bankroll: Cents,
) -> Cents {
    if b <= 0.0 {
        return Cents::ZERO;
    }
    let q = 1.0 - p;
    let f_star = (p * b - q) / b;
    if f_star <= 0.0 {
        return Cents::ZERO;
    }

    let m_multiplier = match m {
        SizingBand::Full => 1.0,
        SizingBand::Half => 0.5,
        SizingBand::Quarter => 0.25,
        SizingBand::Kill => 0.0,
    };

    let fraction = f_star * k * m_multiplier;
    let proposed_amount = bankroll * fraction;

    if proposed_amount > ruin_cap {
        ruin_cap
    } else {
        proposed_amount
    }
}

/// Allocates the total commitment amount to ladder legs using integer-cent rounding.
/// Validates constraints:
/// - Terminal legs sum is 40%-60% of total
/// - Checkpoint legs sum is 40%-60% of total
/// - Single-event legs are each <= 10% of total
pub fn allocate_ladder(total: Cents, legs: &[LegSpec]) -> Result<Ladder, AllocError> {
    if legs.is_empty() {
        if total == Cents::ZERO {
            return Ok(Ladder { total, legs: Vec::new() });
        } else {
            return Err(AllocError::SumMismatch { expected: total.to_dollars(), actual: 0.0 });
        }
    }

    let total_cents = total.0;
    let mut allocated_cents = Vec::new();
    let mut sum_cents = 0;

    for leg in legs {
        let cents = (total_cents as f64 * leg.proportion).round() as u64;
        allocated_cents.push(cents);
        sum_cents += cents;
    }

    let remainder = total_cents as i64 - sum_cents as i64;
    if remainder != 0 {
        let step = remainder.signum();
        let mut rem_to_distribute = remainder.abs();
        let mut idx = 0;
        while rem_to_distribute > 0 {
            if step > 0 {
                allocated_cents[idx % legs.len()] += 1;
            } else {
                allocated_cents[idx % legs.len()] = allocated_cents[idx % legs.len()].saturating_sub(1);
            }
            rem_to_distribute -= 1;
            idx += 1;
        }
    }

    let mut final_legs = Vec::new();
    let mut terminal_sum = Cents::ZERO;
    let mut checkpoint_sum = Cents::ZERO;

    for (i, leg) in legs.iter().enumerate() {
        let amount = Cents(allocated_cents[i]);

        match leg.leg_type {
            LegType::Terminal => terminal_sum = terminal_sum + amount,
            LegType::Checkpoint => checkpoint_sum = checkpoint_sum + amount,
            LegType::SingleEvent => {
                let max_allowed = total * 0.10;
                if amount > max_allowed {
                    return Err(AllocError::SingleEventBoundViolated {
                        name: leg.name.clone(),
                        amount: amount.to_dollars(),
                        max: max_allowed.to_dollars(),
                    });
                }
            }
        }

        let mut final_leg = leg.clone();
        final_leg.allocated_amount = Some(amount);
        final_legs.push(final_leg);
    }

    // Sum verification
    let mut actual_sum = Cents::ZERO;
    for leg in &final_legs {
        actual_sum = actual_sum + leg.allocated_amount.unwrap_or(Cents::ZERO);
    }
    if actual_sum != total {
        return Err(AllocError::SumMismatch { expected: total.to_dollars(), actual: actual_sum.to_dollars() });
    }

    let min_terminal = total * 0.40;
    let max_terminal = total * 0.60;
    if terminal_sum < min_terminal || terminal_sum > max_terminal {
        return Err(AllocError::TerminalBoundViolated {
            sum: terminal_sum.to_dollars(),
            min: min_terminal.to_dollars(),
            max: max_terminal.to_dollars(),
        });
    }

    let min_checkpoint = total * 0.40;
    let max_checkpoint = total * 0.60;
    if checkpoint_sum < min_checkpoint || checkpoint_sum > max_checkpoint {
        return Err(AllocError::CheckpointBoundViolated {
            sum: checkpoint_sum.to_dollars(),
            min: min_checkpoint.to_dollars(),
            max: max_checkpoint.to_dollars(),
        });
    }

    Ok(Ladder {
        total,
        legs: final_legs,
    })
}
