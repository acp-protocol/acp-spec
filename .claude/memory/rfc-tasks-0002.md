# RFC-0002 Task List: Documentation References and Style Guides

**Created**: 2025-12-22
**Plan**: `.claude/memory/rfc-plan-0002.md`

---

## Task Summary

| Task ID | Phase | Description | Depends On | Est. Time | Status |
|---------|-------|-------------|------------|-----------|--------|
| T1.1 | 1 | Update config.schema.json | - | 1h | ✅ Complete |
| T1.2 | 1 | Update cache.schema.json | T1.1 | 1.5h | ✅ Complete |
| T2.1 | 2 | Update ACP-1.0.md Appendix A | T1.1, T1.2 | 45m | ✅ Complete |
| T2.2 | 2 | Ch05: Documentation References | T2.1 | 1.5h | ✅ Complete |
| T2.3 | 2 | Ch05: Style Guides | T2.2 | 1h | ✅ Complete |
| T2.4 | 2 | Ch04: Config Format | T1.1 | 1h | ✅ Complete |
| T2.5 | 2 | Ch03: Cache Format | T1.2 | 45m | ✅ Complete |
| T2.6 | 2 | Ch06: Constraints | T2.3 | 30m | ✅ Complete |
| T2.7 | 2 | Ch11: AI Behavior | T2.2, T2.3 | 1h | ✅ Complete |
| T3.1 | 3 | Update RFC open questions | - | 20m | ✅ Complete |
| T3.2 | 3 | Move RFC to accepted/ | T3.1 | 5m | ✅ Complete (prior) |
| T3.3 | 3 | Update CHANGELOG | T3.2 | 15m | ✅ Complete |
| T4.1 | 4 | Validate schemas | T1.1, T1.2 | 30m | ✅ Complete |
| T4.2 | 4 | Cross-reference verification | T2.7 | 30m | ✅ Complete |

**Implementation Completed**: 2025-12-22

---

## Phase 1: Schema Updates

### T1.1: Update config.schema.json

**Phase**: 1
**Component**: Schemas
**Files**: `schemas/v1/config.schema.json`
**Depends On**: None
**Estimated Time**: 1 hour

**Description**:
Add the `documentation` configuration section to the config schema. This enables project-level management of approved documentation sources and custom style guides.

**Changes Required**:

1. Add `documentation` property to root properties object
2. Add `$defs/approved_source` definition:
   ```json
   {
     "type": "object",
     "required": ["id", "url"],
     "properties": {
       "id": { "type": "string", "pattern": "^[a-z][a-z0-9-]*$" },
       "url": { "type": "string", "format": "uri" },
       "version": { "type": "string" },
       "description": { "type": "string" },
       "sections": { "type": "object", "additionalProperties": { "type": "string" } },
       "fetchable": { "type": "boolean", "default": true },
       "lastVerified": { "type": "string", "format": "date-time" }
     }
   }
   ```

3. Add `$defs/style_guide_definition` definition:
   ```json
   {
     "type": "object",
     "properties": {
       "extends": { "type": "string" },
       "source": { "type": "string" },
       "url": { "type": "string", "format": "uri" },
       "description": { "type": "string" },
       "languages": { "type": "array", "items": { "type": "string" } },
       "rules": { "type": "array", "items": { "type": "string" } },
       "filePatterns": { "type": "array", "items": { "type": "string" } }
     }
   }
   ```

4. Add documentation object:
   ```json
   {
     "documentation": {
       "type": "object",
       "properties": {
         "approvedSources": { "type": "array", "items": { "$ref": "#/$defs/approved_source" } },
         "styleGuides": { "type": "object", "additionalProperties": { "$ref": "#/$defs/style_guide_definition" } },
         "defaults": {
           "type": "object",
           "properties": {
             "fetchRefs": { "type": "boolean", "default": false },
             "style": { "type": "string" }
           }
         },
         "validation": {
           "type": "object",
           "properties": {
             "requireApprovedSources": { "type": "boolean", "default": false },
             "warnUnknownStyle": { "type": "boolean", "default": true }
           }
         }
       }
     }
   }
   ```

**Acceptance Criteria**:
- [ ] `documentation` property added to root
- [ ] `$defs/approved_source` definition complete
- [ ] `$defs/style_guide_definition` definition complete
- [ ] All properties have descriptions
- [ ] Schema validates against JSON Schema Draft-07
- [ ] Existing configs remain valid (backward compat)

---

### T1.2: Update cache.schema.json

**Phase**: 1
**Component**: Schemas
**Files**: `schemas/v1/cache.schema.json`
**Depends On**: T1.1
**Estimated Time**: 1.5 hours

