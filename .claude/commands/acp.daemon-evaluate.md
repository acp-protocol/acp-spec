---
description: Evaluate the ACP Daemon architecture against the project's stated goals and design principles from the specification. Provides value analysis and alignment assessment.
handoffs:
  - label: Back to Daemon Docs
    agent: acp.daemon-document
    prompt: Regenerate detailed documentation
  - label: Implementation Roadmap
    agent: acp.daemon-roadmap
    prompt: Create implementation roadmap based on value analysis
    send: true
  - label: Alternative Analysis
    agent: acp.daemon-alternatives
    prompt: Analyze alternative approaches to the daemon
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--strict` for rigorous alignment checking
- `--gaps` to focus on gaps between daemon and goals
- `--recommendations` to include improvement suggestions
- `--component <n>` to evaluate specific component

## Purpose

This command evaluates the ACP Daemon architecture against the project's **stated goals** from the specification. It determines:

1. **Goal Alignment** - How well does the daemon serve project objectives?
2. **Value Contribution** - What unique value does the daemon provide?
3. **Design Principle Adherence** - Does it follow ACP's design philosophy?
4. **Gap Analysis** - Where does the daemon fall short?
5. **ROI Assessment** - Is the complexity justified by the benefits?

## Project Goals Reference

### From ACP Specification (Chapter 1 - Introduction)

The ACP specification states these **core objectives**:

| Goal ID | Goal | Description |
|---------|------|-------------|
| G1 | **Token Efficiency** | Minimize context sent to AI systems |
| G2 | **Deterministic Context** | Same codebase produces identical context |
| G3 | **Version Controlled** | Context is code, stored in repo |
| G4 | **Offline Capable** | Works without network connection |
| G5 | **Tool Agnostic** | Works with any AI tool |
| G6 | **Safety First** | Constraint information always preserved |
| G7 | **Transparent Integration** | Zero-friction adoption |
| G8 | **Query Optimized** | Efficient O(1) lookups |

### From Tool Integration (Chapter 13)

Additional **design principles**:

| Principle ID | Principle | Description |
|--------------|-----------|-------------|
| P1 | **Transparency** | No manual prompt copying or MCP configuration |
| P2 | **Universality** | Work with any tool that has auto-read mechanism |
| P3 | **Freshness** | Context stays synchronized as codebase evolves |
| P4 | **Efficiency** | Respect token budgets through intelligent selection |
| P5 | **Safety-First** | Critical constraint information always included |

---

## Evaluation Framework

### 1. Goal-by-Goal Analysis

For each project goal, evaluate daemon contribution:

#### G1: Token Efficiency

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Proxy | Minimal bootstrap injection (~50-80 tokens) | ⭐⭐⭐⭐⭐ |
| Primer | Budget-aware section selection | ⭐⭐⭐⭐⭐ |
| Sync | Per-tool optimized output | ⭐⭐⭐⭐ |
| Watch | Incremental updates (avoids full re-index) | ⭐⭐⭐⭐ |
| MCP | On-demand queries (no upfront context) | ⭐⭐⭐⭐⭐ |

**Analysis**:
- Proxy approach reduces context from ~500-4000 tokens (file sync) to ~50-80 tokens
- Primer system uses multi-dimensional value scoring to maximize ROI per token
- MCP provides zero-token baseline with on-demand expansion

**Alignment**: ✅ **STRONG** - Daemon architecture directly serves token efficiency

---

#### G2: Deterministic Context

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Cache | Deterministic JSON output (sorted keys) | ⭐⭐⭐⭐⭐ |
| Sync | Reproducible tool file generation | ⭐⭐⭐⭐ |
| Primer | Deterministic section selection algorithm | ⭐⭐⭐⭐⭐ |
| Watch | Trigger re-index (maintains determinism) | ⭐⭐⭐⭐ |

**Analysis**:
- Cache spec requires deterministic output (sorted keys, consistent ordering)
- Same inputs always produce same primer output given same weights
- Watch component triggers re-index but doesn't affect determinism of output

**Alignment**: ✅ **STRONG** - Daemon maintains deterministic behavior

---

#### G3: Version Controlled

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Sync | Generates committable tool files | ⭐⭐⭐⭐⭐ |
| Cache | Static JSON files (can be committed) | ⭐⭐⭐⭐⭐ |
| Config | Version-controlled configuration | ⭐⭐⭐⭐⭐ |
| Proxy | Runtime-only (no version control impact) | N/A |

**Analysis**:
- `.cursorrules`, `CLAUDE.md`, `AGENTS.md` can all be committed
- Cache files are designed to be regenerated but can be committed
- Proxy is ephemeral, doesn't impact version control

**Alignment**: ✅ **STRONG** - Produces version-controllable artifacts

---

#### G4: Offline Capable

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Sync | Generates local files (offline OK) | ⭐⭐⭐⭐⭐ |
| Cache | Local JSON files (offline OK) | ⭐⭐⭐⭐⭐ |
| Watch | Local filesystem events (offline OK) | ⭐⭐⭐⭐⭐ |
| Proxy | **Requires network** (to upstream API) | ⭐⭐ |
| MCP | Varies (local or network) | ⭐⭐⭐ |

**Analysis**:
- File sync approach works completely offline
- Proxy requires network to reach upstream LLM APIs
- However, proxy can function as pass-through when offline (no injection)

**Alignment**: ⚠️ **PARTIAL** - File sync is offline-capable, proxy requires network

**Mitigation**: Spec acknowledges proxy vs sync tradeoff:
> "Fall back to file sync for offline development"

---

#### G5: Tool Agnostic

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Proxy | Works with **any** tool that accepts custom API URL | ⭐⭐⭐⭐⭐ |
| Sync | Adapters for major tools, extensible | ⭐⭐⭐⭐ |
| MCP | Works with MCP-compatible tools | ⭐⭐⭐ |
| Cache | Tool-agnostic JSON format | ⭐⭐⭐⭐⭐ |

**Analysis**:
- Proxy is the most tool-agnostic approach (any tool with API URL config)
- Sync requires adapters but supports major tools
- MCP is limited to MCP-compatible clients
- Cache is universal JSON

**Alignment**: ✅ **STRONG** - Multiple integration paths cover most tools

---

#### G6: Safety First

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Primer | Safety dimension in value scoring | ⭐⭐⭐⭐⭐ |
| Proxy | Always injects constraint reminder | ⭐⭐⭐⭐⭐ |
| MCP | `acp_constraints` tool for checking | ⭐⭐⭐⭐⭐ |
| Sync | Required sections always included | ⭐⭐⭐⭐⭐ |

**Analysis**:
- Primer system uses `required: true` for critical sections
- Safety dimension has highest weight in default scoring
- Proxy bootstrap always mentions constraint checking
- MCP provides explicit constraint checking tool

**Alignment**: ✅ **STRONG** - Safety is architecturally prioritized

---

#### G7: Transparent Integration

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Sync | Auto-runs with `acp init` | ⭐⭐⭐⭐⭐ |
| Watch | Auto-sync on changes with `--sync` flag | ⭐⭐⭐⭐ |
| Proxy | Requires manual startup and tool config | ⭐⭐⭐ |

**Analysis**:
- `acp init` automatically runs sync, generating all tool files
- Watch mode can auto-sync when cache changes
- Proxy requires explicit startup and tool configuration
- Future unified daemon would improve this (auto-start)

**Alignment**: ⚠️ **PARTIAL** - Sync is transparent, proxy requires setup

**Improvement Opportunity**: Unified daemon with auto-start would achieve full transparency

---

#### G8: Query Optimized

**Daemon Contribution**:

| Component | Contribution | Score |
|-----------|--------------|-------|
| Cache | HashMap-based O(1) lookups | ⭐⭐⭐⭐⭐ |
| MCP | Direct query interface | ⭐⭐⭐⭐⭐ |
| Vars | Qualified name indexing | ⭐⭐⭐⭐⭐ |
| Watch | Incremental updates (maintains index) | ⭐⭐⭐⭐ |

**Analysis**:
- Cache uses HashMap for files, symbols, domains
- MCP provides query tools with O(1) cache access
- Watch maintains index freshness without full rebuild

**Alignment**: ✅ **STRONG** - Query-optimized architecture throughout

---

### 2. Design Principle Adherence

#### P1: Transparency (No Manual Configuration)

| Aspect | Status | Notes |
|--------|--------|-------|
| Tool detection | ✅ Auto | Detects installed tools |
| File generation | ✅ Auto | `acp init` generates all |
| Proxy setup | ⚠️ Manual | Requires `acp proxy` and tool config |
| MCP setup | ⚠️ Manual | Requires config file edit |

**Score**: ⭐⭐⭐ (3/5) - File sync transparent, proxy/MCP require setup

---

#### P2: Universality (Any Tool)

| Approach | Coverage | Limitations |
|----------|----------|-------------|
| Proxy | Any tool with API URL | Requires proxy running |
| File sync | Tools with auto-read files | Needs adapter per tool |
| MCP | MCP-compatible tools | Limited ecosystem |

**Score**: ⭐⭐⭐⭐ (4/5) - Multiple approaches cover most scenarios

---

#### P3: Freshness (Synchronized Context)

| Component | Freshness | Notes |
|-----------|-----------|-------|
| Watch + Sync | Real-time | Auto-syncs on file change |
| Proxy | Always current | Reads cache at request time |
| MCP | Always current | Queries live cache |
| Static files | May be stale | If sync not run after changes |

**Score**: ⭐⭐⭐⭐ (4/5) - Good freshness with watch mode

---

#### P4: Efficiency (Token Budgets)

| Component | Efficiency | Notes |
|-----------|------------|-------|
| Primer | Budget-aware selection | Multi-dimensional scoring |
| Proxy | Minimal injection | ~50-80 tokens |
| Sync | Tool-specific optimization | Respects per-tool budgets |

**Score**: ⭐⭐⭐⭐⭐ (5/5) - Excellent token efficiency design

---

#### P5: Safety-First (Constraints Always)

| Mechanism | Implementation |
|-----------|---------------|
| Required sections | `required: true` in primer schema |
| Safety dimension | Highest weight in scoring algorithm |
| Minimum budget | Enforced for constraint sections |
| Proxy reminder | Always injects constraint check note |

**Score**: ⭐⭐⭐⭐⭐ (5/5) - Safety architecturally guaranteed

---

### 3. Value Contribution Analysis

#### Unique Value Provided by Daemon Architecture

| Value | Without Daemon | With Daemon | Delta |
|-------|----------------|-------------|-------|
| Context freshness | Manual re-sync | Automatic | ⬆️ High |
| Token efficiency | Static files (~1000 tokens) | Proxy (~50 tokens) | ⬆️ 20x better |
| Tool coverage | Manual per-tool | Universal proxy | ⬆️ High |
| Setup friction | Per-tool config | `acp init` | ⬆️ Much lower |
| Query latency | File reads | In-memory cache | ⬆️ Faster |
| Constraint checking | Manual lookup | Automatic reminder | ⬆️ Safer |

---

### 4. Gap Analysis

#### Gaps Between Current State and Goals

| Gap ID | Description | Impact | Severity |
|--------|-------------|--------|----------|
| GAP-D1 | Proxy requires manual startup | Reduces transparency | Medium |
| GAP-D2 | MCP requires config file edit | Reduces transparency | Medium |
| GAP-D3 | Watch mode doesn't auto-index | Must re-run `acp index` | Medium |
| GAP-D4 | No unified daemon process | Multiple processes to manage | Low |
| GAP-D5 | Proxy offline = no injection | Degrades gracefully but loses value | Low |

#### Recommendations

| Gap | Recommendation | Effort |
|-----|----------------|--------|
| GAP-D1 | Auto-start daemon on first `acp` command | Medium |
| GAP-D2 | Provide `acp mcp-install` command | Low |
| GAP-D3 | Implement incremental indexing in watch | High |
| GAP-D4 | Unify into single `acpd` process | High |
| GAP-D5 | Hybrid: use cached primer when offline | Medium |

---

### 5. ROI Assessment

#### Complexity vs. Benefit Analysis

```
Benefit
  ▲
  │                                    ┌─────────────────┐
  │                                    │ Unified Daemon  │
  │                            ┌───────┴─────────────────┘
  │                    ┌───────┴─────────┐
  │            ┌───────┴───────┐ Proxy + │
  │    ┌───────┴───────┐       │ Watch   │
  │    │  Static       │ File  │         │
  │    │  Cache Only   │ Sync  │         │
  │    └───────────────┴───────┴─────────┴─────────────────▶ Complexity
