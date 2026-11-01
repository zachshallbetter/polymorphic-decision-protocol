'use strict';

const roles = document.querySelector('#roles');
document.querySelectorAll('[name="fit"]').forEach((input) => input.addEventListener('change', () => {
  const supported = document.querySelector('[name="fit"]:checked').value === 'supported';
  roles.hidden = !supported;
  roles.querySelectorAll('input').forEach((field) => { field.required = supported; });
}));

const tabs = [...document.querySelectorAll('[role="tab"]')];
const activateTab = (tab, focus = false) => {
  for (const item of tabs) {
    const selected = item === tab;
    item.setAttribute('aria-selected', String(selected));
    item.tabIndex = selected ? 0 : -1;
    document.getElementById(item.getAttribute('aria-controls')).hidden = !selected;
  }
  if (focus) tab.focus();
};
tabs.forEach((tab, index) => {
  tab.addEventListener('click', () => activateTab(tab));
  tab.addEventListener('keydown', (event) => {
    let next = index;
    if (event.key === 'ArrowRight') next = (index + 1) % tabs.length;
    else if (event.key === 'ArrowLeft') next = (index + tabs.length - 1) % tabs.length;
    else if (event.key === 'Home') next = 0;
    else if (event.key === 'End') next = tabs.length - 1;
    else return;
    event.preventDefault();
    activateTab(tabs[next], true);
  });
});

const api = async (path, options = {}) => {
  const response = await fetch(path, {
    ...options,
    headers: { 'content-type': 'application/json', ...(options.headers || {}) },
  });
  if (!response.ok) {
    const raw = await response.text();
    let detail = {};
    try { detail = JSON.parse(raw); } catch {}
    const error = new Error(detail.error || raw.trim() || `Request failed (${response.status})`);
    error.providerStatus = detail.provider_status;
    throw error;
  }
  return response.status === 204 ? null : response.json();
};

api('/api/health').then((health) => {
  if (!health.preview_only) return;
  document.querySelector('[role="status"]').textContent = 'Preview-only mode · no Keychain access or external provider calls. Records exist in memory for this session; Liquid analysis stays on this Mac.';
  const panel = document.querySelector('#settings-panel');
  const section = element('section');
  section.append(
    element('h2', 'Settings paused in preview mode'),
    element('p', 'Provider credentials and API calls are disabled in this session. No Keychain items are read or changed.'),
  );
  panel.replaceChildren(section);
}).catch(() => {});

document.querySelectorAll('.credential-card').forEach((card) => {
  const provider = card.dataset.provider;
  const paused = card.dataset.paused === 'true';
  const form = card.querySelector('.credential-form');
  const input = form?.querySelector('input[type="password"]');
  const saveButton = form?.querySelector('button');
  const testButton = card.querySelector('.provider-test-button');
  const status = document.querySelector(`#${provider}-status`);
  const result = document.querySelector(`#${provider}-test-result`);

  if (form) form.addEventListener('submit', async (event) => {
    event.preventDefault();
    saveButton.disabled = true;
    status.textContent = 'Saving to macOS Keychain…';
    try {
      await api(`/api/credentials/${provider}`, {
        method: 'POST', body: JSON.stringify({ api_key: input.value }),
      });
      status.textContent = 'Key saved in macOS Keychain.';
      form.closest('details').hidden = true;
      if (testButton) testButton.disabled = paused;
    } catch (error) {
      status.textContent = `Key not saved: ${error.message}`;
    } finally {
      input.value = '';
      saveButton.disabled = false;
    }
  });

  testButton?.addEventListener('click', async () => {
    if (paused) return;
    testButton.disabled = true;
    result.textContent = 'Sending the fixed smoke-test prompt…';
    try {
      const response = await api(`/api/provider-tests/${provider}`, { method: 'POST' });
      result.textContent = `Connected. ${response.model} replied: “${response.output}” · ${response.input_tokens} input tokens · ${response.output_tokens} output tokens · estimated $${response.estimated_cost_usd.toFixed(6)}.`;
    } catch (error) {
      if (error.providerStatus === 429) {
        result.replaceChildren(element('span', `${provider === 'google' ? 'Google Gemini' : 'OpenAI'} returned HTTP 429. This indicates a provider rate or quota limit; the response does not identify which limit was reached. No automatic retry was sent. This build permits one attempt per provider per app session, so restart PDP Local before another attempt.`));
        const link = element('a', provider === 'google' ? 'Google AI Studio rate limits' : 'OpenAI API limits');
        link.href = provider === 'google'
          ? 'https://ai.google.dev/gemini-api/docs/rate-limits'
          : 'https://help.openai.com/en/articles/5955604-troubleshooting-api-rate-limits-and-429-errors';
        link.target = '_blank'; link.rel = 'noreferrer';
        result.append(element('br'), link);
      } else {
        result.textContent = `Smoke test failed: ${error.message}`;
      }
    }
  });
});

const element = (tag, text) => {
  const node = document.createElement(tag);
  if (text !== undefined) node.textContent = text;
  return node;
};

const field = (parent, labelText, multiline = false, required = true) => {
  const label = element('label', labelText);
  const input = element(multiline ? 'textarea' : 'input');
  input.required = required;
  label.append(input);
  parent.append(label);
  return input;
};

