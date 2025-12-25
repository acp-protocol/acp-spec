# ACP Documentation Package

**Created**: December 24, 2025  
**Based on**: ACP Documentation Analysis + Diátaxis Framework  
**Status**: Complete structure with content + outlines

---

## Overview

This documentation package implements the recommendations from the ACP Documentation Analysis, creating a dual-structure approach:

1. **User-Facing Documentation** (`acp-docs/`) — Organized by Diátaxis quadrants
2. **Formal Specification** (existing `spec/`) — Kept as authoritative reference

---

## Package Contents

### Complete Documentation (Ready to Use)

| File | Type | Description |
|------|------|-------------|
| `index.md` | Landing | Main entry point, quick links, overview |
| `concepts/why-acp.md` | Explanation | The problem ACP solves + alternatives analysis |
| `concepts/acp-vs-mcp.md` | Explanation | Protocol comparison (on-demand vs always-on) |
| `concepts/acp-vs-rag.md` | Explanation | Why deterministic beats probabilistic |
| `concepts/design-philosophy.md` | Explanation | Core principles |
| `getting-started/quickstart.md` | Tutorial | 5-minute first experience |
| `reference/specification.md` | Reference | Spec chapter index |
| `reference/schemas.md` | Reference | JSON Schema documentation |

### Outlines (Content to be Added)

| File | Type | Description |
|------|------|-------------|
| `tooling/cli.md` | Reference | CLI command reference |
| `tooling/vscode.md` | Reference | VS Code extension |
| `tooling/mcp-server.md` | Reference | MCP Server |
| `tooling/daemon.md` | Reference | Daemon service |
| `guides/index.md` | How-To Index | All how-to guides |
| `guides/annotating-your-codebase.md` | How-To | Add ACP to existing project |
| `guides/integrating-with-cursor.md` | How-To | Cursor IDE setup |
| `guides/protecting-critical-code.md` | How-To | Lock levels |

---

## Directory Structure

```
acp-docs/
├── index.md                           # Landing page
├── concepts/                          # Explanation (Diátaxis)
│   ├── why-acp.md                     # ✅ Complete (with alternatives)
│   ├── acp-vs-mcp.md                  # ✅ Complete (enhanced)
│   ├── acp-vs-rag.md                  # ✅ Complete (new)
│   └── design-philosophy.md           # ✅ Complete
├── getting-started/                   # Tutorial (Diátaxis)
│   └── quickstart.md                  # ✅ Complete
├── reference/                         # Reference (Diátaxis)
│   ├── specification.md               # ✅ Complete
│   └── schemas.md                     # ✅ Complete
├── guides/                            # How-To (Diátaxis)
│   ├── index.md                       # ✅ Complete (index)
│   ├── annotating-your-codebase.md    # 📝 Outline
│   ├── integrating-with-cursor.md     # 📝 Outline
│   └── protecting-critical-code.md    # 📝 Outline
└── tooling/                           # Tool Documentation
    ├── cli.md                         # 📝 Outline
    ├── vscode.md                      # 📝 Outline
    ├── mcp-server.md                  # 📝 Outline
    └── daemon.md                      # 📝 Outline
```

---

## Diátaxis Quadrant Coverage

| Quadrant | Documents | Status |
|----------|-----------|--------|
| **Tutorial** | `quickstart.md`, (first-project.md needed) | ⚠️ Partial |
| **How-To** | `guides/*.md` | ⚠️ Outlines |
| **Reference** | `specification.md`, `schemas.md`, `tooling/*.md` | ✅ Good |
| **Explanation** | `concepts/*.md` | ✅ Complete |

---

## How to Use This Package

### For Documentation Website

1. Copy this structure to your docs site source
2. Configure navigation based on `index.md`
3. Fill in outline sections marked with `> **TODO**:`

### For Fumadocs

```yaml
# fumadocs.config.yaml
nav:
  - title: Home
    href: /
    source: index.md
  - title: Concepts
    pages:
      - concepts/why-acp
      - concepts/acp-vs-mcp
      - concepts/design-philosophy
  - title: Getting Started
    pages:
      - getting-started/quickstart
  - title: Guides
    pages:
      - guides/index
      - guides/annotating-your-codebase
      - guides/integrating-with-cursor
      - guides/protecting-critical-code
  - title: Reference
    pages:
      - reference/specification
      - reference/schemas
  - title: Tooling
    pages:
      - tooling/cli
      - tooling/vscode
      - tooling/mcp-server
      - tooling/daemon
```

---

## Next Steps

### High Priority (Complete These First)

1. **Add installation guide**: `getting-started/installation.md`
2. **Add first-project tutorial**: `getting-started/first-project.md`
3. **Fill CLI reference**: Expand `tooling/cli.md` commands
4. **Add annotation reference**: `reference/annotations.md`

### Medium Priority

1. Fill remaining how-to guide outlines
2. Add screenshots to VS Code extension docs
3. Create ACP vs RAG explanation
4. Add video walkthrough links

### Lower Priority

1. Interactive playground integration
2. Migration guides from specific tools
3. Community showcase section
4. Multi-language examples

---

## Content Guidelines

### Completed Sections

Documents marked ✅ Complete follow these patterns:
- Clear headers and navigation
- Practical examples
- Tables for quick reference
- Cross-links to related content
- Footer with source links

### Outline Sections

Documents marked 📝 Outline include:
- Document structure
- Key section headings
- `> **TODO**:` markers for content to add
- Related links
- Contribution invitation

---

## Contributing

See each document's footer for contribution links.

General process:
1. Pick an outline section
2. Expand `> **TODO**:` markers
3. Add examples and screenshots
4. Submit PR to documentation repo

---

*Generated as part of ACP Documentation Package creation.*
