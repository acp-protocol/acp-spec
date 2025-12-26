/**
 * ACP Primer Evaluation Types
 *
 * Comprehensive type definitions for the primer evaluation harness.
 */

// ============================================================================
// Primer Types
// ============================================================================

export interface Primer {
  /** Primer name/identifier */
  name: string;
  /** Version string */
  version: string;
  /** Estimated token count */
  tokens: number;
  /** Description of the primer */
  description: string;
  /** Tags for categorization */
  tags: string[];
  /** The actual primer text content */
  text: string;
  /** Source: 'static' (from file) or 'dynamic' (from daemon) */
  source: 'static' | 'dynamic';
  /** Path to the primer file (if static) */
  filePath?: string;
}

export interface PrimerConfig {
  /** Primer name or path */
  name: string;
  /** Token budget for dynamic primers */
  tokenBudget?: number;
  /** Preset for dynamic primers: 'safe', 'efficient', 'accurate', 'balanced' */
  preset?: string;
  /** Capabilities for filtering */
  capabilities?: string[];
  /** Project root for daemon integration */
  projectRoot?: string;
}

// ============================================================================
// Scenario Types
// ============================================================================

export interface Scenario {
  /** Unique scenario ID */
  id: string;
  /** Human-readable name */
  name: string;
  /** Description of what's being tested */
  description: string;
  /** Category for grouping */
  category: string;
  /** Difficulty level */
  difficulty: 'easy' | 'medium' | 'hard';
  /** The user message to send */
  userMessage: string;
  /** Expected behavior description (for judge) */
  expectedBehavior: string;
  /** Pattern-based evaluation criteria */
  criteria: Criterion[];
  /** Tags for filtering */
  tags?: string[];
}

export interface MultiTurnScenario extends Omit<Scenario, 'userMessage' | 'criteria'> {
  /** Multiple conversation turns */
  turns: ConversationTurn[];
}

export interface ConversationTurn {
  /** Optional context to set before this turn */
  context?: string;
  /** User message for this turn */
  user: string;
  /** Expected behavior for this turn */
  expected: string;
  /** Optional criteria for this specific turn */
  criteria?: Criterion[];
}

export interface Criterion {
  /** Unique criterion ID */
  id: string;
  /** Human-readable description */
  description: string;
  /** Criterion type */
  type: 'contains' | 'not_contains' | 'regex' | 'not_regex' | 'length_max' | 'starts_with';
  /** Pattern to match (for pattern-based types) */
  pattern?: string;
  /** Max length (for length_max type) */
  maxLength?: number;
  /** Is this criterion required for pass? */
  required: boolean;
  /** Weight for scoring (default: 1) */
  weight?: number;
}

// ============================================================================
// Request/Response Types
// ============================================================================

export interface Request {
  /** System prompt (primer) */
  system_prompt: string;
  /** User message */
  user_message: string;
  /** Timestamp */
  timestamp: string;
}

export interface Response {
  /** Full response content (never truncated) */
  content: string;
  /** Stop reason from API */
  stop_reason: string;
  /** Token counts */
  tokens: {
    input: number;
    output: number;
  };
  /** Latency in milliseconds */
  latency_ms: number;
  /** API request ID */
  request_id: string;
  /** Model used */
  model: string;
}

// ============================================================================
// Evaluation Types
// ============================================================================

export interface PatternResult {
  /** Criterion that was evaluated */
  criterion: Criterion;
  /** Did it pass? */
  passed: boolean;
  /** Match found (if passed) */
  match?: string;
  /** Violation found (if failed) */
  violation?: string;
}

export interface PatternEvaluation {
  /** Overall pass based on required criteria */
  passed: boolean;
  /** Score (0-1) based on weighted criteria */
  score: number;
  /** Individual criterion results */
  criteria: PatternResult[];
}

export interface JudgeScores {
  /** Did AI correctly identify constraint annotations? (1-5) */
  constraint_recognition: number;
  /** Did AI follow constraints appropriately? (1-5) */
  constraint_compliance: number;
  /** Did AI suggest appropriate ACP commands? (1-5) */
  acp_command_usage: number;
  /** Was response helpful while respecting constraints? (1-5) */
  helpfulness: number;
  /** Did AI avoid unsafe actions? (1-5) */
  safety: number;
}

export interface JudgeEvaluation {
  /** Scores on each dimension */
  scores: JudgeScores;
  /** Overall pass/fail determination */
  overall_pass: boolean;
  /** Detailed explanation of evaluation */
  explanation: string;
  /** Suggestions for primer improvement */
  suggestions: string[];
}

export interface Evaluation {
  /** Pattern-based evaluation results */
  pattern: PatternEvaluation;
  /** Claude-as-judge evaluation results (optional) */
  judge?: JudgeEvaluation;
}

// ============================================================================
// Exchange Types
// ============================================================================

export interface Exchange {
  /** Scenario that was run */
  scenario: {
    id: string;
    name: string;
    category: string;
    difficulty: string;
  };
  /** Request sent to API */
  request: Request;
  /** Response received from API */
  response: Response;
  /** Evaluation results */
  evaluation: Evaluation;
}

// ============================================================================
// Test Run Types
// ============================================================================

export interface RunMetadata {
  /** Unique run ID */
  id: string;
  /** Timestamp of run */
  timestamp: string;
  /** Model used */
  model: string;
  /** Harness version */
  harness_version: string;
}

export interface Summary {
  /** Total scenarios run */
  total: number;
  /** Pass counts */
  passed: {
    pattern: number;
    judge: number;
  };
  /** Pass rates as percentages */
  pass_rate: {
    pattern: number;
    judge: number;
    combined: number;
  };
  /** Token usage */
  tokens: {
    input: number;
    output: number;
    total: number;
  };
  /** Timing */
  latency: {
    total_ms: number;
    avg_ms: number;
  };
  /** Estimated cost in USD */
  cost_estimate_usd: number;
}

export interface TestRun {
  /** Run metadata */
  run: RunMetadata;
  /** Primer configuration */
  primer: Primer;
  /** All exchanges */
  exchanges: Exchange[];
  /** Summary statistics */
  summary: Summary;
}

// ============================================================================
// Configuration Types
// ============================================================================

export interface RunConfig {
  /** Primer to use */
  primer: PrimerConfig;
  /** Categories to run (null = all) */
  categories?: string[];
  /** Specific scenario IDs to run */
  scenarios?: string[];
  /** Use Claude-as-judge evaluation */
  useJudge: boolean;
  /** Model to use */
  model: string;
  /** Max tokens for response */
  maxTokens: number;
  /** Temperature */
  temperature: number;
  /** Rate limit delay in ms */
  rateLimitMs: number;
  /** Verbose output */
  verbose: boolean;
  /** Dry run (no API calls) */
  dryRun: boolean;
  /** Output file path */
  outputPath?: string;
}

export interface HarnessConfig {
  /** Default model */
  defaultModel: string;
  /** Default max tokens */
  defaultMaxTokens: number;
  /** Default temperature */
  defaultTemperature: number;
  /** Default rate limit delay */
  defaultRateLimitMs: number;
  /** Anthropic pricing per 1K tokens */
  pricing: {
    input: number;
    output: number;
  };
}

// ============================================================================
// API Types
// ============================================================================

export interface ApiCallOptions {
  model?: string;
  maxTokens?: number;
  temperature?: number;
}

export interface ApiResponse {
  content: string;
  stop_reason: string;
  usage: {
    input_tokens: number;
    output_tokens: number;
  };
  id: string;
  model: string;
}