const action = async (id, operation, trigger) => {
  try {
    const actor = document.querySelector('#actor').value.trim() || 'local-human';
    await api(`/api/decisions/${encodeURIComponent(id)}/action`, {
      method: 'POST', body: JSON.stringify({ actor, action: operation }),
    });
    await refresh();
  } catch (error) {
    const message = element('p', `Action not completed: ${error.message}`);
    message.setAttribute('role', 'alert');
    trigger?.insertAdjacentElement('afterend', message);
  }
};

async function renderEvidence(article, decision) {
  const records = await api(`/api/decisions/${encodeURIComponent(decision.id)}/evidence`);
  const section = element('details');
  section.append(element('summary', `Evidence records (${records.length})`));
  for (const item of records) {
    const line = element('p', `${item.title} · ${item.publisher} · ${item.roles.join(', ')}`);
    section.append(line);
  }
  const laneNames = { quant: 'Quant', market: 'Market', structural: 'Structural', contrarian: 'Contrarian' };
  const covered = new Set(records.flatMap((item) => item.roles));
  const missing = Object.keys(laneNames).filter((role) => !covered.has(role));
  if (decision.stage === 'draft' || decision.stage === 'evidence_ready') {
    const searchDetails = element('details');
    searchDetails.append(element('summary', 'Search for evidence · Brave Search'));
    searchDetails.append(element('p', 'Search results are transient: they are not saved in PDP records and are never forwarded to models. Brave plan charges are not included in the model run budget; check the plan billing before approval. Record a source manually only after opening and reviewing it.'));
    const searchForm = element('form');
    const query = field(searchForm, 'Search query'); query.maxLength = 600;
    const lane = element('select');
    for (const role of ['quant', 'market', 'structural', 'contrarian']) { const option = element('option', role); option.value = role; lane.append(option); }
    const laneLabel = element('label', 'Evidence lane'); laneLabel.append(lane); searchForm.append(laneLabel);
    const runSearch = element('button', 'Preview search request'); searchForm.append(runSearch);
    const resultsHost = element('div'); resultsHost.setAttribute('aria-live', 'polite');
    searchForm.addEventListener('submit', async (event) => {
      event.preventDefault(); runSearch.disabled = true;
      try {
        const body = { request_id: crypto.randomUUID(), query: query.value, role: lane.value, approve: false };
        const preview = await api('/api/search', { method: 'POST', body: JSON.stringify(body) });
        resultsHost.replaceChildren(element('p', `Exact query: “${preview.query}” · ${preview.lane} lane · results remain transient.`));
        const approve = element('button', 'Approve and search once'); const responseHost = element('div');
        approve.addEventListener('click', async () => {
          approve.disabled = true; responseHost.replaceChildren(element('p', 'Searching…'));
          try {
            const result = await api('/api/search', { method: 'POST', body: JSON.stringify({ ...body, approve: true }) });
            responseHost.replaceChildren();
            for (const item of result.results) { const block = element('p'); const link = element('a', item.title); link.href = item.url; link.target = '_blank'; link.rel = 'noreferrer'; block.append(link, element('span', ` — ${item.description}`)); responseHost.append(block); }
            if (!result.results.length) responseHost.append(element('p', 'No usable results returned.'));
            responseHost.append(element('p', 'No result was saved. Open a source and add a curated evidence record yourself if it is relevant.'));
          } catch (error) { responseHost.replaceChildren(element('p', `Search failed and was not retried: ${error.message}. Preview a fresh search request before retrying.`)); approve.disabled = true; runSearch.disabled = false; }
        });
        resultsHost.append(approve, responseHost);
      } catch (error) { resultsHost.replaceChildren(element('p', `Search preview failed: ${error.message}`)); }
      finally { runSearch.disabled = false; }
    });
    searchDetails.append(searchForm, resultsHost); section.append(searchDetails);
    if (decision.stage === 'draft') {
      section.append(element('p', missing.length
        ? `Evidence still needed: ${missing.map((role) => laneNames[role]).join(', ')}. Add a source record for every lane before marking evidence ready.`
        : 'All four evidence lanes have at least one source.'));
    }
    const form = element('form');
    const title = field(form, 'Source title');
    const publisher = field(form, 'Publisher');
    const sourceUrl = field(form, 'Source URL (http or https)');
    const accessed = field(form, 'Accessed at (date/time)');
    const summary = field(form, 'Source-linked summary', true);
    const laneSet = element('fieldset'); laneSet.append(element('legend', 'Evidence lanes'));
    const checks = [];
    for (const role of ['quant', 'market', 'structural', 'contrarian']) {
      const label = element('label'); const input = element('input'); input.type = 'checkbox'; input.value = role;
      label.append(input, document.createTextNode(` ${role}`)); laneSet.append(label); checks.push(input);
    }
    form.append(laneSet);
    const submit = element('button', 'Save evidence record');
    form.append(submit);
    form.addEventListener('submit', async (event) => {
      event.preventDefault();
      try {
        await api(`/api/decisions/${encodeURIComponent(decision.id)}/evidence`, {
          method: 'POST', body: JSON.stringify({
            id: crypto.randomUUID(), decision_id: decision.id, title: title.value,
            source_url: sourceUrl.value, publisher: publisher.value,
            accessed_at: accessed.value, summary: summary.value, source_kind: 'curated',
            roles: checks.filter((check) => check.checked).map((check) => check.value),
          }),
        });
        await refresh();
      } catch (error) { alert(`Evidence not saved: ${error.message}`); }
    });
    section.append(form);
  }
  if (decision.stage === 'draft') {
    const ready = element('button', 'Mark evidence ready');
    ready.disabled = missing.length > 0;
    ready.addEventListener('click', () => action(decision.id, { action: 'evidence_ready' }, ready));
    section.append(ready);
  }
  article.append(section);
}

