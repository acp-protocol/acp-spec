# Validate ACP Schemas for Schema Store Submission

Perform a comprehensive sequential analysis of the ACP JSON schemas (`attempts.schema.json`, `primer.schema.json`, `sync.schema.json`) to prepare them for Schema Store submission.

## Objective

Validate, analyze, and remediate the three new ACP schemas before submitting to the JSON Schema Store. This requires meticulous attention to schema correctness, cross-schema consistency, edge case handling, and Schema Store compliance.

---

## Phase 1: Individual Schema Structural Validation

For each schema (`attempts.schema.json`, `primer.schema.json`, `sync.schema.json`), perform:

### 1.1 JSON Schema Draft Compliance
- [ ] Verify `$schema` is set to `https://json-schema.org/draft-07/schema#`
- [ ] Verify `$id` follows pattern `https://acp-protocol.dev/schemas/v1/{name}.schema.json`
- [ ] Verify `title` and `description` are present and descriptive
- [ ] Check all `$ref` references resolve correctly within the schema

### 1.2 Required Fields Analysis
- [ ] Identify all `required` arrays and verify referenced properties exist
- [ ] Check for orphan required fields (required but not defined in properties)
- [ ] Verify `required` arrays don't contain duplicates

### 1.3 Type Consistency
- [ ] All properties have explicit `type` declarations (or use `$ref`, `oneOf`, `anyOf`, `allOf`)
- [ ] No implicit types that could cause validation ambiguity
- [ ] Arrays have `items` defined
- [ ] Objects have `properties` or `additionalProperties` defined

### 1.4 Default Values
- [ ] All `default` values match their declared type
- [ ] Default values are valid according to any `enum`, `pattern`, `minimum`, `maximum` constraints
- [ ] Consider if defaults make sense for the use case

### 1.5 Pattern and Format Validation
- [ ] All `pattern` regexes are valid and test correctly
- [ ] All `format` values are standard JSON Schema formats (date-time, uri, email, etc.)
- [ ] Version patterns match semver: `^\\d+\\.\\d+\\.\\d+`

---

## Phase 2: Cross-Schema Consistency Analysis

### 2.1 Shared Definitions Alignment
Check consistency across schemas for:
- [ ] Version field patterns (should be identical)
- [ ] Timestamp formats (should all use `date-time`)
- [ ] ID field patterns and naming conventions
- [ ] File path conventions

### 2.2 Enumeration Consistency
- [ ] `attempt_status` values in attempts.schema.json
- [ ] `capabilities` values referenced in primer.schema.json and sync.schema.json
- [ ] `toolName` values in sync.schema.json
- [ ] Lock levels referenced across schemas match constraints.schema.json

### 2.3 Naming Convention Consistency
- [ ] Property naming style (camelCase vs snake_case)
- [ ] Definition naming style (`$defs` keys)
- [ ] Consistent terminology (e.g., "budget" vs "tokenBudget")

### 2.4 Cross-References
- [ ] primer.schema.json sections reference cache/vars data sources correctly
- [ ] sync.schema.json primer config aligns with primer.schema.json structure
- [ ] attempts.schema.json status values align with any references in other schemas

---

## Phase 3: Edge Case Analysis

### 3.1 Boundary Conditions
For each schema, identify and verify handling of:
- [ ] Empty arrays: `[]`
- [ ] Empty objects: `{}`
- [ ] Empty strings: `""`
- [ ] Null values (if allowed)
- [ ] Maximum/minimum values at boundaries
- [ ] Very long strings
- [ ] Unicode characters in strings

### 3.2 attempts.schema.json Specific
- [ ] What happens with 0 attempts?
- [ ] What happens with 0 checkpoints?
- [ ] Maximum number of history entries?
- [ ] File content storage with empty files
- [ ] Hash collision handling (documented?)
- [ ] Circular checkpoint references possible?

### 3.3 primer.schema.json Specific
- [ ] Section with 0 tokens valid?
- [ ] Section with tokens: "dynamic" but no data source?
- [ ] Empty `modifiers` array behavior
- [ ] Circular `dependsOn` references possible?
- [ ] `conflictsWith` self-reference?
- [ ] Score calculation with all weights = 0?
- [ ] Empty `formats` object valid?
- [ ] Category with no sections?

### 3.4 sync.schema.json Specific
- [ ] Empty `tools` array behavior (auto-detect vs none?)
- [ ] Tool in both `tools` and `exclude`?
- [ ] Custom adapter with same name as built-in?
- [ ] `primer.budget` below minimum required sections?
- [ ] `mergeStrategy: "section"` without `sectionMarker`?
- [ ] Conflicting tool configurations?

