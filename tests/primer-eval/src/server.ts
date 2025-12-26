/**
 * GUI Server
 *
 * Express server for the HTML GUI interface.
 */

import express from 'express';
import { readFile, readdir, writeFile, mkdir } from 'fs/promises';
import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import matter from 'gray-matter';
import { runTests, loadPrimer, loadScenarios, getDefaultConfig } from './harness.js';
import type { Primer, Scenario, TestRun, RunConfig } from './types.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT_DIR = join(__dirname, '..');

export async function startServer(port: number): Promise<void> {
  const app = express();

  app.use(express.json());
  app.use(express.static(join(ROOT_DIR, 'gui')));

  // API: List primers
  app.get('/api/primers', async (_req, res) => {
    try {
      const primersDir = join(ROOT_DIR, 'primers');
      const customDir = join(primersDir, 'custom');
      const primers: Array<{ name: string; tokens: number; description: string; isCustom: boolean }> = [];

      // Read main primers directory
      const files = await readdir(primersDir);
      for (const file of files) {
        if (!file.endsWith('.md') || file === 'README.md') continue;

        const content = await readFile(join(primersDir, file), 'utf-8');
        const { data } = matter(content);

        primers.push({
          name: file.replace('.md', ''),
          tokens: data.tokens || 0,
          description: data.description || '',
          isCustom: false,
        });
      }

      // Read custom primers directory
      if (existsSync(customDir)) {
        const customFiles = await readdir(customDir);
        for (const file of customFiles) {
          if (!file.endsWith('.md')) continue;

          const content = await readFile(join(customDir, file), 'utf-8');
          const { data } = matter(content);

          primers.push({
            name: `custom/${file.replace('.md', '')}`,
            tokens: data.tokens || 0,
            description: data.description || '',
            isCustom: true,
          });
        }
      }

      res.json(primers);
    } catch (error) {
      res.status(500).json({ error: String(error) });
    }
  });

  // API: Get primer content (supports paths like custom/my-primer)
  app.get('/api/primers/:name(*)', async (req, res) => {
    try {
      const primer = await loadPrimer({ name: req.params.name });
      res.json(primer);
    } catch (error) {
      res.status(404).json({ error: String(error) });
    }
  });

  // API: Save primer
  app.post('/api/primers/:name(*)', async (req, res) => {
    try {
      const { content, frontmatter } = req.body;
      let name = req.params.name;

      // Determine save location
      let primersDir: string;
      let fileName: string;

      if (name.startsWith('custom/')) {
        // Already a custom primer path
        primersDir = join(ROOT_DIR, 'primers', 'custom');
        fileName = name.replace('custom/', '');
      } else {
        // New primer - save to custom
        primersDir = join(ROOT_DIR, 'primers', 'custom');
        fileName = name;
      }

      if (!existsSync(primersDir)) {
        await mkdir(primersDir, { recursive: true });
      }

      // Quote values that might contain special YAML characters
      const safeName = frontmatter.name.includes(':') ? `"${frontmatter.name}"` : frontmatter.name;
      const safeDesc = frontmatter.description?.includes(':') ? `"${frontmatter.description}"` : (frontmatter.description || '');

      const fileContent = `---
name: ${safeName}
version: "${frontmatter.version || '1.0'}"
tokens: ${frontmatter.tokens || Math.ceil(content.length / 4)}
description: ${safeDesc}
tags: [${(frontmatter.tags || []).join(', ')}]
---

${content}`;

      await writeFile(join(primersDir, `${fileName}.md`), fileContent);
      res.json({ success: true, path: `custom/${fileName}` });
    } catch (error) {
      res.status(500).json({ error: String(error) });
    }
  });

  // API: List scenarios
  app.get('/api/scenarios', async (_req, res) => {
    try {
      const scenarios = await loadScenarios();
      res.json(scenarios);
    } catch (error) {
      res.status(500).json({ error: String(error) });
    }
  });

  // API: List test runs
  app.get('/api/runs', async (_req, res) => {
    try {
      const resultsDir = join(ROOT_DIR, 'results');

      if (!existsSync(resultsDir)) {
        return res.json([]);
      }

      const files = await readdir(resultsDir);
      const runs: Array<{ id: string; timestamp: string; primer: string; passRate: number }> = [];

      for (const file of files) {
        if (!file.endsWith('.json')) continue;

        const content = await readFile(join(resultsDir, file), 'utf-8');
        const data = JSON.parse(content) as TestRun;

        runs.push({
          id: data.run.id,
          timestamp: data.run.timestamp,
          primer: data.primer.name,
          passRate: data.summary.pass_rate.pattern,
        });
      }

      runs.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime());
      res.json(runs);
    } catch (error) {
      res.status(500).json({ error: String(error) });
    }
  });

  // API: Get test run details
  app.get('/api/runs/:id', async (req, res) => {
    try {
      const resultsDir = join(ROOT_DIR, 'results');
      const files = await readdir(resultsDir);

      for (const file of files) {
        if (!file.endsWith('.json')) continue;

        const content = await readFile(join(resultsDir, file), 'utf-8');
        const data = JSON.parse(content) as TestRun;

        if (data.run.id === req.params.id) {
          return res.json(data);
        }
      }

      res.status(404).json({ error: 'Run not found' });
    } catch (error) {
      res.status(500).json({ error: String(error) });
    }
  });

  // API: Run tests (streaming with SSE)
  app.get('/api/run-stream', async (req, res) => {
    // Set up SSE
    res.setHeader('Content-Type', 'text/event-stream');
    res.setHeader('Cache-Control', 'no-cache');
    res.setHeader('Connection', 'keep-alive');
    res.flushHeaders();

    const sendEvent = (event: string, data: unknown) => {
      res.write(`event: ${event}\n`);
      res.write(`data: ${JSON.stringify(data)}\n\n`);
    };

    try {
      const primer = req.query.primer as string;
      const categories = req.query.categories ? (req.query.categories as string).split(',') : undefined;
      const useJudge = req.query.judge === 'true';
      const dryRun = req.query.dryRun === 'true';

      if (!primer) {
        sendEvent('error', { message: 'Primer is required' });
        res.end();
        return;
      }

      const resultsDir = join(ROOT_DIR, 'results');
      if (!existsSync(resultsDir)) {
        await mkdir(resultsDir, { recursive: true });
      }

      const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
      const outputPath = join(resultsDir, `run-${timestamp}.json`);

      // Load primer and scenarios
      const primerData = await loadPrimer({ name: primer });
      const allScenarios = await loadScenarios(categories);

      sendEvent('start', {
        primer: primerData.name,
        tokens: primerData.tokens,
        scenarioCount: allScenarios.length,
        useJudge,
      });

      // Run each scenario and stream results
      const { runScenarioWithProgress } = await import('./harness.js');

      const config: RunConfig = {
        ...getDefaultConfig(),
        primer: { name: primer },
        categories,
        useJudge,
        dryRun,
        outputPath,
      };

      const exchanges: Awaited<ReturnType<typeof runScenarioWithProgress>>[] = [];

      for (let i = 0; i < allScenarios.length; i++) {
        const scenario = allScenarios[i];

        sendEvent('scenario-start', {
          index: i + 1,
          total: allScenarios.length,
          id: scenario.id,
          name: scenario.name,
          category: scenario.category,
        });

        try {
          const exchange = await runScenarioWithProgress(primerData, scenario, config);
          exchanges.push(exchange);

          const passed = exchange.evaluation.pattern.passed &&
            (exchange.evaluation.judge?.overall_pass !== false);

          sendEvent('scenario-complete', {
            index: i + 1,
            total: allScenarios.length,
            id: scenario.id,
            name: scenario.name,
            passed,
            score: exchange.evaluation.pattern.score,
            tokens: exchange.response.tokens,
            latency: exchange.response.latency_ms,
            response: exchange.response.content.substring(0, 500) + (exchange.response.content.length > 500 ? '...' : ''),
          });
        } catch (error) {
          sendEvent('scenario-error', {
            index: i + 1,
            id: scenario.id,
            name: scenario.name,
            error: String(error),
          });
        }
      }

      // Save results and send completion
      const { computeSummaryFromExchanges, saveTestRun } = await import('./harness.js');
      const summary = computeSummaryFromExchanges(exchanges);
      const testRun = await saveTestRun(primerData, exchanges, summary, config);

      sendEvent('complete', {
        runId: testRun.run.id,
        summary: testRun.summary,
      });

      res.end();
    } catch (error) {
      sendEvent('error', { message: String(error) });
      res.end();
    }
  });

  // API: Run tests (non-streaming fallback)
  app.post('/api/run', async (req, res) => {
    try {
      const { primer, categories, useJudge, dryRun } = req.body;

      const resultsDir = join(ROOT_DIR, 'results');
      if (!existsSync(resultsDir)) {
        await mkdir(resultsDir, { recursive: true });
      }

      const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
      const outputPath = join(resultsDir, `run-${timestamp}.json`);

      const config: RunConfig = {
        ...getDefaultConfig(),
        primer: { name: primer },
        categories,
        useJudge: useJudge ?? false,
        dryRun: dryRun ?? false,
        outputPath,
      };

      const result = await runTests(config);
      res.json(result);
    } catch (error) {
      res.status(500).json({ error: String(error) });
    }
  });

  app.listen(port, () => {
    console.log(`GUI available at http://localhost:${port}`);
  });
}