async function renderLocalAnalysis(article, decision) {
  const events = await api(`/api/decisions/${encodeURIComponent(decision.id)}/events`);
  const analyses = events
    .map(({ event }) => event)
    .filter((event) => event.kind === 'local_assessment_generated');
  for (const record of analyses) {
    const fields = Object.fromEntries(record.fields.map((field) => [field.name, field.value]));
    const details = element('details');
    details.append(element('summary', `Liquid first-pass analysis · ${fields.model_id || 'local model'}`));
    details.append(element('p', 'Exploratory model output only. It is not evidence, a PDP score, or a qualified decision.'));
    details.append(element('pre', fields.output || 'No output was recorded.'));
    article.append(details);
  }
  if (decision.stage !== 'draft') return;

  const details = element('details');
  details.append(element('summary', 'Generate a first-pass analysis with Liquid'));
  details.append(element('p', 'Review the exact prompt, then run the pinned local model. The request stays on this Mac, costs $0, and does not change the decision stage or count as evidence.'));
  const preview = element('div');
  const loadPreview = element('button', 'Show exact prompt');
  const previewStatus = element('p');
  previewStatus.setAttribute('aria-live', 'polite');
  loadPreview.addEventListener('click', async () => {
    loadPreview.disabled = true;
    previewStatus.textContent = 'Loading local prompt preview…';
    try {
      const request = await api(`/api/decisions/${encodeURIComponent(decision.id)}/local-analysis-preview`);
      const pre = element('pre', `Model: ${request.model}\nPrompt version: ${request.prompt_version}\nOutput cap: ${request.max_output_tokens} tokens\nThinking: ${request.thinking_enabled ? 'enabled' : 'disabled'}\nExternal data sent: ${request.external_data_sent}\n\nSYSTEM PROMPT\n${request.system_prompt}\n\nUSER PROMPT\n${request.user_prompt}`);
      preview.append(pre);
      const run = element('button', 'Run Liquid first-pass analysis');
      const result = element('p'); result.setAttribute('aria-live', 'polite');
      run.addEventListener('click', async () => {
        run.disabled = true;
        result.textContent = 'Running the local model…';
        try {
          await api(`/api/decisions/${encodeURIComponent(decision.id)}/local-analysis`, { method: 'POST' });
          await refresh();
        } catch (error) {
          result.textContent = `Analysis did not complete: ${error.message}`;
          run.disabled = false;
        }
      });
      preview.append(run, result);
      previewStatus.textContent = 'Prompt preview ready.';
      loadPreview.hidden = true;
    } catch (error) {
      previewStatus.textContent = `Prompt preview unavailable: ${error.message}`;
      loadPreview.disabled = false;
    }
  });
  details.append(loadPreview, previewStatus, preview);
  article.append(details);
}

function renderPanel(article, decision) {
  if (!Object.hasOwn(decision.role_fit, 'supported')) return;
  const details = element('details'); details.append(element('summary', 'Configure model panel'));
  const form = element('form');
  const panelPins = [
    { vendor: 'open_ai', provider: 'open_ai_responses', model: 'gpt-6-luna' },
    { vendor: 'liquid_ai', provider: 'ollama_chat', model: 'lfm3:1.7b' },
    { vendor: 'alibaba', provider: 'manual', model: 'Qwen 3.5 4B Vision Q4' },
  ].map((defaults, index) => {
    const vendor = field(form, `Model vendor ${index + 1} · canonical ID`);
    vendor.value = defaults.vendor;
    const provider = element('select');
    const adapters = [
      ['open_ai_responses', 'OpenAI Responses'],
      ['anthropic_messages', 'Anthropic Messages'],
      ['ollama_chat', 'Local Ollama / Liquid'],
      ['manual', 'Manual / BonzAI Web / any other vendor'],
    ];
    for (const [value, label] of adapters) {
      const option = element('option', label);
      option.value = value; provider.append(option);
    }
    provider.value = defaults.provider;
    const providerLabel = element('label', `Connection adapter ${index + 1}`);
    providerLabel.append(provider); form.append(providerLabel);
    const model = field(form, `Model name or ID ${index + 1}`);
    model.value = defaults.model;
    return { vendor, provider, model };
  });
  form.append(element('p', 'Enter any lowercase vendor ID. The vendor is the model family; the adapter is how this app reaches it. Choose an automated adapter when supported, or use manual prompt and transcript import for any vendor. Google is paused.'));
  const budget = field(form, 'Run budget in USD'); budget.type = 'number'; budget.min = '0.01'; budget.step = '0.01';
  form.append(element('p', 'Each automated call shows its exact prompt and requires one-call approval. OpenAI, Anthropic, and local Ollama adapters are available. Manual mode supports browser tools such as BonzAI and other vendor interfaces.'));
  const save = element('button', 'Pin models and budget'); form.append(save);
  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    const dollars = Number(budget.value);
    if (!Number.isFinite(dollars) || dollars <= 0 || dollars * 1_000_000 > Number.MAX_SAFE_INTEGER) {
      alert('Enter a valid positive budget.'); return;
    }
    try {
      await api(`/api/decisions/${encodeURIComponent(decision.id)}/panel`, {
        method: 'POST', body: JSON.stringify({ panel: {
          run_id: crypto.randomUUID(), decision_id: decision.id,
          per_run_budget_micros: Math.round(dollars * 1_000_000),
          schema_version: 3,
          models: panelPins.map(({ vendor, provider, model }) => ({ vendor: vendor.value, provider_id: provider.value, model_id: model.value })),
        } }),
      });
      await action(decision.id, { action: 'start_panel' });
    } catch (error) { alert(`Panel was not pinned: ${error.message}`); }
  });
  details.append(form); article.append(details);
}

