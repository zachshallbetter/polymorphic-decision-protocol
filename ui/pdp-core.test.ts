import test from 'node:test';
import assert from 'node:assert';
import {
  acs,
  scoreModel,
  flipDistance,
  matrixCell,
  committedFraction,
  allocateLadder,
  calculateHash,
  SumMismatchError,
  TerminalBoundViolatedError,
  SingleEventBoundViolatedError,
  Cents
} from './pdp-core.ts';
import type {
  AgentVerdict,
  ScoreDimension,
  LegSpec
} from './pdp-core.ts';

test('ACS unanimous cross-vendor same evidence class', () => {
  const agents: AgentVerdict[] = [
    {
      agent_id: 'claude-1',
      vendor: 'Anthropic',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    },
    {
      agent_id: 'gemini-1',
      vendor: 'Google',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    },
    {
      agent_id: 'gpt-1',
      vendor: 'OpenAI',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    }
  ];

  const result = acs(agents);
  assert.ok(Math.abs(result.score - 0.5) < 1e-9);
  assert.strictEqual(result.agreeing_agents.length, 3);
});

test('ACS unanimous cross-vendor cross evidence class', () => {
  const agents: AgentVerdict[] = [
    {
      agent_id: 'claude-1',
      vendor: 'Anthropic',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    },
    {
      agent_id: 'gemini-1',
      vendor: 'Google',
      evidence_class: 'Market',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    },
    {
      agent_id: 'gpt-1',
      vendor: 'OpenAI',
      evidence_class: 'Structural',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    }
  ];

  const result = acs(agents);
  assert.ok(Math.abs(result.score - 1.0) < 1e-9);
});

test('ACS one analyst, N hats (correlated)', () => {
  const agents: AgentVerdict[] = [
    {
      agent_id: 'gpt-1',
      vendor: 'OpenAI',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    },
    {
      agent_id: 'gpt-2',
      vendor: 'OpenAI',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    },
    {
      agent_id: 'gpt-3',
      vendor: 'OpenAI',
      evidence_class: 'Quant',
      pick: 'Spain',
      stated_p: 0.20,
      alternatives: [],
      load_bearing_facts: [],
      searched: ''
    }
  ];

  const result = acs(agents);
  assert.ok(Math.abs(result.score - 0.2) < 1e-9);
});

test('Flip distance Spain/France case', () => {
  const dimensions: ScoreDimension[] = [
    {
      name: 'Attack Peak',
      weight: 0.15,
      scores: { Spain: 8.0, France: 10.0 },
      justification: ''
    },
    {
      name: 'Attack Depth',
      weight: 0.15,
      scores: { Spain: 9.0, France: 8.0 },
      justification: ''
    },
    {
      name: 'Defense',
      weight: 0.20,
      scores: { Spain: 10.0, France: 7.0 },
      justification: ''
    },
    {
      name: 'Recent Form',
      weight: 0.15,
      scores: { Spain: 9.0, France: 8.0 },
      justification: ''
    },
    {
      name: 'WC Pedigree',
      weight: 0.10,
      scores: { Spain: 5.0, France: 8.0 },
      justification: ''
    },
    {
      name: 'Cohesion',
      weight: 0.10,
      scores: { Spain: 10.0, France: 8.0 },
      justification: ''
    },
    {
      name: 'Injury Risk',
      weight: 0.10,
      scores: { Spain: 9.0, France: 6.0 },
      justification: ''
    },
    {
      name: 'Experience',
      weight: 0.05,
      scores: { Spain: 7.0, France: 9.0 },
      justification: ''
    }
  ];

  const model = scoreModel(dimensions);
  assert.ok(Math.abs(model.option_scores['Spain'] - 8.65) < 1e-9);
  assert.ok(Math.abs(model.option_scores['France'] - 7.95) < 1e-9);

  const fd = flipDistance(model);
  assert.ok(fd !== null);
  if (!fd) throw new Error('Flip distance should not be null');
  assert.strictEqual(fd.sensitive_dimension, 'WC Pedigree');
  assert.ok(Math.abs(fd.distance - 0.17027027) < 1e-6);
});