**Description**:
Extend the cache schema to store resolved documentation references and enhanced style information per file, plus a top-level documentation index.

**Changes Required**:

1. Add `$defs/ref_entry`:
   ```json
   {
     "type": "object",
     "required": ["url"],
     "properties": {
       "url": { "type": "string", "format": "uri" },
       "sourceId": { "type": ["string", "null"] },
       "version": { "type": ["string", "null"] },
       "section": { "type": ["string", "null"] },
       "fetch": { "type": "boolean", "default": false },
       "scope": { "type": "string", "enum": ["file", "symbol"], "default": "file" },
       "symbolName": { "type": ["string", "null"] }
     }
   }
   ```

2. Add `$defs/style_entry`:
   ```json
   {
     "type": "object",
     "properties": {
       "guide": { "type": "string" },
       "extends": { "type": ["string", "null"] },
       "rules": { "type": "array", "items": { "type": "string" }, "default": [] },
       "source": { "type": ["string", "null"] },
       "sourceUrl": { "type": ["string", "null"], "format": "uri" }
     }
   }
   ```

3. Add to `$defs/file_entry`:
   ```json
   {
     "refs": {
       "type": "array",
       "items": { "$ref": "#/$defs/ref_entry" },
       "default": [],
       "description": "Documentation references for this file"
     },
     "style": {
       "$ref": "#/$defs/style_entry",
       "description": "Style guide configuration for this file"
     }
   }
   ```

4. Add top-level `documentation` property:
   ```json
   {
     "documentation": {
       "type": "object",
       "properties": {
         "sources": {
           "type": "object",
           "additionalProperties": {
             "type": "object",
             "properties": {
               "url": { "type": "string" },
               "version": { "type": ["string", "null"] },
               "fileCount": { "type": "integer" },
               "files": { "type": "array", "items": { "type": "string" } }
             }
           }
         },
         "styles": {
           "type": "object",
           "additionalProperties": {
             "type": "object",
             "properties": {
               "fileCount": { "type": "integer" },
               "files": { "type": "array", "items": { "type": "string" } },
               "source": { "type": ["string", "null"] }
             }
           }
         },
         "unresolvedRefs": {
           "type": "array",
           "items": {
             "type": "object",
             "properties": {
               "file": { "type": "string" },
               "ref": { "type": "string" },
               "reason": { "type": "string" }
             }
           }
         }
       }
     }
   }
   ```

**Acceptance Criteria**:
- [ ] `$defs/ref_entry` definition added
- [ ] `$defs/style_entry` definition added
- [ ] `refs` array added to `file_entry`
- [ ] `style` property type changed from string to `$ref: style_entry`
- [ ] Top-level `documentation` index added
- [ ] Existing caches with string `style` remain valid
- [ ] Schema validates

---

## Phase 2: Specification Updates

### T2.1: Update ACP-1.0.md Appendix A

**Phase**: 2
**Component**: Specification
**Files**: `spec/ACP-1.0.md`
**Depends On**: T1.1, T1.2
**Estimated Time**: 45 minutes

**Description**:
Add new annotation types to the reserved annotations table in Appendix A.

**Changes Required**:

Find the reserved annotations table and add:

| Annotation | Parameters | Example | Description |
|------------|------------|---------|-------------|
| `@acp:ref-version` | `<version>` | `@acp:ref-version "4.0"` | Pin reference to specific version |
| `@acp:ref-section` | `<path>` | `@acp:ref-section "utility-classes"` | Reference specific section |
| `@acp:ref-fetch` | `<boolean>` | `@acp:ref-fetch true` | Hint for AI to fetch content |
| `@acp:style-extends` | `<guide>` | `@acp:style-extends prettier` | Extend another style guide |

Update existing:
- `@acp:ref` - note support for source IDs from config
- `@acp:style` - note link to config styleGuides

**Acceptance Criteria**:
- [ ] Four new annotations added to table
- [ ] Existing annotations updated with notes
- [ ] Examples provided for each
- [ ] Cross-references to Chapter 05 added

---

### T2.2: Ch05 - Documentation References Section

**Phase**: 2
**Component**: Specification
**Files**: `spec/chapters/05-annotations.md`
**Depends On**: T2.1
**Estimated Time**: 1.5 hours

**Description**:
Add comprehensive documentation references section covering all @acp:ref-* annotations.

**Content to Add**:

```markdown
## X. Documentation References (RFC-0002)

### X.1 Overview

Documentation references link code to authoritative external sources...

### X.2 @acp:ref

**Syntax**: `@acp:ref <url|source-id> - <directive>`

References documentation by URL or configured source ID...

**Grammar (EBNF)**:
```ebnf
ref_annotation = "@acp:ref" , whitespace , ref_value , [ whitespace , "-" , whitespace , directive ] ;
ref_value = quoted_url | source_id ;
```

### X.3 @acp:ref-version

**Syntax**: `@acp:ref-version "<version>"`

Pins reference to specific documentation version...

### X.4 @acp:ref-section

**Syntax**: `@acp:ref-section "<section-path>"`

References specific section within documentation...

### X.5 @acp:ref-fetch

**Syntax**: `@acp:ref-fetch <true|false>`

Hints whether AI tools should fetch this reference...

### X.6 Resolution Order

1. Check `documentation.approvedSources` for matching ID
2. If found, use source URL + version
3. If `@acp:ref-section` specified, append section path
4. If not found and validation strict, error
5. Otherwise treat as literal URL

### X.7 Examples

[Include examples from RFC showing config + annotations]
```

**Acceptance Criteria**:
- [ ] Section added with correct numbering
- [ ] All four ref-related annotations documented
- [ ] EBNF grammar provided
- [ ] Resolution order explained
- [ ] Examples with approved sources
- [ ] Examples with direct URLs

---

### T2.3: Ch05 - Style Guides Section

**Phase**: 2
**Component**: Specification
**Files**: `spec/chapters/05-annotations.md`
**Depends On**: T2.2
**Estimated Time**: 1 hour

**Description**:
Update/extend the style guides section with @acp:style-extends and built-in registry.

**Content to Add/Update**:

```markdown
## Y. Style Guides (RFC-0002)

### Y.1 @acp:style-extends

**Syntax**: `@acp:style-extends <guide-name>`

Extends another style guide, inheriting its rules...

### Y.2 Built-in Style Guides

The following style guide names are recognized by default:

| Guide Name | Language | Documentation URL |
|------------|----------|-------------------|
| `google-typescript` | TypeScript | https://google.github.io/styleguide/tsguide.html |
| `google-javascript` | JavaScript | https://google.github.io/styleguide/jsguide.html |
| `google-python` | Python | https://google.github.io/styleguide/pyguide.html |
| `airbnb-javascript` | JavaScript | https://github.com/airbnb/javascript |
| `airbnb-react` | React | https://github.com/airbnb/javascript/tree/master/react |
| `pep8` | Python | https://peps.python.org/pep-0008/ |
| `black` | Python | https://black.readthedocs.io/ |
| `prettier` | Multi | https://prettier.io/docs/en/options.html |
| `rustfmt` | Rust | https://rust-lang.github.io/rustfmt/ |
| `standardjs` | JavaScript | https://standardjs.com/rules.html |
| `tailwindcss-v3` | CSS | https://v2.tailwindcss.com/docs |
| `tailwindcss-v4` | CSS | https://tailwindcss.com/docs |

### Y.3 Custom Style Guides

Define custom guides in `.acp.config.json`:
[Config example]

### Y.4 Style Inheritance

Rule precedence (highest to lowest):
1. Symbol-level `@acp:style-rules`
2. File-level `@acp:style-rules`
3. File-level `@acp:style`
4. Config `documentation.defaults.style`
```

**Acceptance Criteria**:
- [ ] `@acp:style-extends` documented
- [ ] Built-in style guide table complete
- [ ] Custom guide configuration explained
- [ ] Inheritance/precedence rules documented
- [ ] Examples provided

---

### T2.4: Ch04 - Config Format

**Phase**: 2
**Component**: Specification
**Files**: `spec/chapters/04-config-format.md`
**Depends On**: T1.1
**Estimated Time**: 1 hour

**Description**:
Add documentation configuration section to the config format chapter.

**Content to Add**:

```markdown
## X. Documentation Configuration (RFC-0002)

### X.1 Overview

The `documentation` section configures project-level documentation sources and style guides.

### X.2 Approved Sources

```json
{
  "documentation": {
    "approvedSources": [
      {
        "id": "react-docs",
        "url": "https://react.dev/reference/react",
        "version": "18.x",
        "description": "React official documentation",
        "fetchable": true
      }
    ]
  }
}
```

[Properties table]

### X.3 Style Guides

```json
{
  "documentation": {
    "styleGuides": {
      "company-react": {
        "extends": "airbnb-react",
        "rules": ["prefer-function-components", "use-strict-types"]
      }
    }
  }
}
```

### X.4 Defaults

### X.5 Validation
```