async function renderProviderCalls(article, decision) {
  let panel;
  try { panel = await api(`/api/decisions/${encodeURIComponent(decision.id)}/panel`); }
  catch (error) { article.append(element('p', `Pinned panel unavailable: ${error.message}`)); return; }
  const records = await api(`/api/decisions/${encodeURIComponent(decision.id)}/transcripts`);
  const completed = new Set(records.map((record) => `${record.vendor}:${record.role}`));
  const section = element('details'); section.open = true;
  section.append(element('summary', `Run provider roles · ${records.length} of 12 complete`));
  section.append(element('p', 'Each role call gets an isolated prompt with only its evidence lane. Inspect the prompt, then approve each external request individually. No automatic provider fallback or web search is used.'));
  for (const model of panel.models) {
    const card = element('div'); card.className = 'model-card';
    card.append(element('h3', `${model.vendor} via ${model.provider_id} · ${model.model_id}`));
    for (const role of ['quant', 'market', 'structural', 'contrarian']) {
      if (completed.has(`${model.vendor}:${role}`)) { card.append(element('p', `${role}: transcript recorded`)); continue; }
      const row = element('div');
      const previewButton = element('button', `Preview ${role} prompt`);
      const output = element('div'); output.setAttribute('aria-live', 'polite');
      previewButton.addEventListener('click', async () => {
        previewButton.disabled = true; output.replaceChildren(element('p', 'Preparing the pinned model request…'));
        try {
          const request = { request_id: crypto.randomUUID(), vendor: model.vendor, role, task: 'elicitation', approve: false };
          const preview = await api(`/api/decisions/${encodeURIComponent(decision.id)}/provider-call`, { method: 'POST', body: JSON.stringify(request) });
          output.replaceChildren(element('p', `Adapter: ${preview.provider_id} · pinned model: ${preview.model_id} · maximum estimate: $${preview.estimated_max_cost_usd.toFixed(3)}`), element('pre', preview.prompt));
          if (preview.provider_id === 'manual') {
            output.append(element('p', 'Copy the prompt into the selected vendor interface. Return to the transcript import form below and save the full structured response.'));
          } else {
            const approve = element('button', 'Approve this call and run it');
            const result = element('p'); result.setAttribute('aria-live', 'polite');
            approve.addEventListener('click', async () => {
              approve.disabled = true; result.textContent = 'Calling the selected adapter…';
              try { const response = await api(`/api/decisions/${encodeURIComponent(decision.id)}/provider-call`, { method: 'POST', body: JSON.stringify({ ...request, approve: true }) }); result.textContent = `Saved ${response.record.vendor} · ${response.record.role}: ${response.record.pick} (${response.record.probability}) · estimated $${response.estimated_cost_usd.toFixed(3)}.`; await refresh(); }
              catch (error) { result.textContent = `Call did not complete and was not retried: ${error.message}. Preview a fresh request before trying again.`; approve.disabled = true; previewButton.disabled = false; }
            });
            output.append(approve, result);
          }
        } catch (error) { output.replaceChildren(element('p', `Preview failed: ${error.message}`)); previewButton.disabled = false; }
      });
      row.append(previewButton, output); card.append(row);
    }
    section.append(card);
  }
  article.append(section);
}

