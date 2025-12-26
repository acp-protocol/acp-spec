/**
 * ACP Primer Evaluation Harness
 *
 * Main test runner for evaluating primer effectiveness.
 */

import { readFile, readdir, writeFile, mkdir } from 'fs/promises';
import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import matter from 'gray-matter';
import { nanoid } from 'nanoid';

import { getClient, sleep, DEFAULT_CONFIG } from './api.js';
import { evaluateWithJudge } from './judge.js';
import type {
  Primer,
  PrimerConfig,
  Scenario,
  Criterion,
  Exchange,
  Request,
  Response,
  PatternResult,
  PatternEvaluation,
  Evaluation,
  TestRun,
  RunConfig,
  Summary,
} from './types.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT_DIR = join(__dirname, '..');

// ============================================================================
// Primer Loading
// ============================================================================

/**
 * Load a primer from a markdown file
 */
export async function loadPrimer(config: PrimerConfig): Promise<Primer> {
  const primersDir = join(ROOT_DIR, 'primers');

  // Try to find the primer file
  const possiblePaths: string[] = [];

  // If name already includes path separator, use it directly
  if (config.name.includes('/')) {
    possiblePaths.push(join(primersDir, `${config.name}.md`));
  } else {
    // Try main directory first, then custom
    possiblePaths.push(join(primersDir, `${config.name}.md`));
    possiblePaths.push(join(primersDir, 'custom', `${config.name}.md`));
  }

  // Also try as absolute path
  possiblePaths.push(config.name);

  let filePath: string | null = null;
  for (const p of possiblePaths) {
    if (existsSync(p)) {
      filePath = p;
      break;
    }
  }

  if (!filePath) {
    throw new Error(`Primer not found: ${config.name}`);
  }

  const content = await readFile(filePath, 'utf-8');
  const { data, content: text } = matter(content);

  return {
    name: data.name || config.name,
    version: data.version || '1.0',
    tokens: data.tokens || estimateTokens(text),
    description: data.description || '',
    tags: data.tags || [],
    text: text.trim(),
    source: 'static',
    filePath,
  };
}

/**
 * List all available primers
 */
export async function listPrimers(): Promise<string[]> {
  const primersDir = join(ROOT_DIR, 'primers');
  const files = await readdir(primersDir);
  return files
    .filter((f) => f.endsWith('.md') && f !== 'README.md')
    .map((f) => f.replace('.md', ''));
}

/**
 * Estimate token count for text (rough approximation: 4 chars per token)
 */
function estimateTokens(text: string): number {
  return Math.ceil(text.length / 4);
}

// ============================================================================
// Scenario Loading
// ============================================================================

/**
 * Load scenarios from JSON files
 */
export async function loadScenarios(categories?: string[]): Promise<Scenario[]> {
  const scenariosDir = join(ROOT_DIR, 'scenarios');
  const files = await readdir(scenariosDir);

  const scenarios: Scenario[] = [];

  for (const file of files) {
    if (!file.endsWith('.json')) continue;

    const content = await readFile(join(scenariosDir, file), 'utf-8');
    const data = JSON.parse(content);

    // Handle both array and object with scenarios property
    const scenarioList: Scenario[] = Array.isArray(data) ? data : data.scenarios || [];

    for (const scenario of scenarioList) {
      // Filter by category if specified
      if (categories && categories.length > 0) {
        if (!categories.includes(scenario.category)) continue;
      }
      scenarios.push(scenario);
    }
  }

  return scenarios;
}

// ============================================================================
// Pattern Evaluation
// ============================================================================

/**
 * Evaluate response against pattern-based criteria
 */
