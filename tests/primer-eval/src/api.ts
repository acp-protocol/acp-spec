/**
 * Anthropic API Client
 *
 * Handles all communication with the Anthropic API.
 */

import Anthropic from '@anthropic-ai/sdk';
import type { ApiCallOptions, ApiResponse, HarnessConfig } from './types.js';

// Default configuration
export const DEFAULT_CONFIG: HarnessConfig = {
  defaultModel: 'claude-sonnet-4-20250514',
  defaultMaxTokens: 1024,
  defaultTemperature: 0,
  defaultRateLimitMs: 1000,
  pricing: {
    // Pricing per 1K tokens (as of late 2024)
    input: 0.003,
    output: 0.015,
  },
};

/**
 * API Client for Anthropic
 */
export class AnthropicClient {
  private client: Anthropic;
  private config: HarnessConfig;

  constructor(config: Partial<HarnessConfig> = {}) {
    const apiKey = process.env.ANTHROPIC_API_KEY;
    if (!apiKey) {
      throw new Error('ANTHROPIC_API_KEY environment variable is required');
    }

    this.client = new Anthropic({ apiKey });
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Call Claude with a system prompt and user message
   */
  async call(
    systemPrompt: string,
    userMessage: string,
    options: ApiCallOptions = {}
  ): Promise<ApiResponse> {
    const model = options.model || this.config.defaultModel;
    const maxTokens = options.maxTokens || this.config.defaultMaxTokens;
    const temperature = options.temperature ?? this.config.defaultTemperature;

    const startTime = Date.now();

    const response = await this.client.messages.create({
      model,
      max_tokens: maxTokens,
      temperature,
      system: systemPrompt,
      messages: [{ role: 'user', content: userMessage }],
    });

    const latency = Date.now() - startTime;

    // Extract text content
    const content = response.content
      .filter((block): block is Anthropic.TextBlock => block.type === 'text')
      .map((block) => block.text)
      .join('\n');

    return {
      content,
      stop_reason: response.stop_reason || 'end_turn',
      usage: {
        input_tokens: response.usage.input_tokens,
        output_tokens: response.usage.output_tokens,
      },
      id: response.id,
      model: response.model,
    };
  }

  /**
   * Estimate cost for a given token usage
   */
  estimateCost(inputTokens: number, outputTokens: number): number {
    const inputCost = (inputTokens / 1000) * this.config.pricing.input;
    const outputCost = (outputTokens / 1000) * this.config.pricing.output;
    return inputCost + outputCost;
  }

  /**
   * Get the default model
   */
  getDefaultModel(): string {
    return this.config.defaultModel;
  }

  /**
   * Get rate limit delay in ms
   */
  getRateLimitMs(): number {
    return this.config.defaultRateLimitMs;
  }
}

/**
 * Sleep for a given number of milliseconds
 */
export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Create a singleton client instance
 */
let clientInstance: AnthropicClient | null = null;

export function getClient(config?: Partial<HarnessConfig>): AnthropicClient {
  if (!clientInstance) {
    clientInstance = new AnthropicClient(config);
  }
  return clientInstance;
}

export function resetClient(): void {
  clientInstance = null;
}