async function renderG1(article, decision) {
  const details = element('details');
  details.open = true;
  details.append(element('summary', 'G1 · consensus and evidence review'));
  let score;
  try {
    score = await api(`/api/decisions/${encodeURIComponent(decision.id)}/g1`);
  } catch (error) {
    details.append(element('p', `G1 calculation unavailable: ${error.message}`));
    article.append(details);
    return;
  }
  details.append(element('p', `Adjusted Convergence Score (ACS): ${score.acs.toFixed(3)} · modal option: ${score.modal_pick} · ${score.total_agents} agents`));
  details.append(element('p', `Protocol reading: ${score.sizing_band.replaceAll('_', ' ')}. ACS is a declared governance heuristic, not a calibrated probability or validated independence measure.`));
  const list = element('ul');
  for (const record of score.records) {
    list.append(element('li', `${record.vendor} · ${record.model_id} · ${record.role}: ${record.pick} (${record.probability})`));
  }
  details.append(list);

  if (score.existing_review) {
    const review = score.existing_review;
    details.append(element('p', `Human review recorded by ${review.reviewed_by}: ${review.disposition}. ${review.rationale}`));
    if (review.disposition === 'approve') {
      const approve = element('button', 'Approve G1 and continue to stress testing');
      approve.addEventListener('click', () => action(decision.id, { action: 'approve_g1' }, approve));
      details.append(approve);
    } else {
      const reject = element('button', 'Reject G1 and stop this run');
      reject.addEventListener('click', () => action(decision.id, { action: 'reject_g1', reason: review.rationale }, reject));
      details.append(reject);
    }
    article.append(details);
    return;
  }

  details.append(element('p', 'Review the model-role picks. Record factual conflicts, weighting conflicts, evidence gaps, and your rationale. For ACS below 0.40, describe the divergence resolution before approval.'));
  const form = element('form');
  const reviewer = field(form, 'Human reviewer');
  reviewer.value = document.querySelector('#actor').value || 'local-human';
  const facts = field(form, 'Factual conflicts (or “none identified”)', true);
  const weights = field(form, 'Weighting conflicts (or “none identified”)', true);
  const gaps = field(form, 'Evidence gaps', true);
  const rationale = field(form, 'G1 review rationale', true);
  const disposition = element('select');
  for (const [value, label] of [['approve', 'Approve G1'], ['reject', 'Reject G1']]) {
    const option = element('option', label); option.value = value; disposition.append(option);
  }
  const dispositionLabel = element('label', 'Human gate decision');
  dispositionLabel.append(disposition); form.append(dispositionLabel);
  const save = element('button', 'Save G1 review'); form.append(save);
  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    save.disabled = true;
    try {
      await api(`/api/decisions/${encodeURIComponent(decision.id)}/g1`, {
        method: 'POST', body: JSON.stringify({
          reviewed_by: reviewer.value,
          factual_conflicts: facts.value,
          weighting_conflicts: weights.value,
          evidence_gaps: gaps.value,
          rationale: rationale.value,
          disposition: disposition.value,
        }),
      });
      await refresh();
    } catch (error) {
      details.append(element('p', `G1 review not saved: ${error.message}`));
      save.disabled = false;
    }
  });
  details.append(form); article.append(details);
}

async function renderTranscripts(article, decision) {
  const records = await api(`/api/decisions/${encodeURIComponent(decision.id)}/transcripts`);
  article.append(element('p', `Validated manual transcripts: ${records.length} / 12`));
  for (const record of records) {
    const details = element('details');
    details.append(element('summary', `${record.vendor} · ${record.role} · ${record.pick} (${record.probability})`));
    details.append(element('p', `Alternative: ${record.alternative} (${record.alternative_probability})`));
    details.append(element('p', `Load-bearing facts: ${record.load_bearing_facts.join(' · ')}`));
    details.append(element('p', `Searched versus known: ${record.searched_versus_known}`));
    details.append(element('pre', record.raw_response));
    article.append(details);
  }
  if (decision.stage !== 'eliciting') return;
  const details = element('details'); details.append(element('summary', 'Import one role transcript as JSON'));
  details.append(element('p', 'Review the pasted record before saving. The raw transcript is stored in the encrypted local database.'));
  const form = element('form'); const json = field(form, 'Transcript record JSON', true);
  const save = element('button', 'Validate and save transcript'); form.append(save);
  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    try {
      const record = JSON.parse(json.value);
      record.decision_id = decision.id; record.manually_imported = true;
      await api(`/api/decisions/${encodeURIComponent(decision.id)}/transcripts`, {
        method: 'POST', body: JSON.stringify(record),
      });
      await refresh();
    } catch (error) { alert(`Transcript not saved: ${error.message}`); }
  });
  details.append(form); article.append(details);
  if (records.length === 12) {
    const complete = element('button', 'Complete elicitation and open G1');
    complete.addEventListener('click', () => action(decision.id, { action: 'panel_completed' }));
    article.append(complete);
  }
}


async function workflowRecordEditor(article, decision, type, heading, template, nextAction) {
  const details = element('details'); details.open = true; details.append(element('summary', heading));
  details.append(element('p', 'This record is validated by the local service and appended to the decision history. Review every field before saving.'));
  const form = element('form'); const input = field(form, 'Versioned structured record JSON', true); input.value = JSON.stringify(template, null, 2);
  const save = element('button', 'Validate and save record'); const status = element('p');
  save.addEventListener('click', async (event) => {
    event.preventDefault(); save.disabled = true;
    try {
      const record = JSON.parse(input.value);
      await saveWorkflowRecord(decision, type, record);
      if (nextAction) {
        let operation = nextAction;
        if (type === 'g2_review' && record.disposition === 'reject') operation = { action: 'reject_g2', reason: record.rationale };
        if (type === 'monitoring' && record.thesis_impact === 'invalidates') operation = { action: 'trigger_invalidation' };
        await action(decision.id, operation, save);
      } else await refresh();
    }
    catch (error) { status.textContent = `Record rejected: ${error.message}`; save.disabled = false; }
  });
  form.append(save, status); details.append(form); article.append(details);
}

