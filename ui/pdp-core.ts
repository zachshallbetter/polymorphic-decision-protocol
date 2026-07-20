export type Vendor = 'Anthropic' | 'OpenAI' | 'Google';
export type EvidenceClass = 'Quant' | 'Market' | 'Structural' | 'Contrarian';

export class Cents {
  readonly raw: number;

  constructor(raw: number) {
    if (!Number.isInteger(raw)) {
      throw new Error(`Cents value must be an integer, got: ${raw}`);
    }
    this.raw = raw;
  }

  static ZERO = new Cents(0);

  static fromDollars(dollars: number): Cents {
    return new Cents(Math.round(dollars * 100));
  }

  toDollars(): number {
    return this.raw / 100;
  }

  toString(): string {
    return `$${this.toDollars().toFixed(2)}`;
  }

  add(other: Cents): Cents {
    return new Cents(this.raw + other.raw);
  }

  sub(other: Cents): Cents {
    return new Cents(Math.max(0, this.raw - other.raw));
  }

  mul(factor: number): Cents {
    if (factor <= 0) return Cents.ZERO;
    return new Cents(Math.round(this.raw * factor));
  }

  div(divisor: number): Cents {
    if (divisor <= 0) return Cents.ZERO;
    return new Cents(Math.round(this.raw / divisor));
  }

  toJSON(): number {
    return this.raw;
  }
}

export interface AgentVerdict {
  agent_id: string;
  vendor: Vendor;
  evidence_class: EvidenceClass;
  pick: string;
  stated_p: number;
  alternatives: [string, number][];
  load_bearing_facts: string[];
  searched: string;
}

export interface Acs {
  score: number;
  agreeing_agents: string[];
  total_agents: number;
}

export interface FlipDistance {
  distance: number;
  sensitive_dimension: string | null;
}

export interface ScoreDimension {
  name: string;
  weight: number;
  scores: Record<string, number>;
  justification: string;
}

export interface ScoredModel {
  dimensions: ScoreDimension[];
  option_scores: Record<string, number>;
}

export type SizingBand = 'Full' | 'Half' | 'Quarter' | 'Kill';
export type LegType = 'Terminal' | 'Checkpoint' | 'SingleEvent';

export interface LegSpec {
  name: string;
  leg_type: LegType;
  proportion: number;
  allocated_amount?: Cents;
}

export interface Ladder {
  total: Cents;
  legs: LegSpec[];
}

export interface InvalidationCondition {
  condition_text: string;
  observable_inputs: string[];
  exit_action: string;
  status: 'Armed' | 'Triggered' | 'Expired';
}

export interface KellyInputs {
  p: number;
  b: number;
  k: number;
  bankroll: Cents;
  ruin_cap: Cents;
}

export interface CommitmentStructure {
  ladder: Ladder;
  total_committed: Cents;
  matrix_cell: SizingBand;
  kelly_inputs: KellyInputs;
  confirmed_by: string;
}

export interface PreCommittedExit {
  action_text: string;
  target_asset: string;
}

export interface Outcome {
  winning_pick: string;
  details: string;
}

export interface ProcessScore {
  panel_decorrelation: number;
  elicitation_quality: number;
  self_critique_before_pressure: number;
  weights_frozen: number;
  sensitivity_analysis: number;
  invalidation_quality: number;
  entry_discipline: number;
  lock_integrity: number;
  total_score: number;
}

export type Gate = 'G1' | 'G2' | 'G3';

export type DecisionState =
  | 'Draft'
  | 'Eliciting'
  | { G1Pending: { acs: Acs } }
  | 'StressTesting'
  | { G2Pending: { flip_distance: FlipDistance; invalidation: InvalidationCondition } }
  | 'ComposingStructure'
  | { G3Pending: { proposal: CommitmentStructure } }
  | { Locked: { structure: CommitmentStructure; invalidation: InvalidationCondition } }
  | { InvalidationTriggered: { exit: PreCommittedExit } }
  | { Settled: { outcome: Outcome } }
  | { Audited: { process_score: ProcessScore } }
  | { Killed: { at_gate: Gate; reason: string } };

// --- Error definitions for Ladder Allocation ---

export class SumMismatchError extends Error {
  expected: number;
  actual: number;
  constructor(expected: number, actual: number) {
    super(`Allocated sum mismatch: expected ${expected}, got ${actual}`);
    this.expected = expected;
    this.actual = actual;
    this.name = 'SumMismatchError';
  }
}

export class TerminalBoundViolatedError extends Error {
  sum: number;
  min: number;
  max: number;
  constructor(sum: number, min: number, max: number) {
    super(`Terminal allocation bound violated: sum ${sum}, must be between ${min} and ${max}`);
    this.sum = sum;
    this.min = min;
    this.max = max;
    this.name = 'TerminalBoundViolatedError';
  }
}