export function evaluatePatterns(response: string, criteria: Criterion[]): PatternEvaluation {
  const results: PatternResult[] = criteria.map((criterion) => {
    const result = evaluateCriterion(response, criterion);
    return {
      criterion,
      ...result,
    };
  });

  // Check if all required criteria passed
  const requiredResults = results.filter((r) => r.criterion.required);
  const passed = requiredResults.every((r) => r.passed);

  // Calculate weighted score
  const totalWeight = results.reduce((sum, r) => sum + (r.criterion.weight || 1), 0);
  const passedWeight = results
    .filter((r) => r.passed)
    .reduce((sum, r) => sum + (r.criterion.weight || 1), 0);
  const score = totalWeight > 0 ? passedWeight / totalWeight : 0;

  return { passed, score, criteria: results };
}

/**
 * Evaluate a single criterion
 */
function evaluateCriterion(
  response: string,
  criterion: Criterion
): { passed: boolean; match?: string; violation?: string } {
  const responseLower = response.toLowerCase();

  switch (criterion.type) {
    case 'contains': {
      const patternLower = criterion.pattern!.toLowerCase();
      const idx = responseLower.indexOf(patternLower);
      if (idx >= 0) {
        const match = response.substring(idx, idx + criterion.pattern!.length);
        return { passed: true, match };
      }
      return { passed: false };
    }

    case 'not_contains': {
      const patternLower = criterion.pattern!.toLowerCase();
      const idx = responseLower.indexOf(patternLower);
      if (idx >= 0) {
        const violation = response.substring(idx, idx + criterion.pattern!.length);
        return { passed: false, violation };
      }
      return { passed: true };
    }

    case 'regex': {
      const regex = new RegExp(criterion.pattern!, 'i');
      const match = response.match(regex);
      if (match) {
        return { passed: true, match: match[0] };
      }
      return { passed: false };
    }

    case 'not_regex': {
      const regex = new RegExp(criterion.pattern!, 'i');
      const match = response.match(regex);
      if (match) {
        return { passed: false, violation: match[0] };
      }
      return { passed: true };
    }

    case 'length_max': {
      if (response.length <= criterion.maxLength!) {
        return { passed: true };
      }
      return { passed: false, violation: `Response length ${response.length} exceeds max ${criterion.maxLength}` };
    }

    case 'starts_with': {
      const patternLower = criterion.pattern!.toLowerCase();
      if (responseLower.startsWith(patternLower)) {
        return { passed: true, match: response.substring(0, criterion.pattern!.length) };
      }
      return { passed: false };
    }

    default:
      return { passed: false, violation: `Unknown criterion type: ${criterion.type}` };
  }
}

// ============================================================================
// Test Execution
// ============================================================================

/**
 * Run a single scenario
 */
export async function runScenario(
  primer: Primer,
  scenario: Scenario,
  config: RunConfig
): Promise<Exchange> {
  const client = getClient();

  const request: Request = {
    system_prompt: primer.text,
    user_message: scenario.userMessage,
    timestamp: new Date().toISOString(),
  };

  if (config.dryRun) {
    // Return mock exchange for dry run
    return {
      scenario: {
        id: scenario.id,
        name: scenario.name,
        category: scenario.category,
        difficulty: scenario.difficulty,
      },
      request,
      response: {
        content: '[DRY RUN - No API call made]',
        stop_reason: 'dry_run',
        tokens: { input: estimateTokens(primer.text + scenario.userMessage), output: 0 },
        latency_ms: 0,
        request_id: 'dry_run',
        model: config.model,
      },
      evaluation: {
        pattern: { passed: false, score: 0, criteria: [] },
      },
    };
  }

  // Make actual API call
  const startTime = Date.now();
  const apiResponse = await client.call(primer.text, scenario.userMessage, {
    model: config.model,
    maxTokens: config.maxTokens,
    temperature: config.temperature,
  });

  const response: Response = {
    content: apiResponse.content,
    stop_reason: apiResponse.stop_reason,
    tokens: {
      input: apiResponse.usage.input_tokens,
      output: apiResponse.usage.output_tokens,
    },
    latency_ms: Date.now() - startTime,
    request_id: apiResponse.id,
    model: apiResponse.model,
  };

  // Evaluate patterns
  const patternEval = evaluatePatterns(response.content, scenario.criteria);

  // Optionally evaluate with judge
  let evaluation: Evaluation = { pattern: patternEval };

  if (config.useJudge) {
    const exchange: Exchange = {
      scenario: {
        id: scenario.id,
        name: scenario.name,
        category: scenario.category,
        difficulty: scenario.difficulty,
      },
      request,
      response,
      evaluation: { pattern: patternEval },
    };

    const judgeEval = await evaluateWithJudge(exchange, scenario.expectedBehavior);
    evaluation.judge = judgeEval;
  }

  return {
    scenario: {
      id: scenario.id,
      name: scenario.name,
      category: scenario.category,
      difficulty: scenario.difficulty,
    },
    request,
    response,
    evaluation,
  };
}

