# RFC-XXXX: [Title]

> **Instructions**: Copy this template to `proposed/NNNN-short-name.md` and fill in all sections.
> Delete this instruction block and any `[bracketed instructions]` before submitting.

- **RFC ID**: XXXX
- **Title**: [Descriptive title]
- **Author**: [Name] <[email]>
- **Status**: Draft
- **Created**: YYYY-MM-DD
- **Updated**: YYYY-MM-DD
- **Discussion**: [Link to GitHub discussion, if any]

---

## Summary

[One paragraph explanation of the proposal. What is it? What does it do? Keep it concise.]

## Motivation

[Why are we doing this? What use cases does it support? What is the expected outcome?]

### Problem Statement

[Describe the problem this RFC solves.]

### Goals

[What are the specific goals of this proposal?]

- Goal 1
- Goal 2

### Non-Goals

[What is explicitly out of scope for this RFC?]

- Non-goal 1
- Non-goal 2

## Detailed Design

[This is the bulk of the RFC. Explain the design in enough detail for somebody familiar with ACP to understand, and for somebody familiar with implementation to implement.]

### Overview

[High-level overview of the solution.]

### Syntax

[If proposing new annotations or syntax, define them precisely.]

```
@acp:new-annotation <value>
@acp:new-annotation-param param1=value1, param2=value2
```

**Grammar (EBNF):**
```ebnf
new_annotation = "@acp:new-annotation" , [ whitespace , value ] ;
value          = quoted_string | identifier ;
```

### Schema Changes

[If proposing changes to JSON schemas, show the exact changes.]

**Cache schema additions:**
```json
{
  "new_field": {
    "type": "string",
    "description": "Description of the new field"
  }
}
```

**Config schema additions:**
```json
{
  "new_option": {
    "type": "boolean",
    "default": false,
    "description": "Description of the new option"
  }
}
```

### Behavior

[Describe the runtime/indexing behavior in detail.]

1. When the indexer encounters `@acp:new-annotation`:
    - Step 1
    - Step 2
2. When an AI tool queries this:
    - Expected behavior

### Error Handling

[How should errors be handled?]

| Error Condition | Permissive Mode | Strict Mode |
|-----------------|-----------------|-------------|
| Invalid value | Warn, use default | Error, abort |
| ... | ... | ... |

### Examples

[Provide concrete examples showing the feature in use.]

**Example 1: Basic usage**
```typescript
/**
 * @acp:new-annotation example-value
 */
function example() {
  // ...
}
```

**Example 2: With parameters**
```typescript
/**
 * @acp:new-annotation-param key=value, other=123
 */
class MyClass {
  // ...
}
```

**Example 3: In cache output**
```json
{
  "files": {
    "src/example.ts": {
      "new_field": "example-value"
    }
  }
}
```

## Drawbacks

[Why should we *not* do this? Be honest about the downsides.]

- **Complexity**: This adds X to the specification...
- **Learning curve**: Users must understand...
- **Implementation cost**: Requires changes to...
- **Potential for misuse**: Could be abused by...

## Alternatives

[What other designs have been considered? What is the impact of not doing this?]

### Alternative A: [Name]

[Description of alternative approach]

**Pros:**
- Pro 1
- Pro 2

**Cons:**
- Con 1
- Con 2

**Why rejected:** [Reason]

### Alternative B: [Name]

[Description]

**Why rejected:** [Reason]

### Do Nothing

[What happens if we don't implement this RFC?]

## Compatibility

### Backward Compatibility

[How does this affect existing ACP users?]

- Existing caches: [Compatible/Incompatible because...]
- Existing configs: [Compatible/Incompatible because...]
- Existing annotations: [Compatible/Incompatible because...]

### Forward Compatibility

[How does this affect future ACP development?]

- Does this close off future possibilities?
- Does this enable future enhancements?

### Migration Path

[If breaking, how do users migrate?]

1. Step 1: ...
2. Step 2: ...
3. Step 3: ...

**Migration tooling:** [Will the CLI provide migration commands?]

## Implementation

### Specification Changes

[List specific changes needed to the spec document.]

- Section X.Y: Add paragraph about...
- Section A.B: Modify to include...
- New section: Create section for...

### Schema Changes

[List changes needed to JSON schemas.]

- `cache.schema.json`: Add `new_field` to FileEntry
- `config.schema.json`: Add `new_option` to root

### CLI Changes

[List changes needed to the reference CLI.]

- `acp index`: Parse new annotation
- `acp query`: Support querying new field
- New command: `acp new-command` (if applicable)

### MCP Server Changes

[List changes needed to the MCP server.]

- New resource: `acp://new-resource`
- New tool: `acp_new_tool`
- Modified tool: `acp_query` to support new type

### Tooling Impact

[How does this affect other tools?]

| Tool | Impact | Required Changes |
|------|--------|------------------|
| VS Code Extension | Low/Medium/High | Description |
| Language Server | Low/Medium/High | Description |
| Third-party tools | Low/Medium/High | Description |

## Rollout Plan

[How should this be rolled out?]

1. **Phase 1**: Implement in CLI behind feature flag
2. **Phase 2**: Update specification
3. **Phase 3**: Update schemas
4. **Phase 4**: Remove feature flag, release

## Open Questions

[List any unresolved questions that need community input.]

1. Should we...?
2. How should we handle...?
3. Is X or Y preferred for...?

## Resolved Questions

[Questions that were raised and resolved during discussion.]

1. **Q**: [Question]
   **A**: [Resolution and reasoning]

## References

[Links to related resources.]

- Related RFC: [RFC-NNNN](./NNNN-name.md)
- GitHub Issue: [#123](https://github.com/acp-protocol/acp-spec/issues/123)
- External resource: [Link](https://example.com)
- Prior art: [Similar feature in X](https://example.com)

---

## Appendix

[Optional: Additional details, extended examples, or supplementary material.]

### A. Extended Examples

[More detailed examples if needed.]

### B. Benchmarks

[Performance considerations if relevant.]

### C. Security Considerations

[Security implications if relevant.]

---

## Changelog

[Track significant changes to this RFC during review.]

| Date | Change |
|------|--------|
| YYYY-MM-DD | Initial draft |
| YYYY-MM-DD | Addressed feedback on X |
| YYYY-MM-DD | Revised syntax based on discussion |

---

<!--
## RFC Process Checklist (for maintainers)

- [ ] RFC number assigned
- [ ] Added to proposed/
- [ ] Discussion link added
- [ ] Initial review complete
- [ ] Community feedback period (2+ weeks)
- [ ] FCP announced
- [ ] FCP complete (10 days)
- [ ] Decision made
- [ ] Moved to accepted/ or rejected/
-->