export class CheckpointBoundViolatedError extends Error {
  sum: number;
  min: number;
  max: number;
  constructor(sum: number, min: number, max: number) {
    super(`Checkpoint allocation bound violated: sum ${sum}, must be between ${min} and ${max}`);
    this.sum = sum;
    this.min = min;
    this.max = max;
    this.name = 'CheckpointBoundViolatedError';
  }
}

export class SingleEventBoundViolatedError extends Error {
  legName: string;
  amount: number;
  max: number;
  constructor(legName: string, amount: number, max: number) {
    super(`Single-event leg '${legName}' allocation (${amount}) exceeds max allowed (${max})`);
    this.legName = legName;
    this.amount = amount;
    this.max = max;
    this.name = 'SingleEventBoundViolatedError';
  }
}

// --- Hashing and Cryptography ---

export async function calculateHash(value: any): Promise<string> {
  const serialized = JSON.stringify(value);
  const encoder = new TextEncoder();
  const data = encoder.encode(serialized);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// --- Mathematical Logic Functions ---

export function acs(agents: AgentVerdict[]): Acs {
  if (agents.length === 0) {
    return {
      score: 0.0,
      agreeing_agents: [],
      total_agents: 0,
    };
  }

  const pickCounts: Record<string, number> = {};
  for (const agent of agents) {
    pickCounts[agent.pick] = (pickCounts[agent.pick] || 0) + 1;
  }

  let modalPick = '';
  let maxCount = 0;
  for (const pick in pickCounts) {
    if (pickCounts[pick] > maxCount) {
      maxCount = pickCounts[pick];
      modalPick = pick;
    }
  }

  const agreeingAgents = agents.filter(a => a.pick === modalPick);
  const agreeingIds = agreeingAgents.map(a => a.agent_id);

  let numerator = 0.0;
  for (const agent of agreeingAgents) {
    let sharesBoth = false;
    let sharesOne = false;

    for (const other of agreeingAgents) {
      if (agent.agent_id === other.agent_id) {
        continue;
      }
      const sameVendor = agent.vendor === other.vendor;
      const sameEvidence = agent.evidence_class === other.evidence_class;

      if (sameVendor && sameEvidence) {
        sharesBoth = true;
      } else if (sameVendor || sameEvidence) {
        sharesOne = true;
      }
    }

    const weight = sharesBoth ? 0.2 : sharesOne ? 0.5 : 1.0;
    numerator += weight;
  }

  const score = numerator / agents.length;

  return {
    score,
    agreeing_agents: agreeingIds,
    total_agents: agents.length,
  };
}

export function scoreModel(dimensions: ScoreDimension[]): ScoredModel {
  const optionScores: Record<string, number> = {};
  const options = new Set<string>();

  for (const dim of dimensions) {
    for (const option in dim.scores) {
      options.add(option);
    }
  }

  for (const option of options) {
    let totalScore = 0.0;
    for (const dim of dimensions) {
      const score = dim.scores[option] !== undefined ? dim.scores[option] : 0.0;
      totalScore += dim.weight * score;
    }
    optionScores[option] = totalScore;
  }

  return {
    dimensions,
    option_scores: optionScores,
  };
}

export function flipDistance(model: ScoredModel): FlipDistance | null {
  const optionScores = Object.entries(model.option_scores);
  if (optionScores.length < 2) {
    return null;
  }

  optionScores.sort((a, b) => b[1] - a[1]);

  const topOption = optionScores[0][0];
  const secondOption = optionScores[1][0];

  const scoreA = optionScores[0][1];
  const scoreB = optionScores[1][1];
  const dOverall = scoreA - scoreB;

  if (dOverall <= 0.0) {
    return {
      distance: 0.0,
      sensitive_dimension: null,
    };
  }

  let minAbsDelta = Infinity;
  let bestDim: string | null = null;

  for (const dim of model.dimensions) {
    const wK = dim.weight;
    if (wK >= 1.0) {
      continue;
    }

    const sA = dim.scores[topOption] !== undefined ? dim.scores[topOption] : 0.0;
    const sB = dim.scores[secondOption] !== undefined ? dim.scores[secondOption] : 0.0;
    const dK = sA - sB;

    const c = (dK - dOverall) / (1.0 - wK);

    if (c > 0.0) {
      const requiredDelta = -dOverall / c;
      if (requiredDelta >= -wK) {
        const absDelta = Math.abs(requiredDelta);
        if (absDelta < minAbsDelta) {
          minAbsDelta = absDelta;
          bestDim = dim.name;
        }
      }
    } else if (c < 0.0) {
      const requiredDelta = -dOverall / c;
      if (requiredDelta <= 1.0 - wK) {
        const absDelta = Math.abs(requiredDelta);
        if (absDelta < minAbsDelta) {
          minAbsDelta = absDelta;
          bestDim = dim.name;
        }
      }
    }
  }

  if (minAbsDelta === Infinity) {
    return null;
  }

  return {
    distance: minAbsDelta,
    sensitive_dimension: bestDim,
  };
}

export function matrixCell(acsVal: number, fdVal: number): SizingBand {
  if (fdVal > 0.15) {
    if (acsVal >= 0.75) return 'Full';
    if (acsVal >= 0.40) return 'Half';
    return 'Quarter';
  } else if (fdVal >= 0.05) {
    if (acsVal >= 0.75) return 'Half';
    if (acsVal >= 0.40) return 'Quarter';
    return 'Kill';
  } else {
    if (acsVal >= 0.75) return 'Quarter';
    return 'Kill';
  }
}

export function committedFraction(
  p: number,
  b: number,
  k: number,
  m: SizingBand,
  ruinCap: Cents,
  bankroll: Cents
): Cents {
  if (b <= 0.0) {
    return Cents.ZERO;
  }
  const q = 1.0 - p;
  const fStar = (p * b - q) / b;
  if (fStar <= 0.0) {
    return Cents.ZERO;
  }

  const mMultiplier = m === 'Full' ? 1.0 : m === 'Half' ? 0.5 : m === 'Quarter' ? 0.25 : 0.0;
  const fraction = fStar * k * mMultiplier;
  const proposedAmount = bankroll.mul(fraction);

  return proposedAmount.raw > ruinCap.raw ? ruinCap : proposedAmount;
}

export function allocateLadder(total: Cents, legs: LegSpec[]): Ladder {
  if (legs.length === 0) {
    if (total.raw === 0) {
      return { total, legs: [] };
    } else {
      throw new SumMismatchError(total.toDollars(), 0.0);
    }
  }

  const totalCents = total.raw;
  const allocatedCents: number[] = [];
  let sumCents = 0;

  for (const leg of legs) {
    const cents = Math.round(totalCents * leg.proportion);
    allocatedCents.push(cents);
    sumCents += cents;
  }

  let remainder = totalCents - sumCents;
  if (remainder !== 0) {
    const step = Math.sign(remainder);
    let remToDistribute = Math.abs(remainder);
    let idx = 0;
    while (remToDistribute > 0) {
      allocatedCents[idx % legs.length] += step;
      remToDistribute -= 1;
      idx += 1;
    }
  }

  const finalLegs: LegSpec[] = [];
  let terminalSum = Cents.ZERO;
  let checkpointSum = Cents.ZERO;

  for (let i = 0; i < legs.length; i++) {
    const amount = new Cents(allocatedCents[i]);
    const leg = legs[i];

    if (leg.leg_type === 'Terminal') {
      terminalSum = terminalSum.add(amount);
    } else if (leg.leg_type === 'Checkpoint') {
      checkpointSum = checkpointSum.add(amount);
    } else if (leg.leg_type === 'SingleEvent') {
      const maxAllowed = total.mul(0.10);
      if (amount.raw > maxAllowed.raw) {
        throw new SingleEventBoundViolatedError(leg.name, amount.toDollars(), maxAllowed.toDollars());
      }
    }

    finalLegs.push({
      ...leg,
      allocated_amount: amount,
    });
  }

  let actualSum = Cents.ZERO;
  for (const leg of finalLegs) {
    actualSum = actualSum.add(leg.allocated_amount || Cents.ZERO);
  }
  if (actualSum.raw !== total.raw) {
    throw new SumMismatchError(total.toDollars(), actualSum.toDollars());
  }

  const minTerminal = total.mul(0.40);
  const maxTerminal = total.mul(0.60);
  if (terminalSum.raw < minTerminal.raw || terminalSum.raw > maxTerminal.raw) {
    throw new TerminalBoundViolatedError(terminalSum.toDollars(), minTerminal.toDollars(), maxTerminal.toDollars());
  }

  const minCheckpoint = total.mul(0.40);
  const maxCheckpoint = total.mul(0.60);
  if (checkpointSum.raw < minCheckpoint.raw || checkpointSum.raw > maxCheckpoint.raw) {
    throw new CheckpointBoundViolatedError(checkpointSum.toDollars(), minCheckpoint.toDollars(), maxCheckpoint.toDollars());
  }

  return {
    total,
    legs: finalLegs,
  };
}