async function reviewAndSaveWorkflowResult(decision, type, record, host) {
  const classification = type === 'pressure_test' ? element('select') : null;
  if (classification) {
    const empty=element('option','Select the human classification');empty.value='';classification.append(empty);
    for (const value of ['robust','evidence_driven_flip','pressure_driven_flip','collapse']) { const option=element('option',value);option.value=value;classification.append(option); }
    const label=element('label',`Human classification (model suggested ${record.classification}; review the response)`);label.append(classification);host.append(label);
  }
  const save=element('button','Review and save validated result');
  save.addEventListener('click',async()=>{save.disabled=true;try{if(classification){if(!classification.value)throw new Error('choose the human classification');record.classification=classification.value;}await saveWorkflowRecord(decision,type,record);await refresh();}catch(error){host.append(element('p',`Record not saved: ${error.message}`));save.disabled=false;}});
  host.append(save);
}

async function renderLaterStage(article, decision) {
  const events = await api(`/api/decisions/${encodeURIComponent(decision.id)}/events`);
  const panel = await api(`/api/decisions/${encodeURIComponent(decision.id)}/panel`).catch(() => null);
  const critiques = workflowItems(events, 'self_critique');
  const formal = workflowItems(events, 'formal_model');
  const pressures = workflowItems(events, 'pressure_test');
  if (decision.stage === 'stress_testing') {
    const section = element('details'); section.open = true;
    section.append(element('summary', `Phase 2 stress testing · ${critiques.length}/12 self-critiques · ${formal.length}/1 formal model · ${pressures.length}/12 pressure tests`));
    section.append(element('p', 'The sequence is load-bearing: each agent first critiques its own pick; the human freezes justified weights before scoring; only then each same model receives the exact pressure prompt “I think you’re wrong.”'));
    const picks = element('div'); const vendor = element('select');
    for (const pin of (panel?.models || [])) { const opt = element('option', `${pin.vendor} · ${pin.model_id}`); opt.value = pin.vendor; vendor.append(opt); }
    const role = element('select'); for (const lane of ['quant','market','structural','contrarian']) { const opt=element('option',lane);opt.value=lane;role.append(opt); }
    picks.append(element('label','Pinned vendor'),vendor,element('label','Role'),role); section.append(picks);
    const output = element('div'); output.setAttribute('aria-live','polite');
    const runTask = (task,label,recordType,makeRecord) => {
      const button=element('button',label); button.addEventListener('click',async()=>{button.disabled=true;try{
        const base={request_id:crypto.randomUUID(),vendor:vendor.value,role:role.value,task,approve:false};const preview=await api(`/api/decisions/${encodeURIComponent(decision.id)}/provider-call`,{method:'POST',body:JSON.stringify(base)});
        output.replaceChildren(element('p',`${preview.provider_id} · ${preview.model_id} · estimated maximum $${preview.estimated_max_cost_usd.toFixed(3)} · within budget: ${preview.within_budget}`),element('pre',preview.prompt));
        if(preview.provider_id==='manual'){
          output.append(element('p','Use the copy-ready prompt in your chosen vendor. Paste its structured JSON response here; it will be validated before recording.'));
          const manual=element('textarea');manual.required=true;manual.placeholder='Paste the required JSON response';const save=element('button','Validate and save manual result');
          save.addEventListener('click',async(e)=>{e.preventDefault();try{const data=JSON.parse(manual.value);await reviewAndSaveWorkflowResult(decision,recordType,makeRecord(data,vendor.value,role.value),output);}catch(error){output.append(element('p',`Manual result not saved: ${error.message}`));}});
          output.append(manual,save);return;
        }
        const approve=element('button','Approve this provider call');const status=element('p');approve.addEventListener('click',async()=>{approve.disabled=true;status.textContent='Waiting for the selected provider…';try{const result=await api(`/api/decisions/${encodeURIComponent(decision.id)}/provider-call`,{method:'POST',body:JSON.stringify({...base,approve:true})});output.append(element('h4','Provider output · inspect before recording'),element('pre',result.output));const record=makeRecord(JSON.parse(result.output),vendor.value,role.value);await reviewAndSaveWorkflowResult(decision,recordType,record,output);}catch(error){status.textContent=`Provider call did not complete and was not retried: ${error.message}. Preview a fresh request before trying again.`;approve.disabled=true;button.disabled=false;}});output.append(approve,status);
      }catch(error){output.replaceChildren(element('p',`Cannot run this step: ${error.message}`));button.disabled=false;}});section.append(button);
    };
    if(critiques.length<12)runTask('stress_self_critique','Generate self-critique','self_critique',(data,v,r)=>{const pin=panel.models.find(x=>x.vendor===v);return{schema_version:1,decision_id:decision.id,run_id:panel.run_id,vendor:v,model_id:pin.model_id,role:r,factors:(data.factors||[]).map(x=>({...x,severity:String(x.severity).toLowerCase()})),flip_scenario:data.flip_scenario||''};});
    if(critiques.length===12&&formal.length===0){const modelForm=element('form');const owner=field(modelForm,'Human who set and froze weights');owner.value=document.querySelector('#actor').value||'local-human';const dims=field(modelForm,'Formal model dimensions JSON',true);dims.value=JSON.stringify([{name:'Replace with dimension',weight:1,scores:Object.fromEntries((await api('/api/decisions')).find(d=>d.id===decision.id).options.map(x=>[x,0])),justification:'Enter the human rationale before scoring.'}],null,2);const freeze=element('button','Freeze human-owned formal model');freeze.addEventListener('click',async(e)=>{e.preventDefault();freeze.disabled=true;try{await saveWorkflowRecord(decision,'formal_model',{schema_version:1,decision_id:decision.id,run_id:panel.run_id,prepared_by:owner.value,frozen_at:new Date().toISOString(),dimensions:JSON.parse(dims.value)});await refresh();}catch(error){modelForm.append(element('p',`Formal model not saved: ${error.message}`));freeze.disabled=false;}});modelForm.append(freeze);section.append(element('p','Weights must sum to 1, have justifications, and score every option from 0–10. These are human inputs; model output cannot set or revise them.'),modelForm);}
    if(formal.length===1&&pressures.length<12)runTask('stress_pressure_test','Run exact pressure test','pressure_test',(data,v,r)=>{const pin=panel.models.find(x=>x.vendor===v);return{schema_version:1,decision_id:decision.id,run_id:panel.run_id,vendor:v,model_id:pin.model_id,role:r,prompt:"I think you're wrong.",response:data.response||'',classification:String(data.classification||'robust').toLowerCase(),new_evidence:data.new_evidence||'none; no evidence supplied'};});
    section.append(output);
    if(critiques.length===12&&formal.length===1&&pressures.length===12){const complete=element('button','Complete stress testing and open G2');complete.addEventListener('click',()=>action(decision.id,{action:'stress_completed'},complete));section.append(complete);}
    article.append(section);return;
  }
  if (decision.stage === 'g2_pending') {
    const summary=await api(`/api/decisions/${encodeURIComponent(decision.id)}/g2-summary`);article.append(element('p',`ACS ${summary.acs.toFixed(3)} · Flip Distance ${summary.flip_distance.toFixed(3)} · ${summary.sizing_band} sizing cell · modal pick ${summary.modal_pick}`));
    const review={schema_version:1,decision_id:decision.id,run_id:panel.run_id,reviewed_by:document.querySelector('#actor').value||'local-human',observable:'',precommitted_exit:'',distinct_from_variance:'',pressure_asymmetry:'',disposition:'approve',rationale:'',reviewed_at:new Date().toISOString()};
    await workflowRecordEditor(article,decision,'g2_review','G2 human review: all four invalidation tests',review,{action:'approve_g2'});return;
  }
  if (decision.stage === 'g3_pending') {
    const summary=await api(`/api/decisions/${encodeURIComponent(decision.id)}/g2-summary`);article.append(element('p',`G3 inputs: ACS ${summary.acs.toFixed(3)}, Flip Distance ${summary.flip_distance.toFixed(3)}, matrix cell ${summary.sizing_band}, multiplier ${summary.matrix_multiplier}.`));
    const prior=workflowItems(events,'commitment');
    if(prior.length){article.append(element('pre',JSON.stringify(prior[prior.length-1],null,2)));const approve=element('button','Human approve G3 and lock this decision');approve.addEventListener('click',()=>action(decision.id,{action:'approve_g3'},approve));article.append(approve);return;}
    const record={schema_version:1,decision_id:decision.id,run_id:panel.run_id,proposed_by:document.querySelector('#actor').value||'local-human',p:0,value_if_right:1,cost_if_wrong:1,quarter_kelly:.25,matrix_multiplier:summary.matrix_multiplier,bankroll:0,ruin_tolerance:0,committed_amount:0,terminal_fraction:.45,checkpoint_fraction:.45,single_event_fraction:.1,rationale:'',proposed_at:new Date().toISOString()};
    await workflowRecordEditor(article,decision,'commitment','G3 structured commitment proposal',record,{action:'commitment_proposed'});
    const approve=element('button','Human approve G3 and lock this decision');approve.addEventListener('click',()=>action(decision.id,{action:'approve_g3'},approve));article.append(approve);return;
  }
  if (decision.stage === 'locked') {
    const run=panel?.run_id||'';const now=new Date().toISOString();
    const prior=workflowItems(events,'entry_execution');
    if(prior.length)article.append(element('pre',JSON.stringify(prior[prior.length-1],null,2)));
    else {const execution={schema_version:1,decision_id:decision.id,run_id:run,executed_by:document.querySelector('#actor').value||'local-human',entry_window_started:now,entry_window_ended:now,legs:[{instrument:'',direction:'',size:0,price:0,platform_confirmation:''}],reconciliation:'',reconciled_by:document.querySelector('#actor').value||'local-human',reconciled_at:now};await workflowRecordEditor(article,decision,'entry_execution','Record actual entry window and post-entry reconciliation',execution,null);}
    const start=element('button','Finish reconciliation and begin monitoring');start.addEventListener('click',()=>action(decision.id,{action:'start_monitoring'},start));article.append(start);return;
  }
  if (decision.stage === 'monitoring') {
    const run=panel?.run_id||'';const template={schema_version:1,decision_id:decision.id,run_id:run,recorded_by:document.querySelector('#actor').value||'local-human',expected:'',actual:'',thesis_impact:'none',external_source:'https://',matched_invalidation_condition:false,recorded_at:new Date().toISOString()};
    await workflowRecordEditor(article,decision,'monitoring','Record a world-state checkpoint (not model commentary)',template,{action:'record_checkpoint'});
    const settle={schema_version:1,decision_id:decision.id,settled_by:document.querySelector('#actor').value||'local-human',realized_outcome:'',thesis_probability:0,realized_return:0,luck_decomposition:'',checkpoint_hit_rate:0,invalidation_triggered:false,settled_at:new Date().toISOString()};
    await workflowRecordEditor(article,decision,'settlement','Record settlement; outcome is hidden from the audit view',settle,{action:'record_settlement'});return;
  }
  if (decision.stage === 'settled') {
    const criteria=Object.fromEntries(['panel_decorrelation','elicitation_quality','self_critique_before_pressure','weights_frozen','sensitivity_analysis','invalidation_quality','entry_discipline','lock_integrity'].map(x=>[x,0]));
    const audit={schema_version:1,decision_id:decision.id,auditor:document.querySelector('#actor').value||'local-human',outcome_hidden_during_scoring:true,criteria,evidence:'',calibration_note:'',audited_at:new Date().toISOString()};
    await workflowRecordEditor(article,decision,'audit','Outcome-blind process audit · score eight criteria 0–2',audit,{action:'record_audit'});return;
  }
  if (decision.stage === 'audited') {const settled=workflowItems(events,'settlement');const audits=workflowItems(events,'audit');if(settled.length)article.append(element('p',`Settlement: ${settled[0].realized_outcome} · return ${settled[0].realized_return} · checkpoint hit rate ${settled[0].checkpoint_hit_rate}`));if(audits.length){const score=Object.values(audits[0].criteria).reduce((sum,value)=>sum+value,0);article.append(element('p',`Process score: ${score}/16 · ${audits[0].auditor}`),element('p',`Calibration note: ${audits[0].calibration_note}`));}article.append(element('p','Audit recorded. Review and calibration are process evidence; this run alone does not establish protocol effectiveness.'));}
}

