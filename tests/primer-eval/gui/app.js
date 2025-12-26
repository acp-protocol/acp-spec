/**
 * ACP Primer Evaluation GUI
 */

// State
let currentView = 'runs';
let primers = [];
let scenarios = [];
let runs = [];
let selectedPrimer = null;

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
  setupNavigation();
  await loadData();
  showView('runs');
});

// Navigation
function setupNavigation() {
  document.querySelectorAll('.nav-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      showView(btn.dataset.view);
    });
  });
}

function showView(view) {
  currentView = view;

  // Update nav
  document.querySelectorAll('.nav-btn').forEach(btn => {
    btn.classList.toggle('active', btn.dataset.view === view);
  });

  // Update views
  document.querySelectorAll('.view').forEach(v => {
    v.classList.remove('active');
  });

  const viewEl = document.getElementById(`${view}-view`);
  if (viewEl) {
    viewEl.classList.add('active');
  }

  // Load data for view
  if (view === 'runs') loadRuns();
  if (view === 'primers') loadPrimers();
  if (view === 'scenarios') loadScenarios();
  if (view === 'new-test') loadTestForm();
}

// Data Loading
async function loadData() {
  try {
    const [primersRes, scenariosRes, runsRes] = await Promise.all([
      fetch('/api/primers'),
      fetch('/api/scenarios'),
      fetch('/api/runs')
    ]);

    primers = await primersRes.json();
    scenarios = await scenariosRes.json();
    runs = await runsRes.json();
  } catch (error) {
    console.error('Failed to load data:', error);
  }
}

// Runs View
async function loadRuns() {
  const container = document.getElementById('runs-list');

  if (runs.length === 0) {
    container.innerHTML = '<p class="loading">No test runs yet. Run a test to get started.</p>';
    return;
  }

  container.innerHTML = runs.map(run => `
    <div class="card" onclick="showRunDetails('${run.id}')">
      <div class="card-header">
        <div class="card-title">${run.primer}</div>
        <span class="badge ${run.passRate >= 80 ? 'badge-pass' : run.passRate >= 50 ? 'badge-pending' : 'badge-fail'}">
          ${run.passRate.toFixed(1)}% pass
        </span>
      </div>
      <div class="card-meta">${new Date(run.timestamp).toLocaleString()}</div>
    </div>
  `).join('');
}

async function showRunDetails(runId) {
  const container = document.getElementById('run-details-content');
  container.innerHTML = '<p class="loading">Loading run details...</p>';

  document.getElementById('runs-view').classList.remove('active');
  document.getElementById('run-details-view').classList.add('active');

  try {
    const res = await fetch(`/api/runs/${runId}`);
    const run = await res.json();

    container.innerHTML = `
      <div class="run-summary">
        <h2>${run.primer.name} - ${new Date(run.run.timestamp).toLocaleString()}</h2>
        <div class="summary-stats">
          <div class="stat">
            <div class="stat-value">${run.summary.total}</div>
            <div class="stat-label">Scenarios</div>
          </div>
          <div class="stat">
            <div class="stat-value">${run.summary.passed.pattern}/${run.summary.total}</div>
            <div class="stat-label">Pattern Pass</div>
          </div>
          ${run.summary.passed.judge !== undefined ? `
          <div class="stat">
            <div class="stat-value">${run.summary.passed.judge}/${run.summary.total}</div>
            <div class="stat-label">Judge Pass</div>
          </div>
          ` : ''}
          <div class="stat">
            <div class="stat-value">${run.summary.pass_rate.pattern.toFixed(1)}%</div>
            <div class="stat-label">Pass Rate</div>
          </div>
          <div class="stat">
            <div class="stat-value">${run.summary.tokens.total.toLocaleString()}</div>
            <div class="stat-label">Tokens</div>
          </div>
          <div class="stat">
            <div class="stat-value">$${run.summary.cost_estimate_usd.toFixed(4)}</div>
            <div class="stat-label">Cost</div>
          </div>
        </div>
      </div>

      <h3>Exchanges</h3>
      <div class="exchange-list">
        ${run.exchanges.map((ex, i) => renderExchange(ex, i)).join('')}
      </div>
    `;

    // Setup expand/collapse
    document.querySelectorAll('.exchange-header').forEach(header => {
      header.addEventListener('click', () => {
        const content = header.nextElementSibling;
        content.classList.toggle('expanded');
      });
    });

    // Highlight code
    document.querySelectorAll('pre code').forEach(block => {
      hljs.highlightElement(block);
    });
  } catch (error) {
    container.innerHTML = `<p class="loading">Error loading run: ${error.message}</p>`;
  }
}