```

| Component | Complexity | Benefit | ROI |
|-----------|------------|---------|-----|
| Static cache | Low | Medium | ⭐⭐⭐⭐ Good |
| File sync | Medium | High | ⭐⭐⭐⭐ Good |
| Proxy | Medium | Very High | ⭐⭐⭐⭐⭐ Excellent |
| Watch | Medium | High | ⭐⭐⭐⭐ Good |
| MCP | High | High | ⭐⭐⭐ Moderate |
| Unified daemon | Very High | Very High | ⭐⭐⭐ Moderate |

**Assessment**: The current architecture provides excellent ROI. The unified daemon is aspirational but not essential for core value delivery.

---

## Summary Scorecard

### Goal Alignment Summary

| Goal | Alignment | Score |
|------|-----------|-------|
| G1: Token Efficiency | ✅ Strong | 5/5 |
| G2: Deterministic | ✅ Strong | 5/5 |
| G3: Version Controlled | ✅ Strong | 5/5 |
| G4: Offline Capable | ⚠️ Partial | 3/5 |
| G5: Tool Agnostic | ✅ Strong | 5/5 |
| G6: Safety First | ✅ Strong | 5/5 |
| G7: Transparent | ⚠️ Partial | 3/5 |
| G8: Query Optimized | ✅ Strong | 5/5 |
| **Overall** | **✅ Well Aligned** | **36/40 (90%)** |

### Design Principle Adherence

| Principle | Score |
|-----------|-------|
| P1: Transparency | 3/5 |
| P2: Universality | 4/5 |
| P3: Freshness | 4/5 |
| P4: Efficiency | 5/5 |
| P5: Safety-First | 5/5 |
| **Overall** | **21/25 (84%)** |

### Final Assessment

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     DAEMON VALUE ASSESSMENT                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Overall Alignment:  ████████████████████░░░░  90%  WELL ALIGNED       │
│                                                                         │
│  Principle Adherence: ████████████████░░░░░░░  84%  GOOD               │
│                                                                         │
│  ROI Assessment:      ████████████████████░░░░  HIGH VALUE             │
│                                                                         │
│  Recommendation:      PROCEED with current architecture                 │
│                       Focus on GAP-D1, GAP-D3 for improvements         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Completion Criteria

### Evaluation Complete When:
- [ ] All 8 goals evaluated
- [ ] All 5 design principles assessed
- [ ] Value contribution analyzed
- [ ] Gaps identified
- [ ] ROI assessment complete
- [ ] Summary scorecard generated
- [ ] Recommendations provided
