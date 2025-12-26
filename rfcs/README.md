# ACP Request for Comments (RFCs)

This directory contains RFCs (Request for Comments) for significant changes to the AI Context Protocol.

## RFC Index

| RFC | Title | Status | Implemented |
|-----|-------|--------|-------------|
| [RFC-0001](rfc-0001-self-documenting-annotations.md) | Self-Documenting Annotations | Implemented | 2025-12-21 |
| [RFC-0002](rfc-0002-documentation-references-and-style-guides.md) | Documentation References and Style Guides | Implemented | 2025-12-22 |
| [RFC-0003](rfc-0003-annotation-provenance-tracking.md) | Annotation Provenance Tracking | Implemented | 2025-12-22 |
| [RFC-0004](rfc-0004-tiered-interface-primers.md) | Tiered Interface Primers | Implemented | 2025-12-25 |
| [RFC-0005](rfc-0005-cli-provenance-implementation.md) | CLI Provenance Implementation | Implemented | 2025-12-22 |
| [RFC-0006](rfc-0006-documentation-system-bridging.md) | Documentation System Bridging | Implemented | 2025-12-24 |
| [RFC-0007](rfc-0007-acp-complete-documentation-solution.md) | ACP Complete Documentation Solution (Umbrella) | Draft | - |
| [RFC-0008](rfc-0008-acp-type-annotations.md) | ACP Type Annotations | Draft | - |
| [RFC-0009](rfc-0009-extended-annotation-types.md) | Extended Annotation Types | Implemented | 2025-12-25 |
| [RFC-0010](rfc-0010-documentation-generator.md) | ACP Documentation Generator | Draft | - |
| [RFC-0011](rfc-0011-ide-lsp-integration.md) | IDE and LSP Integration | Draft | - |

### Status Legend

| Status | Description |
|--------|-------------|
| **Draft** | Initial proposal, not yet formally submitted |
| **Proposed** | Submitted for review, discussion open |
| **Accepted** | Approved for implementation |
| **Spec Implemented** | Specification and schemas implemented; CLI pending |
| **Implemented** | Fully implemented in the codebase |
| **Rejected** | Not accepted (with documented reasons) |

---

## RFC Process

RFCs are the mechanism for proposing **significant changes** to the ACP protocol. This includes:

- New annotation namespaces or major syntax changes
- New constraint types or behavioral changes
- Breaking changes to file formats (cache, config, schemas)
- Major new features affecting multiple components
- Process or governance changes

For smaller changes, use a standard GitHub Issue or Pull Request instead.

### Lifecycle

```
Draft → Proposed → Final Comment Period (FCP) → Accepted/Rejected → Implemented
```

1. **Draft**: Author creates initial proposal
2. **Proposed**: Formal submission for community review
3. **Final Comment Period (FCP)**: 10 days for final feedback before decision
4. **Accepted/Rejected**: Maintainers make final decision
5. **Implemented**: Code changes merged, RFC marked complete

---

## How to Submit an RFC

### Option 1: GitHub Issue Form (Recommended)

Use the [RFC Proposal Issue Template](../.github/ISSUE_TEMPLATE/rfc_proposal.yml) to submit a new RFC:

1. Go to **Issues** → **New Issue**
2. Select **RFC Proposal**
3. Fill out the structured form
4. Submit for initial discussion

The form guides you through all required sections and ensures completeness.

### Option 2: Direct PR

For contributors familiar with the process:

1. Copy [`TEMPLATE.md`](TEMPLATE.md) to a new file: `rfc-NNNN-short-name.md`
2. Fill in all sections (see template for guidance)
3. Submit a Pull Request
4. Request review from maintainers

### RFC Numbering

- RFCs are numbered sequentially: `RFC-0001`, `RFC-0002`, etc.
- Numbers are assigned when the RFC enters **Proposed** status
- Draft RFCs may use placeholder numbers until formal submission

---

## RFC Template

All RFCs must follow the structure defined in [`TEMPLATE.md`](TEMPLATE.md). Key sections include:

| Section | Required | Description |
|---------|----------|-------------|
| Summary | Yes | One-paragraph overview |
| Motivation | Yes | Problem statement, goals, non-goals |
| Detailed Design | Yes | Full technical specification |
| Schema Changes | If applicable | JSON schema modifications |
| Examples | Yes | Concrete usage examples |
| Drawbacks | Yes | Honest assessment of downsides |
| Alternatives | Yes | Other approaches considered |
| Compatibility | Yes | Backward/forward compatibility |
| Implementation | Yes | Phased implementation plan |
| Open Questions | If any | Unresolved decisions |

---

## Review Process

### For Authors

1. **Be responsive**: Address feedback promptly
2. **Iterate**: RFCs often go through multiple revisions
3. **Champion**: Guide your RFC through the process
4. **Implement**: If accepted, you're expected to help implement (or find someone who will)

### For Reviewers

1. **Be constructive**: Focus on improving the proposal
2. **Ask questions**: Clarify ambiguities early
3. **Consider impact**: Evaluate backward compatibility, complexity, maintenance burden
4. **Suggest alternatives**: If you disagree, propose concrete alternatives

### Decision Criteria

RFCs are evaluated on:

- **Alignment**: Does it fit ACP's goals and philosophy?
- **Feasibility**: Can it be implemented reasonably?
- **Compatibility**: Does it break existing users?
- **Complexity**: Is the added complexity justified?
- **Completeness**: Are all details specified?

---

## Directory Structure

```
rfcs/
├── README.md                    # This file
├── TEMPLATE.md                  # RFC template
├── rfc-0001-*.md               # Individual RFCs
├── rfc-0002-*.md
└── ...
```

---

## Related Resources

- [ACP Specification](../spec/ACP-1.0.md)
- [JSON Schemas](../schemas/v1/)
- [CLI Reference Implementation](../../acp-cli/)
- [GitHub Discussions](https://github.com/anthropics/acp-protocol/discussions)

---

## Questions?

- For RFC process questions: Open a [Discussion](https://github.com/anthropics/acp-protocol/discussions)
- For specific RFC feedback: Comment on the RFC's PR or linked issue
- For implementation questions: See the relevant RFC's implementation notes
