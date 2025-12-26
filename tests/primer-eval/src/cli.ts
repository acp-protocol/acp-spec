#!/usr/bin/env node
/**
 * ACP Primer Evaluation CLI
 *
 * Command-line interface for running primer evaluations.
 */

import { Command } from 'commander';
import { runTests, getDefaultConfig, listPrimers } from './harness.js';
import { startServer } from './server.js';
import type { RunConfig } from './types.js';

const program = new Command();

program
  .name('primer-eval')
  .description('ACP Primer Evaluation Harness')
  .version('1.0.0');

program
  .command('run')
  .description('Run primer evaluation tests')
  .option('-p, --primer <name>', 'Primer to use', 'minimal')
  .option('-c, --categories <categories>', 'Categories to run (comma-separated)')
  .option('-s, --scenarios <ids>', 'Specific scenario IDs (comma-separated)')
  .option('-j, --judge', 'Enable Claude-as-judge evaluation')
  .option('-m, --model <model>', 'Model to use', 'claude-sonnet-4-20250514')
  .option('-t, --temperature <temp>', 'Temperature', '0')
  .option('--max-tokens <tokens>', 'Max tokens for response', '1024')
  .option('--rate-limit <ms>', 'Rate limit delay in ms', '1000')
  .option('-v, --verbose', 'Verbose output')
  .option('-d, --dry-run', 'Dry run (no API calls)')
  .option('-o, --output <path>', 'Output file path for results')
  .action(async (options) => {
    const config: RunConfig = {
      ...getDefaultConfig(),
      primer: { name: options.primer },
      categories: options.categories?.split(','),
      scenarios: options.scenarios?.split(','),
      useJudge: options.judge ?? false,
      model: options.model,
      temperature: parseFloat(options.temperature),
      maxTokens: parseInt(options.maxTokens),
      rateLimitMs: parseInt(options.rateLimit),
      verbose: options.verbose ?? false,
      dryRun: options.dryRun ?? false,
      outputPath: options.output,
    };

    try {
      await runTests(config);
    } catch (error) {
      console.error('Error:', error instanceof Error ? error.message : 'Unknown error');
      process.exit(1);
    }
  });

program
  .command('list')
  .description('List available primers')
  .action(async () => {
    try {
      const primers = await listPrimers();
      console.log('Available primers:');
      for (const primer of primers) {
        console.log(`  - ${primer}`);
      }
    } catch (error) {
      console.error('Error:', error instanceof Error ? error.message : 'Unknown error');
      process.exit(1);
    }
  });

program
  .command('compare')
  .description('Compare multiple primers')
  .argument('<primers...>', 'Primers to compare (space-separated)')
  .option('-c, --categories <categories>', 'Categories to run (comma-separated)')
  .option('-j, --judge', 'Enable Claude-as-judge evaluation')
  .option('-o, --output <path>', 'Output file path for results')
  .action(async (primers: string[], options) => {
    console.log(`Comparing primers: ${primers.join(', ')}`);

    for (const primer of primers) {
      console.log(`\n${'#'.repeat(60)}`);
      console.log(`# Testing: ${primer}`);
      console.log(`${'#'.repeat(60)}`);

      const config: RunConfig = {
        ...getDefaultConfig(),
        primer: { name: primer },
        categories: options.categories?.split(','),
        useJudge: options.judge ?? false,
        outputPath: options.output ? `${options.output.replace('.json', '')}-${primer}.json` : undefined,
      };

      try {
        await runTests(config);
      } catch (error) {
        console.error(`Error testing ${primer}:`, error instanceof Error ? error.message : 'Unknown error');
      }
    }
  });

program
  .command('gui')
  .description('Start the HTML GUI server')
  .option('-p, --port <port>', 'Port to run server on', '3000')
  .action(async (options) => {
    const port = parseInt(options.port);
    console.log(`Starting GUI server on http://localhost:${port}`);
    await startServer(port);
  });

program.parse();