**Acceptance Criteria**:
- [ ] New section for documentation config
- [ ] All properties documented with types
- [ ] JSON examples provided
- [ ] Cross-references to Chapter 05
- [ ] Cross-references to schemas

---

### T2.5: Ch03 - Cache Format

**Phase**: 2
**Component**: Specification
**Files**: `spec/chapters/03-cache-format.md`
**Depends On**: T1.2
**Estimated Time**: 45 minutes

**Description**:
Document new cache fields for refs, style, and documentation index.

**Content to Add**:

```markdown
### X.Y File Entry: Documentation References (RFC-0002)

#### refs

Array of resolved documentation references:

```json
{
  "refs": [
    {
      "url": "https://tailwindcss.com/docs/v4",
      "sourceId": "tailwindcss-v4",
      "version": "4.0",
      "fetch": true,
      "scope": "file"
    }
  ]
}
```

#### style

Enhanced style configuration:

```json
{
  "style": {
    "guide": "tailwindcss-v4",
    "extends": "prettier",
    "rules": ["utility-first", "no-custom-css"],
    "source": "tailwindcss-v4",
    "sourceUrl": "https://tailwindcss.com/docs/v4"
  }
}
```

### X.Z Documentation Index (RFC-0002)

Top-level aggregation of documentation usage:

```json
{
  "documentation": {
    "sources": { ... },
    "styles": { ... },
    "unresolvedRefs": [ ... ]
  }
}
```
```

**Acceptance Criteria**:
- [ ] `refs` array documented
- [ ] `style` object structure documented
- [ ] `documentation` index documented
- [ ] JSON examples provided
- [ ] Links to schema definitions

---

### T2.6: Ch06 - Constraints

**Phase**: 2
**Component**: Specification
**Files**: `spec/chapters/06-constraints.md`
**Depends On**: T2.3
**Estimated Time**: 30 minutes

**Description**:
Update style constraint section with references to RFC-0002 additions.

**Changes Required**:

- Update style constraint section to reference style guide definitions
- Add note about inheritance from config defaults
- Document override precedence (symbol > file > config)
- Cross-reference Chapter 05 style guides section

**Acceptance Criteria**:
- [ ] Style constraint section updated
- [ ] Precedence rules documented
- [ ] Cross-reference to Chapter 05 added
- [ ] Config defaults mentioned

---

### T2.7: Ch11 - AI Behavior

**Phase**: 2
**Component**: Specification
**Files**: `spec/chapters/11-tool-integration.md`
**Depends On**: T2.2, T2.3
**Estimated Time**: 1 hour

**Description**:
Add AI behavior specification for handling documentation references.

**Content to Add**:

```markdown
## X. Documentation Reference Handling (RFC-0002)

### X.1 When to Consult References

AI tools SHOULD consult documentation references when:

| Scenario | Action |
|----------|--------|
| `@acp:ref-fetch true` is set | SHOULD fetch and read documentation |
| Making changes to file with `@acp:ref` | MAY fetch for context |
| User asks about conventions | SHOULD cite referenced documentation |
| Uncertainty about framework usage | SHOULD check referenced docs |
| `@acp:style` with linked source | SHOULD follow style guide |

### X.2 When NOT to Fetch

AI tools SHOULD NOT:
- Fetch documentation on every request (performance)
- Ignore `@acp:ref-fetch false` directives
- Assume documentation content without fetching
- Override style rules without user request

### X.3 Rate Limiting Guidance

Tools SHOULD implement reasonable rate limiting when fetching documentation.
Recommended: Cache fetched documentation for the duration of the session.

### X.4 Style Application Behavior

| Setting | AI Behavior |
|---------|-------------|
| `@acp:style google-typescript` | Follow Google TS conventions for new code |
| `@acp:style-rules max-line-length=100` | Apply specific rule |
| No style specified | Follow surrounding code patterns |
| Conflicting styles | Symbol-level takes precedence |
```

**Acceptance Criteria**:
- [ ] "Documentation References" section added
- [ ] Consultation guidelines table
- [ ] "When NOT to fetch" section
- [ ] Rate limiting SHOULD-level guidance
- [ ] Style application table
- [ ] SHOULD/MAY/MUST levels correctly applied

---

## Phase 3: RFC Finalization

### T3.1: Update RFC Open Questions

**Phase**: 3
**Component**: RFC
**Files**: `rfcs/proposed/rfc-0002-documentation-references-and-style-guides.md`
**Depends On**: None
**Estimated Time**: 20 minutes

**Description**:
Resolve remaining open questions and update RFC metadata.

