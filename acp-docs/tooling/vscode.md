# ACP VS Code Extension

**Document Type**: Reference + How-To  
**Status**: OUTLINE — Content to be added  
**Last Updated**: December 2025

---

## Overview

The ACP VS Code extension provides IDE integration for the AI Context Protocol, including:
- Real-time annotation highlighting
- Constraint visualization
- IntelliSense for ACP annotations
- Cache status indicators
- Quick fixes and code actions

---

## Installation

> **TODO**: Expand with screenshots and verification steps

### From Marketplace
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "ACP Protocol"
4. Click Install

### From VSIX
```bash
code --install-extension acp-vscode-0.1.0.vsix
```

### From Source
```bash
git clone https://github.com/acp-protocol/acp-vscode
cd acp-vscode
npm install
npm run package
code --install-extension acp-vscode-*.vsix
```

---

## Features

### Annotation Highlighting

> **TODO**: Add screenshots, configuration options

The extension highlights ACP annotations with distinct colors:

| Annotation Type | Color | Example |
|-----------------|-------|---------|
| `@acp:lock frozen` | 🔴 Red | Critical code |
| `@acp:lock restricted` | 🟡 Yellow | Approval needed |
| `@acp:domain` | 🔵 Blue | Domain markers |
| `@acp:fn`, `@acp:class` | 🟢 Green | Documentation |

---

### Constraint Gutter Icons

> **TODO**: Add icon examples, click behaviors

Files with constraints show gutter icons:
- 🔒 Frozen files
- ⚠️ Restricted files
- 📋 Approval required

---

### IntelliSense

> **TODO**: Add GIF showing autocomplete

The extension provides autocomplete for:
- Annotation types (`@acp:`)
- Lock levels (`frozen`, `restricted`, etc.)
- Domain names from cache
- Symbol references

---

### Code Actions

> **TODO**: Document all code actions

Available quick fixes:
- "Add missing @acp:fn annotation"
- "Convert comment to ACP annotation"
- "Generate domain from file path"

---

### Status Bar

> **TODO**: Add screenshot, click actions

The status bar shows:
- Cache status (fresh/stale)
- File constraint level
- Quick actions menu

---

## Configuration

### Extension Settings

```json
{
  "acp.enabled": true,
  "acp.autoIndex": true,
  "acp.highlightAnnotations": true,
  "acp.showGutterIcons": true,
  "acp.showStatusBar": true,
  "acp.cacheWatchMode": "auto"
}
```

### Settings Reference

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `acp.enabled` | boolean | `true` | Enable extension |
| `acp.autoIndex` | boolean | `true` | Auto-reindex on save |
| `acp.highlightAnnotations` | boolean | `true` | Highlight annotations |
| `acp.showGutterIcons` | boolean | `true` | Show constraint icons |
| `acp.showStatusBar` | boolean | `true` | Show status bar item |
| `acp.cacheWatchMode` | string | `"auto"` | `"auto"`, `"manual"`, `"daemon"` |

> **TODO**: Add all settings with detailed descriptions

---

## Commands

> **TODO**: Document all commands with keybindings

| Command | Keybinding | Description |
|---------|------------|-------------|
| `ACP: Index Project` | Ctrl+Shift+A I | Regenerate cache |
| `ACP: Show Constraints` | Ctrl+Shift+A C | Show file constraints |
| `ACP: Go to Domain` | Ctrl+Shift+A D | Jump to domain file |
| `ACP: Add Annotation` | Ctrl+Shift+A A | Insert annotation |

---

## LSP Integration

> **TODO**: Document LSP features

The extension uses a Language Server Protocol (LSP) server for:
- Diagnostics
- Hover information
- Go to definition (for variables)
- Find references

### LSP Configuration

```json
{
  "acp.lsp.enabled": true,
  "acp.lsp.path": "acp-lsp",
  "acp.lsp.trace": "off"
}
```

---

## Troubleshooting

> **TODO**: Expand with common issues

### Extension Not Activating

1. Check that `.acp.config.json` exists
2. Verify the extension is enabled
3. Check Output panel for errors

### Highlighting Not Working

1. Verify language is supported
2. Check `acp.highlightAnnotations` setting
3. Restart VS Code

### Cache Not Updating

1. Check `acp.autoIndex` setting
2. Run `ACP: Index Project` manually
3. Check for indexing errors in Output

---

## Sections to Add

- [ ] **Keyboard Shortcuts**: Complete keybinding reference
- [ ] **Themes**: Customizing annotation colors
- [ ] **Multi-root Workspaces**: Handling multiple projects
- [ ] **Remote Development**: WSL, SSH, Containers
- [ ] **Performance**: Large codebase optimization
- [ ] **Debugging**: Enabling extension debug logging

---

*This document is an outline. [Contribute content →](https://github.com/acp-protocol/acp-vscode)*
