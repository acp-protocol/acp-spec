# RFC-XXXX: [Title]

- **RFC ID**: XXXX
- **Title**: [Descriptive title]
- **Author**: [Name] <[email]>
- **Status**: Draft | Proposed | Accepted | Rejected | Withdrawn
- **Created**: YYYY-MM-DD
- **Updated**: YYYY-MM-DD

## Summary

One paragraph explanation of the proposal.

## Motivation

Why are we doing this? What use cases does it support? What is the expected outcome?

## Detailed Design

This is the bulk of the RFC. Explain the design in enough detail for somebody familiar with ACP to understand, and for somebody familiar with implementation to implement.

### Syntax

If proposing new annotations or syntax:

```
@acp:new-annotation <value>
@acp:new-annotation-param param1=value1, param2=value2
```

### Schema Changes

If proposing changes to JSON schemas:

```json
{
  "new_field": {
    "type": "string",
    "description": "Description of the new field"
  }
}
```

### Behavior

Describe the runtime/indexing behavior in detail.

### Examples

Provide concrete examples showing the feature in use:

```typescript
/**
 * @acp:new-annotation example-value
 */
function example() {
  // ...
}
```

## Drawbacks

Why should we *not* do this?

## Alternatives

What other designs have been considered? What is the impact of not doing this?

## Compatibility

### Backward Compatibility

How does this affect existing ACP users?

### Forward Compatibility

How does this affect future ACP development?

### Migration Path

If breaking, how do users migrate?

## Implementation

### Specification Changes

List changes needed to the spec document.

### Schema Changes

List changes needed to JSON schemas.

### CLI Changes

List changes needed to the reference CLI.

### Tooling Impact

How does this affect:
- MCP Server
- VS Code Extension
- Language Server
- Other tools

## Open Questions

List any unresolved questions.

## References

- Link to related RFCs
- Link to relevant issues
- Link to external resources

---

## RFC Process

### Stages

1. **Draft**: Initial idea, seeking feedback
2. **Proposed**: Formally submitted for review
3. **Final Comment Period (FCP)**: 10 days for final feedback
4. **Accepted**: Approved for implementation
5. **Rejected**: Not accepted (with reasoning)
6. **Withdrawn**: Author withdrew the proposal

### Review Criteria

- Does it align with ACP's goals?
- Is it well-specified?
- Is it implementable?
- Does it maintain backward compatibility (or justify breaking)?
- Has community feedback been addressed?

### Acceptance

RFCs are accepted by maintainer consensus after FCP with no blocking objections.