**Changes Required**:

1. Update Q3 (Non-HTTP schemes):
   ```markdown
   3. **Should we support non-HTTP sources (e.g., file://, man://)?**
      - **RESOLVED**: HTTP(S) only for v1. Extensible scheme support deferred to future RFC.
   ```

2. Update Q4 (Rate limiting):
   ```markdown
   4. **Rate limiting for documentation fetches?**
      - **RESOLVED**: Left to implementations. Spec includes SHOULD-level guidance in Chapter 11.
   ```

3. Add prerequisite note:
   ```markdown
   > **Note**: This RFC builds upon RFC-0001 (Self-Documenting Annotations).
   ```

4. Update status:
   ```markdown
   - **Status**: Accepted
   - **Updated**: 2025-12-22
   ```

**Acceptance Criteria**:
- [ ] Open questions resolved
- [ ] RFC-0001 reference added
- [ ] Status changed to "Accepted"
- [ ] Updated date set

---

### T3.2: Move RFC to accepted/

**Phase**: 3
**Component**: RFC
**Files**: Move RFC file
**Depends On**: T3.1
**Estimated Time**: 5 minutes

**Command**:
```bash
mv rfcs/proposed/rfc-0002-documentation-references-and-style-guides.md \
   rfcs/accepted/rfc-0002-documentation-references-and-style-guides.md
```

**Acceptance Criteria**:
- [ ] RFC in accepted/ directory
- [ ] No broken links

---

### T3.3: Update CHANGELOG

**Phase**: 3
**Component**: Documentation
**Files**: `CHANGELOG.md`
**Depends On**: T3.2
**Estimated Time**: 15 minutes

**Content to Add**:

```markdown
## [1.X.0] - 2025-12-22

### Added

- **RFC-0002: Documentation References and Style Guides**
  - Project-level `documentation.approvedSources` configuration
  - Custom style guide definitions in `documentation.styleGuides`
  - New annotations: `@acp:ref-version`, `@acp:ref-section`, `@acp:ref-fetch`, `@acp:style-extends`
  - Enhanced cache format with `refs[]` and `style` object per file
  - Top-level `documentation` index in cache
  - Built-in style guide registry with URLs
  - AI behavior specification for documentation handling
```

**Acceptance Criteria**:
- [ ] New version section added
- [ ] All schema changes listed
- [ ] All spec changes listed
- [ ] RFC link included

---

## Phase 4: Validation

### T4.1: Validate Schemas

**Phase**: 4
**Component**: Validation
**Files**: `schemas/v1/*.schema.json`
**Depends On**: T1.1, T1.2
**Estimated Time**: 30 minutes

**Validation Steps**:

1. Validate schemas are valid JSON Schema Draft-07
2. Check all $refs resolve correctly
3. Test with example fixtures
4. Verify backward compatibility

**Commands**:
```bash
# Validate schema structure
npx ajv validate -s schemas/v1/config.schema.json
npx ajv validate -s schemas/v1/cache.schema.json

# Test with fixtures (if available)
```

**Acceptance Criteria**:
- [ ] config.schema.json is valid Draft-07
- [ ] cache.schema.json is valid Draft-07
- [ ] All $refs resolve
- [ ] Example JSON validates
- [ ] Existing fixtures still pass

---

### T4.2: Cross-Reference Verification

**Phase**: 4
**Component**: Validation
**Files**: All spec files
**Depends On**: T2.7
**Estimated Time**: 30 minutes

**Verification Steps**:

1. Check all chapter cross-references
2. Verify schema references match actual schemas
3. Confirm RFC references are accurate
4. Test internal links

**Acceptance Criteria**:
- [ ] All [Chapter X] links work
- [ ] Schema references accurate
- [ ] RFC-0001 and RFC-0002 references correct
- [ ] No orphaned sections
- [ ] Table of contents updated if needed

---

## Execution Notes

### Parallel Execution Opportunities

- T2.4 (Ch04) can run parallel with T2.2, T2.3 after T1.1
- T2.5 (Ch03) can run parallel with T2.2, T2.3 after T1.2
- T3.1 (RFC update) can run parallel with Phase 2
- T4.1 (schema validation) can run after T1.2 in parallel with Phase 2

### Critical Path

```
T1.1 → T1.2 → T2.1 → T2.2 → T2.3 → T2.7 → T4.2
```

### Rollback Plan

If issues found during validation:
1. Schema changes are additive - no rollback needed for existing data
2. Spec changes can be reverted via git
3. RFC can remain in proposed/ if implementation incomplete

---

*Task list created: 2025-12-22*