test('Matrix Cell lookup', () => {
  assert.strictEqual(matrixCell(0.80, 0.20), 'Full');
  assert.strictEqual(matrixCell(0.50, 0.20), 'Half');
  assert.strictEqual(matrixCell(0.30, 0.20), 'Quarter');

  assert.strictEqual(matrixCell(0.80, 0.10), 'Half');
  assert.strictEqual(matrixCell(0.30, 0.10), 'Kill');

  assert.strictEqual(matrixCell(0.80, 0.02), 'Quarter');
  assert.strictEqual(matrixCell(0.30, 0.02), 'Kill');
});

test('Kelly sizing cap', () => {
  const p = 0.20;
  const b = 4.9;
  const k = 0.25;
  const m = 'Quarter';
  const ruinCap = Cents.fromDollars(5.0);
  const bankroll = Cents.fromDollars(1000.0);

  const amount = committedFraction(p, b, k, m, ruinCap, bankroll);
  assert.strictEqual(amount.raw, 230); // 229.59 cents rounded to 230 cents ($2.30)

  const amountCapped = committedFraction(p, b, k, m, Cents.fromDollars(1.0), bankroll);
  assert.strictEqual(amountCapped.raw, 100);
});

test('Ladder allocation success', () => {
  const total = Cents.fromDollars(10.00);
  const legs: LegSpec[] = [
    {
      name: 'Spain Winner',
      leg_type: 'Terminal',
      proportion: 0.50
    },
    {
      name: 'Spain Semis',
      leg_type: 'Checkpoint',
      proportion: 0.40
    },
    {
      name: 'ESP vs CPV',
      leg_type: 'SingleEvent',
      proportion: 0.10
    }
  ];

  const ladder = allocateLadder(total, legs);
  assert.strictEqual(ladder.total.raw, 1000);
  assert.strictEqual(ladder.legs[0].allocated_amount?.raw, 500);
  assert.strictEqual(ladder.legs[1].allocated_amount?.raw, 400);
  assert.strictEqual(ladder.legs[2].allocated_amount?.raw, 100);
});

test('Ladder allocation rounding', () => {
  const total = Cents.fromDollars(10.00);
  const legs: LegSpec[] = [
    {
      name: 'Spain Winner',
      leg_type: 'Terminal',
      proportion: 0.501
    },
    {
      name: 'Spain Semis',
      leg_type: 'Checkpoint',
      proportion: 0.401
    },
    {
      name: 'ESP vs CPV',
      leg_type: 'SingleEvent',
      proportion: 0.098
    }
  ];

  const ladder = allocateLadder(total, legs);
  const sum = ladder.legs.reduce((acc, l) => acc + (l.allocated_amount?.raw || 0), 0);
  assert.strictEqual(sum, 1000);
});

test('Ladder allocation errors', () => {
  const total = Cents.fromDollars(10.00);

  // Single-event > 10%
  const legs1: LegSpec[] = [
    { name: 'Spain Winner', leg_type: 'Terminal', proportion: 0.45 },
    { name: 'Spain Semis', leg_type: 'Checkpoint', proportion: 0.40 },
    { name: 'ESP vs CPV', leg_type: 'SingleEvent', proportion: 0.15 }
  ];
  assert.throws(() => allocateLadder(total, legs1), SingleEventBoundViolatedError);

  // Terminal < 40%
  const legs2: LegSpec[] = [
    { name: 'Spain Winner', leg_type: 'Terminal', proportion: 0.35 },
    { name: 'Spain Semis', leg_type: 'Checkpoint', proportion: 0.55 },
    { name: 'ESP vs CPV', leg_type: 'SingleEvent', proportion: 0.10 }
  ];
  assert.throws(() => allocateLadder(total, legs2), TerminalBoundViolatedError);
});

test('SHA-256 deterministic content hashing', async () => {
  const condition1 = {
    condition_text: "Yamal AND Nico Williams both miss game time",
    observable_inputs: ["yamal_injury", "williams_injury"],
    exit_action: "France becomes pick",
    status: "Armed"
  };
  const hash1 = await calculateHash(condition1);
  const hash2 = await calculateHash({ ...condition1 });
  assert.strictEqual(hash1, hash2);

  const hash3 = await calculateHash({ ...condition1, status: "Triggered" });
  assert.notStrictEqual(hash1, hash3);
});