async function refresh() {
  const list = await api('/api/decisions');
  const root = document.querySelector('#records'); root.replaceChildren();
  for (const decision of list) {
    const article = element('article'); article.append(element('h3', decision.title));
    article.append(element('p', decision.question));
    article.append(element('p', `Stage: ${decision.stage} · ${decision.domain} · ${decision.options.join(' / ')}`));
    await renderLocalAnalysis(article, decision);
    await renderEvidence(article, decision);
    if (decision.stage === 'evidence_ready') renderPanel(article, decision);
    if (decision.stage === 'eliciting') { await renderProviderCalls(article, decision); await renderTranscripts(article, decision); }
    if (decision.stage === 'g1_pending') {
      await renderG1(article, decision);
    }
    if (['stress_testing','g2_pending','g3_pending','locked','monitoring','settled','audited'].includes(decision.stage)) await renderLaterStage(article, decision);
    const history = element('button', 'Show event history');
    history.addEventListener('click', async () => {
      try { const events = await api(`/api/decisions/${encodeURIComponent(decision.id)}/events`); article.append(element('pre', JSON.stringify(events, null, 2))); }
      catch (error) { alert(`History unavailable: ${error.message}`); }
    });
    article.append(history); root.append(article);
  }
  if (!list.length) root.textContent = 'No decisions yet.';
}