/**
 * Run all tests with given configuration
 */
export async function runTests(config: RunConfig): Promise<TestRun> {
  const primer = await loadPrimer(config.primer);
  const scenarios = await loadScenarios(config.categories);

  // Filter by specific scenario IDs if provided
  let filteredScenarios = scenarios;
  if (config.scenarios && config.scenarios.length > 0) {
    filteredScenarios = scenarios.filter((s) => config.scenarios!.includes(s.id));
  }

  if (filteredScenarios.length === 0) {
    throw new Error('No scenarios to run');
  }

  const client = getClient();
  const exchanges: Exchange[] = [];

  console.log(`\n${'='.repeat(60)}`);
  console.log(`Testing: ${primer.name} (~${primer.tokens} tokens)`);
  console.log(`Scenarios: ${filteredScenarios.length}`);
  console.log(`Judge: ${config.useJudge ? 'enabled' : 'disabled'}`);
  console.log(`${'='.repeat(60)}\n`);

  for (const scenario of filteredScenarios) {
    process.stdout.write(`  ${scenario.name.padEnd(35)}`);

    try {
      const exchange = await runScenario(primer, scenario, config);
      exchanges.push(exchange);

      if (config.dryRun) {
        console.log('[DRY RUN]');
      } else {
        const patternPassed = exchange.evaluation.pattern.passed;
        const judgePassed = exchange.evaluation.judge?.overall_pass ?? true;
        const passed = patternPassed && judgePassed;

        const patternScore = (exchange.evaluation.pattern.score * 100).toFixed(0);

        if (passed) {
          console.log(`✅ PASS (${patternScore}%)`);
        } else {
          console.log(`❌ FAIL (${patternScore}%)`);

          if (config.verbose) {
            // Show failed criteria
            const failed = exchange.evaluation.pattern.criteria.filter(
              (c) => !c.passed && c.criterion.required
            );
            for (const f of failed) {
              console.log(`     ↳ ${f.criterion.description}`);
            }

            // Show judge feedback
            if (exchange.evaluation.judge && !exchange.evaluation.judge.overall_pass) {
              console.log(`     ↳ Judge: ${exchange.evaluation.judge.explanation}`);
            }
          }
        }
      }

      // Rate limiting
      if (!config.dryRun) {
        await sleep(config.rateLimitMs);
      }
    } catch (error) {
      console.log(`❌ ERROR: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  // Calculate summary
  const summary = computeSummary(exchanges, client);

  // Print summary
  console.log(`\n${'-'.repeat(40)}`);
  console.log(`Results: ${summary.passed.pattern}/${summary.total} passed (pattern)`);
  if (config.useJudge) {
    console.log(`         ${summary.passed.judge}/${summary.total} passed (judge)`);
  }
  console.log(`Pass rate: ${summary.pass_rate.pattern.toFixed(1)}% (pattern)`);
  if (config.useJudge) {
    console.log(`           ${summary.pass_rate.judge.toFixed(1)}% (judge)`);
  }
  console.log(`Tokens: ${summary.tokens.total} (${summary.tokens.input} in, ${summary.tokens.output} out)`);
  console.log(`Cost: $${summary.cost_estimate_usd.toFixed(4)}`);

  const runId = `run-${new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)}`;

  const testRun: TestRun = {
    run: {
      id: runId,
      timestamp: new Date().toISOString(),
      model: config.model,
      harness_version: '1.0.0',
    },
    primer,
    exchanges,
    summary,
  };

  // Write results to file if output path specified
  if (config.outputPath) {
    const outputDir = dirname(config.outputPath);
    if (!existsSync(outputDir)) {
      await mkdir(outputDir, { recursive: true });
    }
    await writeFile(config.outputPath, JSON.stringify(testRun, null, 2));
    console.log(`\nResults written to: ${config.outputPath}`);
  }

  return testRun;
}

/**
 * Compute summary statistics from exchanges
 */
function computeSummary(
  exchanges: Exchange[],
  client: ReturnType<typeof getClient>
): Summary {
  const total = exchanges.length;

  const patternPassed = exchanges.filter((e) => e.evaluation.pattern.passed).length;
  const judgePassed = exchanges.filter((e) => e.evaluation.judge?.overall_pass ?? true).length;

  const inputTokens = exchanges.reduce((sum, e) => sum + e.response.tokens.input, 0);
  const outputTokens = exchanges.reduce((sum, e) => sum + e.response.tokens.output, 0);
  const totalTokens = inputTokens + outputTokens;

  const totalLatency = exchanges.reduce((sum, e) => sum + e.response.latency_ms, 0);
  const avgLatency = total > 0 ? totalLatency / total : 0;

  const cost = client.estimateCost(inputTokens, outputTokens);

  return {
    total,
    passed: {
      pattern: patternPassed,
      judge: judgePassed,
    },
    pass_rate: {
      pattern: total > 0 ? (patternPassed / total) * 100 : 0,
      judge: total > 0 ? (judgePassed / total) * 100 : 0,
      combined: total > 0 ? ((patternPassed + judgePassed) / (total * 2)) * 100 : 0,
    },
    tokens: {
      input: inputTokens,
      output: outputTokens,
      total: totalTokens,
    },
    latency: {
      total_ms: totalLatency,
      avg_ms: avgLatency,
    },
    cost_estimate_usd: cost,
  };
}

// ============================================================================
// Default Configuration
// ============================================================================

export function getDefaultConfig(): RunConfig {
  return {
    primer: { name: 'minimal' },
    categories: undefined,
    scenarios: undefined,
    useJudge: false,
    model: DEFAULT_CONFIG.defaultModel,
    maxTokens: DEFAULT_CONFIG.defaultMaxTokens,
    temperature: DEFAULT_CONFIG.defaultTemperature,
    rateLimitMs: DEFAULT_CONFIG.defaultRateLimitMs,
    verbose: false,
    dryRun: false,
  };
}

// ============================================================================
// Streaming Support Functions
// ============================================================================

/**
 * Run a single scenario (alias for streaming support)
 */
export const runScenarioWithProgress = runScenario;

/**
 * Compute summary from exchanges (for streaming)
 */
export function computeSummaryFromExchanges(exchanges: Exchange[]): Summary {
  const client = getClient();
  return computeSummary(exchanges, client);
}

/**
 * Save test run to file
 */
export async function saveTestRun(
  primer: Primer,
  exchanges: Exchange[],
  summary: Summary,
  config: RunConfig
): Promise<TestRun> {
  const runId = `run-${new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19)}`;

  const testRun: TestRun = {
    run: {
      id: runId,
      timestamp: new Date().toISOString(),
      model: config.model,
      harness_version: '1.0.0',
    },
    primer,
    exchanges,
    summary,
  };

  if (config.outputPath) {
    const outputDir = dirname(config.outputPath);
    if (!existsSync(outputDir)) {
      await mkdir(outputDir, { recursive: true });
    }
    await writeFile(config.outputPath, JSON.stringify(testRun, null, 2));
  }

  return testRun;
}
