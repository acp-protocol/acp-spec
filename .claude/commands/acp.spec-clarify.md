---
description: Resolve open questions and ambiguities from the spec directive audit before remediation. Guides user through decisions required for RFC-001 compliance.
handoffs:
  - label: Back to Audit
    agent: acp.spec-audit-directives
    prompt: Re-run audit to verify no new questions emerged
  - label: Proceed to Remediate
    agent: acp.spec-remediate
    prompt: Apply decisions and remediate the specification
    send: true
  - label: Export Decisions
    agent: acp.spec-export-decisions
    prompt: Export decisions to JSON for record keeping
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--load <path>` to load previous decisions from a JSON file
- `--interactive` to prompt for each decision (default)
- `--defaults` to accept recommended defaults without prompting
- `--question <ID>` to address a specific question
- `--export <path>` to save decisions to JSON

## Purpose

This command facilitates decision-making for open questions identified during the spec audit. These decisions affect how RFC-001 requirements will be implemented in the ACP specification.

**Why this step matters:**
- RFC-001 has several open questions without definitive answers
- The specification must make consistent choices
- Decisions affect backward compatibility and migration paths
- User preferences influence strictness levels

## Outline

1. **Load audit findings** from previous audit or re-run if needed:
   ```bash
   ./scripts/audit-spec-directives.sh --json --questions-only
   ```

   Expected output:
   ```json
   {
     "questions": [
       {
         "id": "Q01",
         "category": "directive-requirement",
         "question": "Should directive suffix be strictly required or just strongly recommended?",
         "options": [
           { "key": "A", "label": "Required in v2.0 with migration period", "default": false },
           { "key": "B", "label": "Always optional with auto-generated defaults", "default": true }
         ],
         "rfc_reference": "RFC-001 Open Question 1",
         "impact": "high",
         "affects": ["Chapter 2", "CLI behavior", "Migration guide"]
       },
       ...
     ],
     "decisions_file": ".acp/spec-decisions.json"
   }
   ```

2. **Present questions** grouped by category:

   **Categories:**
    - `directive-requirement` - How strict should directive rules be?
    - `directive-format` - Syntax and length constraints
    - `backward-compatibility` - How to handle legacy annotations
    - `hierarchy-behavior` - Annotation level precedence
    - `tooling-integration` - CLI and cache behavior

3. **Collect decisions** interactively or from defaults

4. **Validate consistency** - Ensure decisions don't conflict

5. **Save decisions** to `.acp/spec-decisions.json`

## Open Questions from RFC-001

### Q01: Directive Requirement Strictness

**Question**: Should directive suffix be strictly required or just strongly recommended?

| Option | Description | Trade-offs |
|--------|-------------|------------|
| A | Required in v2.0 with migration period | Forces clarity, may break existing projects |
| B | Always optional with auto-generated defaults | Backward compatible, less explicit |

**Recommended**: Option B (optional with defaults)
**Reason**: Maintains backward compatibility while encouraging adoption

---

### Q02: Maximum Directive Length

**Question**: Should there be a maximum directive length?

| Option | Description | Trade-offs |
|--------|-------------|------------|
| A | No limit | Flexibility, potential cache bloat |
| B | Soft limit of 500 chars (warn, don't error) | Balance, enforced via warnings |
| C | Hard limit of 500 chars (error if exceeded) | Strict, may truncate useful context |

**Recommended**: Option B (soft limit with warning)
**Reason**: Prevents abuse while allowing flexibility for edge cases

---

### Q03: Conflicting Directive Resolution

**Question**: How should conflicting directives be handled?

| Option | Description | Trade-offs |
|--------|-------------|------------|
| A | File-level always wins | Predictable, may lose granularity |
| B | Most restrictive wins | Safety-first, may be overly cautious |
| C | Most specific scope wins (inline > symbol > file) | Intuitive, complex resolution |

**Recommended**: Option C (most specific wins)
**Reason**: Follows principle of least surprise; inline annotations are deliberate

---

### Q04: Auto-Generated Directive Format

**Question**: When auto-generating directives for legacy annotations, what format?

| Option | Description | Example |
|--------|-------------|---------|
| A | Generic directive per type | `@acp:lock frozen` → "Locked at frozen level" |
| B | Full recommended directive | `@acp:lock frozen` → "MUST NOT modify this file..." |
| C | Empty directive (placeholder) | `@acp:lock frozen` → "" |

**Recommended**: Option B (full recommended directive)
**Reason**: Provides maximum clarity for AI agents

---

### Q05: Directive Localization

**Question**: Should directives support localization markers?

| Option | Description | Example |
|--------|-------------|---------|
| A | No localization | English only |
| B | Optional language tag | `[en]`, `[es]`, etc. |
| C | Full i18n with resource bundles | External locale files |

**Recommended**: Option A (no localization)
**Reason**: Adds complexity for unclear benefit; most AI models understand English

---

### Q06: Symbol-Level Annotation Scope

**Question**: Should symbol-level annotations apply to nested symbols?

| Option | Description | Example |
|--------|-------------|---------|
| A | Symbol only (no inheritance) | Class lock doesn't lock methods |
| B | Symbol and direct children | Class lock applies to direct methods |
| C | Symbol and all descendants | Class lock applies to everything inside |

**Recommended**: Option A (no inheritance)
**Reason**: Explicit is better than implicit; prevents surprises

---

### Q07: CLI Warning Behavior

**Question**: How should CLI handle missing directives in non-strict mode?

| Option | Description | Behavior |
|--------|-------------|----------|
| A | Silent | Index completes without mention |
| B | Summary warning | "47 annotations missing directives" |
| C | Per-annotation warning | Warning for each missing directive |
| D | First-N then summary | First 5 warnings, then count |

**Recommended**: Option D (first-N then summary)
**Reason**: Balance between visibility and noise

---

### Q08: Cache Storage of Generated Directives

**Question**: Should auto-generated directives be stored in cache?

| Option | Description | Trade-offs |
|--------|-------------|------------|
| A | Store both original and generated | Transparency, larger cache |
| B | Store only generated (if no original) | Smaller cache, loses "auto" marker |
| C | Mark as auto-generated with flag | Best of both, slightly complex |

**Recommended**: Option C (mark with flag)
**Reason**: Tools can distinguish explicit vs generated directives

---

## Decision Validation

After collecting decisions, validate for consistency:

| Conflict | Resolution |
|----------|------------|
| Q01=A (required) + Q04=C (empty placeholder) | Invalid - empty can't satisfy requirement |
| Q03=A (file wins) + Q06=C (full inheritance) | Potentially confusing - warn user |
| Q02=C (hard limit) + long recommended directives | May need to shorten standard directives |

## Output Format

Save decisions to `.acp/spec-decisions.json`:

```json
{
  "$schema": "https://acp-protocol.dev/schemas/v1/spec-decisions.schema.json",
  "version": "1.0.0",
  "created": "[TIMESTAMP]",
  "rfc_reference": "RFC-001",
  "decisions": {
    "Q01": {
      "choice": "B",
      "label": "Always optional with auto-generated defaults",
      "rationale": "User prioritized backward compatibility",
      "decided_at": "[TIMESTAMP]"
    },
    ...
  },
  "conflicts_resolved": [],
  "notes": "[User notes if any]"
}
```

## Interactive Mode

When running interactively, present each question as:

```
┌─────────────────────────────────────────────────────────────────────┐
│ Q01: Directive Requirement Strictness                               │
├─────────────────────────────────────────────────────────────────────┤
│ Should directive suffix be strictly required or strongly recommended?│
│                                                                     │
│ [A] Required in v2.0 with migration period                          │
│     → Forces clarity, may break existing projects                   │
│                                                                     │
│ [B] Always optional with auto-generated defaults  ★ RECOMMENDED     │
│     → Backward compatible, less explicit                            │
│                                                                     │
│ Affects: Chapter 2, CLI behavior, Migration guide                   │
│ Impact: HIGH                                                        │
└─────────────────────────────────────────────────────────────────────┘

Your choice [A/B] (default: B): _
```

## Completion Criteria

### Clarification Complete When:
- [ ] All open questions presented
- [ ] All decisions collected (or defaults accepted)
- [ ] No conflicting decisions remain
- [ ] Decisions validated for consistency
- [ ] Decisions saved to JSON file
- [ ] Summary displayed to user

## Next Steps

After decisions are captured:

1. **Review summary** of all decisions
2. **Export decisions** if needed for documentation
3. **Hand off to remediate** with decisions file

The remediate command will use these decisions to update the specification consistently.