function renderExchange(exchange, index) {
  const passed = exchange.evaluation.pattern.passed &&
    (exchange.evaluation.judge?.overall_pass !== false);

  return `
    <div class="exchange-card">
      <div class="exchange-header">
        <div>
          <span class="badge ${passed ? 'badge-pass' : 'badge-fail'}">${passed ? 'PASS' : 'FAIL'}</span>
          <strong>${exchange.scenario.name}</strong>
          <span class="badge badge-category">${exchange.scenario.category}</span>
          <span class="badge badge-difficulty">${exchange.scenario.difficulty}</span>
        </div>
        <div class="card-meta">
          Score: ${(exchange.evaluation.pattern.score * 100).toFixed(0)}% |
          ${exchange.response.tokens.input + exchange.response.tokens.output} tokens |
          ${exchange.response.latency_ms}ms
        </div>
      </div>
      <div class="exchange-content">
        <div class="exchange-section">
          <h4>User Message</h4>
          <pre><code>${escapeHtml(exchange.request.user_message)}</code></pre>
        </div>

        <div class="exchange-section">
          <h4>AI Response</h4>
          <pre><code>${escapeHtml(exchange.response.content)}</code></pre>
        </div>

        <div class="exchange-section">
          <h4>Pattern Evaluation</h4>
          <ul class="criteria-list">
            ${exchange.evaluation.pattern.criteria.map(c => `
              <li class="criteria-item">
                <span class="criteria-icon">${c.passed ? '✅' : '❌'}</span>
                <span class="criteria-desc">${c.criterion.description}</span>
                ${c.match ? `<span class="criteria-match">Match: "${c.match}"</span>` : ''}
                ${c.violation ? `<span class="criteria-match">Violation: "${c.violation}"</span>` : ''}
              </li>
            `).join('')}
          </ul>
        </div>

        ${exchange.evaluation.judge ? `
        <div class="exchange-section">
          <h4>Judge Evaluation</h4>
          <div class="judge-scores">
            ${Object.entries(exchange.evaluation.judge.scores).map(([key, value]) => `
              <div class="judge-score">
                <div class="judge-score-value">${value}/5</div>
                <div class="judge-score-label">${key.replace(/_/g, ' ')}</div>
              </div>
            `).join('')}
          </div>
          <p style="margin-top: 1rem;"><strong>Explanation:</strong> ${exchange.evaluation.judge.explanation}</p>
          ${exchange.evaluation.judge.suggestions.length > 0 ? `
            <p><strong>Suggestions:</strong></p>
            <ul>
              ${exchange.evaluation.judge.suggestions.map(s => `<li>${s}</li>`).join('')}
            </ul>
          ` : ''}
        </div>
        ` : ''}
      </div>
    </div>
  `;
}

// Primers View
async function loadPrimers() {
  const list = document.getElementById('primers-list');

  // Sort: built-in first, then custom
  const sortedPrimers = [...primers].sort((a, b) => {
    if (a.isCustom && !b.isCustom) return 1;
    if (!a.isCustom && b.isCustom) return -1;
    return a.name.localeCompare(b.name);
  });

  list.innerHTML = sortedPrimers.map(p => `
    <div class="sidebar-item ${selectedPrimer === p.name ? 'active' : ''}"
         onclick="selectPrimer('${p.name}')">
      <div class="sidebar-item-name">
        ${p.isCustom ? '<span class="custom-badge">custom</span> ' : ''}${p.name.replace('custom/', '')}
      </div>
      <div class="sidebar-item-meta">~${p.tokens} tokens</div>
    </div>
  `).join('');

  // Setup editor
  document.getElementById('primer-content').addEventListener('input', updatePreview);
  document.getElementById('save-primer-btn').addEventListener('click', savePrimer);
  document.getElementById('new-primer-btn').addEventListener('click', newPrimer);
}

async function selectPrimer(name) {
  selectedPrimer = name;

  // Update sidebar
  document.querySelectorAll('.sidebar-item').forEach(item => {
    item.classList.toggle('active', item.querySelector('.sidebar-item-name').textContent === name);
  });

  try {
    const res = await fetch(`/api/primers/${name}`);
    const primer = await res.json();

    document.getElementById('primer-name').textContent = primer.name;
    document.getElementById('primer-meta-name').value = primer.name;
    document.getElementById('primer-meta-tokens').value = primer.tokens;
    document.getElementById('primer-meta-desc').value = primer.description;
    document.getElementById('primer-meta-tags').value = primer.tags.join(', ');
    document.getElementById('primer-content').value = primer.text;
    document.getElementById('save-primer-btn').disabled = false;

    updatePreview();
  } catch (error) {
    console.error('Failed to load primer:', error);
  }
}