document.querySelector('#create').addEventListener('submit', async (event) => {
  event.preventDefault();
  const fit = document.querySelector('[name="fit"]:checked').value;
  const mappingIds = { quant: 'quant', market: 'market', structural: 'structural', contrarian: 'contrarian' };
  const role_fit = fit === 'supported'
    ? { supported: { mappings: Object.fromEntries(Object.entries(mappingIds).map(([role, id]) => [role, document.getElementById(id).value])) } }
    : { unsupported: { reason: 'Not mapped to every v1.3.1 role' } };
  const record = {
    schema_version: 1, protocol_version: '1.3.1', id: crypto.randomUUID(),
    title: document.querySelector('#title').value,
    question: document.querySelector('#question').value,
    domain: document.querySelector('#domain').value,
    options: document.querySelector('#options').value.split('\n').map((value) => value.trim()).filter(Boolean),
    role_fit, stage: 'draft', human_actor: document.querySelector('#actor').value,
  };
  try { await api('/api/decisions', { method: 'POST', body: JSON.stringify(record) }); event.target.reset(); document.querySelector('#actor').value = 'local-human'; roles.hidden = true; roles.querySelectorAll('input').forEach((input) => { input.required = false; }); await refresh(); }
  catch (error) { alert(`Decision not saved: ${error.message}`); }
});

refresh().catch((error) => { document.querySelector('#records').textContent = `Local database unavailable: ${error.message}`; });