---

## Phase 4: Error Message Quality

### 4.1 Validation Error Context
- [ ] Each property has a `description` for clear error messages
- [ ] Enums have meaningful value names
- [ ] Pattern constraints have `title` or `description` explaining expected format
- [ ] Complex `allOf`/`anyOf`/`oneOf` have clear descriptions

### 4.2 Error Recovery Guidance
Consider documenting in descriptions:
- [ ] What to do when validation fails
- [ ] Common mistakes and how to fix them
- [ ] Links to documentation

---

## Phase 5: Schema Store Requirements

### 5.1 Schema Store Catalog Entry
Verify each schema is ready for catalog.json entry:
```json
{
  "name": "ACP [Type] Configuration",
  "description": "[Clear description]",
  "fileMatch": ["[patterns]"],
  "url": "https://acp-protocol.dev/schemas/v1/[name].schema.json"
}
```

### 5.2 File Match Patterns
Verify appropriate file patterns:
- [ ] `attempts.schema.json`: `["acp.attempts.json", ".acp/acp.attempts.json"]`
- [ ] `primer.schema.json`: `["primer.json", "primer.defaults.json", ".acp/primer.json"]`
- [ ] `sync.schema.json`: `["acp.sync.json", ".acp/acp.sync.json"]`

### 5.3 URL Accessibility
- [ ] Schema URLs will be accessible at acp-protocol.dev
- [ ] `$id` matches planned hosting URL
- [ ] No localhost or development URLs

### 5.4 Schema Store Validation
- [ ] Schemas pass `ajv compile` without errors
- [ ] Schemas pass Schema Store's own validation
- [ ] No deprecated JSON Schema keywords used
- [ ] File size reasonable (<100KB each)

---

## Phase 6: Test Fixtures

### 6.1 Valid Examples
For each schema, verify `examples` array contains:
- [ ] Minimal valid document
- [ ] Full-featured document
- [ ] Examples covering major use cases

### 6.2 Invalid Examples (for testing)
Document expected rejections:
- [ ] Missing required fields
- [ ] Invalid enum values
- [ ] Type mismatches
- [ ] Pattern violations
- [ ] Constraint violations (min/max)

---

## Phase 7: Remediation

For each issue found:

### 7.1 Issue Documentation
```markdown
### Issue: [Brief title]
- **Schema**: [which schema]
- **Location**: [JSON path]
- **Severity**: Critical/High/Medium/Low
- **Description**: [What's wrong]
- **Impact**: [What breaks]
- **Fix**: [How to fix]
```

### 7.2 Apply Fixes
- [ ] Make minimal, targeted changes
- [ ] Document each change
- [ ] Re-validate after changes
- [ ] Update examples if affected

### 7.3 Verification
- [ ] Run `ajv validate` on all schemas
- [ ] Run `ajv validate` on all examples against their schemas
- [ ] Verify cross-schema references still work
- [ ] Test with real-world sample data

---

## Phase 8: Final Pre-Submission Checklist

### 8.1 Schema Quality
- [ ] All schemas pass JSON Schema meta-validation
- [ ] All examples validate against their schemas
- [ ] No validation warnings
- [ ] Consistent formatting (2-space indent, sorted keys optional)

### 8.2 Documentation
- [ ] All properties have descriptions
- [ ] Examples are representative
- [ ] Schema Store entry is prepared

### 8.3 Testing
- [ ] Positive test cases pass
- [ ] Negative test cases fail as expected
- [ ] Edge cases handled appropriately

### 8.4 Submission Ready
- [ ] Schemas are at stable URLs
- [ ] PR to SchemaStore/schemastore prepared
- [ ] Catalog entry JSON ready

---

## Execution Instructions

Run this analysis sequentially. For each phase:

1. **Read** the relevant schema file(s) completely
2. **Analyze** according to the checklist items
3. **Document** all findings (pass/fail/needs attention)
4. **Remediate** any issues before proceeding to next phase
5. **Verify** fixes don't introduce new issues

After completing all phases, provide:
1. Summary of all issues found and fixed
2. Any remaining concerns or recommendations
3. Final schemas ready for submission
4. Schema Store catalog entry JSON

---

## Schema Locations

```
schemas/v1/attempts.schema.json
schemas/v1/primer.schema.json  
schemas/v1/sync.schema.json
```

Begin with Phase 1 on `attempts.schema.json`, then proceed through all phases for each schema before moving to cross-schema analysis.