function updatePreview() {
  const content = document.getElementById('primer-content').value;
  document.getElementById('primer-preview').textContent = content;

  // Update token estimate
  const tokens = Math.ceil(content.length / 4);
  document.getElementById('primer-meta-tokens').value = tokens;
}

function newPrimer() {
  selectedPrimer = null;

  document.getElementById('primer-name').textContent = 'New Primer';
  document.getElementById('primer-meta-name').value = '';
  document.getElementById('primer-meta-tokens').value = '';
  document.getElementById('primer-meta-desc').value = '';
  document.getElementById('primer-meta-tags').value = '';
  document.getElementById('primer-content').value = '';
  document.getElementById('primer-preview').textContent = '';
  document.getElementById('save-primer-btn').disabled = false;

  // Clear sidebar selection
  document.querySelectorAll('.sidebar-item').forEach(item => {
    item.classList.remove('active');
  });
}

async function savePrimer() {
  const name = document.getElementById('primer-meta-name').value.trim();
  if (!name) {
    alert('Please enter a primer name');
    return;
  }

  const content = document.getElementById('primer-content').value;
  const frontmatter = {
    name: name,
    version: '1.0',
    tokens: parseInt(document.getElementById('primer-meta-tokens').value) || Math.ceil(content.length / 4),
    description: document.getElementById('primer-meta-desc').value,
    tags: document.getElementById('primer-meta-tags').value.split(',').map(t => t.trim()).filter(t => t)
  };

  // Use selected primer path if editing, otherwise create new with sanitized name
  const savePath = selectedPrimer && selectedPrimer.startsWith('custom/')
    ? selectedPrimer
    : name.toLowerCase().replace(/[^a-z0-9]+/g, '-');

  try {
    const res = await fetch(`/api/primers/${savePath}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content, frontmatter })
    });

    if (res.ok) {
      const result = await res.json();
      alert('Primer saved successfully');
      // Reload primers
      const primersRes = await fetch('/api/primers');
      primers = await primersRes.json();
      selectedPrimer = result.path; // Select the saved primer
      loadPrimers();
    } else {
      const error = await res.json();
      alert(`Failed to save: ${error.error}`);
    }
  } catch (error) {
    alert(`Failed to save: ${error.message}`);
  }
}

// Scenarios View
function loadScenarios() {
  const container = document.getElementById('scenarios-list');

  if (scenarios.length === 0) {
    container.innerHTML = '<p class="loading">No scenarios found.</p>';
    return;
  }

  container.innerHTML = scenarios.map(s => `
    <div class="card">
      <div class="card-header">
        <div class="card-title">${s.name}</div>
        <div>
          <span class="badge badge-category">${s.category}</span>
          <span class="badge badge-difficulty">${s.difficulty}</span>
        </div>
      </div>
      <div class="card-meta">${s.description}</div>
      <details style="margin-top: 1rem;">
        <summary style="cursor: pointer; color: #666;">Show user message</summary>
        <pre style="margin-top: 0.5rem; padding: 0.75rem; background: #f8f9fa; border-radius: 4px; font-size: 0.875rem; white-space: pre-wrap;">${escapeHtml(s.userMessage)}</pre>
      </details>
    </div>
  `).join('');
}

// New Test Form
function loadTestForm() {
  const select = document.getElementById('test-primer');
  const builtIn = primers.filter(p => !p.isCustom);
  const custom = primers.filter(p => p.isCustom);

  let options = '<option value="">Select a primer...</option>';

  if (builtIn.length > 0) {
    options += '<optgroup label="Built-in">';
    options += builtIn.map(p => `<option value="${p.name}">${p.name} (~${p.tokens} tokens)</option>`).join('');
    options += '</optgroup>';
  }

  if (custom.length > 0) {
    options += '<optgroup label="Custom">';
    options += custom.map(p => `<option value="${p.name}">${p.name.replace('custom/', '')} (~${p.tokens} tokens)</option>`).join('');
    options += '</optgroup>';
  }

  select.innerHTML = options;

  // Setup form submission
  document.getElementById('new-test-form').addEventListener('submit', runTest);
}

async function runTest(event) {
  event.preventDefault();

  const primer = document.getElementById('test-primer').value;
  if (!primer) {
    alert('Please select a primer');
    return;
  }

  const categories = Array.from(document.querySelectorAll('#test-categories input:checked'))
    .map(cb => cb.value);

  const useJudge = document.getElementById('test-judge').checked;
  const dryRun = document.getElementById('test-dry-run').checked;

  // Show progress panel
  document.getElementById('new-test-form').classList.add('hidden');
  const progressPanel = document.getElementById('test-progress');
  progressPanel.classList.remove('hidden');
  progressPanel.innerHTML = `
    <h3>Starting test run...</h3>
    <div class="progress-log" id="progress-log"></div>
  `;

  const logEl = document.getElementById('progress-log');
  const addLog = (message, type = 'info') => {
    const entry = document.createElement('div');
    entry.className = `log-entry log-${type}`;
    entry.innerHTML = message;
    logEl.appendChild(entry);
    logEl.scrollTop = logEl.scrollHeight;
  };

  try {
    // Build URL with query params
    const params = new URLSearchParams({
      primer,
      categories: categories.join(','),
      judge: useJudge.toString(),
      dryRun: dryRun.toString()
    });

    const eventSource = new EventSource(`/api/run-stream?${params}`);
    let runId = null;

    eventSource.addEventListener('start', (e) => {
      const data = JSON.parse(e.data);
      progressPanel.querySelector('h3').textContent = `Testing: ${data.primer} (~${data.tokens} tokens)`;
      addLog(`Starting ${data.scenarioCount} scenarios${data.useJudge ? ' with Claude-as-judge' : ''}...`, 'info');
    });

    eventSource.addEventListener('scenario-start', (e) => {
      const data = JSON.parse(e.data);
      addLog(`<span class="log-progress">[${data.index}/${data.total}]</span> Running: <strong>${data.name}</strong> <span class="badge badge-category">${data.category}</span>`, 'pending');
    });

    eventSource.addEventListener('scenario-complete', (e) => {
      const data = JSON.parse(e.data);
      const icon = data.passed ? '✅' : '❌';
      const scorePercent = (data.score * 100).toFixed(0);
      addLog(
        `<span class="log-progress">[${data.index}/${data.total}]</span> ${icon} <strong>${data.name}</strong> - ${scorePercent}% (${data.tokens.input + data.tokens.output} tokens, ${data.latency}ms)`,
        data.passed ? 'success' : 'fail'
      );

      // Show response preview
      if (data.response) {
        addLog(`<details class="response-preview"><summary>Response preview</summary><pre>${escapeHtml(data.response)}</pre></details>`, 'response');
      }
    });

    eventSource.addEventListener('scenario-error', (e) => {
      const data = JSON.parse(e.data);
      addLog(`<span class="log-progress">[${data.index}]</span> ❌ <strong>${data.name}</strong> - ERROR: ${data.error}`, 'error');
    });

    eventSource.addEventListener('complete', (e) => {
      const data = JSON.parse(e.data);
      runId = data.runId;

      addLog('<hr>', 'divider');
      addLog(`<strong>Complete!</strong> Pass rate: ${data.summary.pass_rate.pattern.toFixed(1)}% | Tokens: ${data.summary.tokens.total} | Cost: $${data.summary.cost_estimate_usd.toFixed(4)}`, 'complete');

      // Add button to view results
      const viewBtn = document.createElement('button');
      viewBtn.className = 'btn btn-primary';
      viewBtn.style.marginTop = '1rem';
      viewBtn.textContent = 'View Full Results';
      viewBtn.onclick = async () => {
        const runsRes = await fetch('/api/runs');
        runs = await runsRes.json();
        showRunDetails(runId);
      };
      logEl.appendChild(viewBtn);

      eventSource.close();
    });

    eventSource.addEventListener('error', (e) => {
      if (e.data) {
        const data = JSON.parse(e.data);
        addLog(`Error: ${data.message}`, 'error');
      }
      eventSource.close();
    });

    eventSource.onerror = () => {
      addLog('Connection lost. Check if server is running.', 'error');
      eventSource.close();
    };

  } catch (error) {
    addLog(`Test failed: ${error.message}`, 'error');
  }

  // Add reset button
  const resetBtn = document.createElement('button');
  resetBtn.className = 'btn';
  resetBtn.style.marginTop = '1rem';
  resetBtn.style.marginLeft = '0.5rem';
  resetBtn.textContent = 'Run Another Test';
  resetBtn.onclick = () => {
    document.getElementById('new-test-form').classList.remove('hidden');
    progressPanel.classList.add('hidden');
  };

  // Wait a bit then add reset button
  setTimeout(() => {
    if (!logEl.querySelector('.btn')) {
      logEl.appendChild(resetBtn);
    }
  }, 2000);
}

// Utilities
function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}
