# RFC-0012: Built-in Documentation Templates and Theming

- **RFC ID**: 0012
- **Title**: Built-in Documentation Templates and Theming
- **Author**: David (ACP Protocol)
- **Status**: Draft
- **Created**: 2025-12-24
- **Updated**: 2025-12-24
- **Discussion**: [Pending GitHub Discussion]
- **Parent**: RFC-0010 (Cache-First Documentation Generator)
- **Related**: RFC-0007 (ACP Complete Documentation Solution)

---

## Summary

This RFC defines the built-in template and theming system for `acp docs`. Templates and assets are compiled into the CLI binary for zero-configuration documentation generation, with a layered override system for customization.

**Core principles:**
- Zero setup: `acp docs` works out of the box
- Single-file deploy: CSS/JS bundled, no external dependencies
- Theme variants via CSS custom properties
- Progressive customization: override only what you need

---

## Motivation

### Problem Statement

Documentation generators typically require:
1. Installing separate theme packages
2. Configuring template paths
3. Managing asset pipelines
4. Version-matching themes to generators

This friction discourages adoption and complicates CI/CD pipelines.

### Goals

1. **Zero configuration**: `acp docs` produces styled output immediately
2. **Self-contained output**: Generated docs work without external CDNs
3. **Multiple themes**: Ship 2-3 built-in themes for common use cases
4. **Easy customization**: Override templates/CSS without forking
5. **Deterministic output**: Same input produces identical output

### Non-Goals

1. Supporting external theme packages/registries
2. Complex asset pipelines (Sass, PostCSS, bundlers)
3. JavaScript frameworks in templates
4. Runtime theme switching (compile-time only)

---

## Detailed Design

### 1. Asset Compilation Strategy

Templates and assets are embedded at compile time:

```rust
// src/docs/assets.rs
pub const DEFAULT_CSS: &str = include_str!("../../assets/acp-docs.css");
pub const DEFAULT_JS: &str = include_str!("../../assets/acp-docs.js");

pub const TEMPLATE_BASE: &str = include_str!("../../templates/html/base.html.tera");
pub const TEMPLATE_INDEX: &str = include_str!("../../templates/html/index.html.tera");
pub const TEMPLATE_MODULE: &str = include_str!("../../templates/html/module.html.tera");
pub const TEMPLATE_SYMBOL: &str = include_str!("../../templates/html/symbol.html.tera");
pub const TEMPLATE_DOMAIN: &str = include_str!("../../templates/html/domain.html.tera");
pub const TEMPLATE_VARIABLES: &str = include_str!("../../templates/html/variables.html.tera");

// Markdown templates
pub const TEMPLATE_MD_INDEX: &str = include_str!("../../templates/markdown/index.md.tera");
pub const TEMPLATE_MD_MODULE: &str = include_str!("../../templates/markdown/module.md.tera");
```

**Benefits:**
- Single binary distribution
- No runtime file discovery
- Templates always match CLI version
- Works in any directory

### 2. Built-in Themes

Three themes ship with the CLI:

| Theme            | Inspiration   | Use Case                                |
|------------------|---------------|-----------------------------------------|
| `furo` (default) | Sphinx Furo   | Clean, modern, familiar to Python devs  |
| `minimal`        | Plain HTML    | Lightweight, print-friendly, accessible |
| `api`            | Swagger/Redoc | API reference style                     |

```bash
acp docs                    # Uses 'furo' theme
acp docs --theme furo       # Explicit
acp docs --theme minimal    # Lightweight
acp docs --theme api        # API reference style
```

### 3. CSS Architecture

Single CSS file with theme variants via custom properties:

```css
/* acp-docs.css */

/* ============================================
   Base Theme Variables (Furo-inspired)
   ============================================ */
:root {
  /* Colors */
  --acp-bg: #ffffff;
  --acp-bg-secondary: #f8f9fa;
  --acp-text: #24292e;
  --acp-text-muted: #6a737d;
  --acp-link: #0366d6;
  --acp-border: #e1e4e8;
  
  /* Syntax highlighting */
  --acp-code-bg: #f6f8fa;
  --acp-code-text: #24292e;
  
  /* Constraint colors */
  --acp-frozen: #dc3545;
  --acp-restricted: #fd7e14;
  --acp-guarded: #ffc107;
  
  /* Badge colors */
  --acp-badge-pure: #28a745;
  --acp-badge-async: #6f42c1;
  --acp-badge-deprecated: #6c757d;
  --acp-badge-experimental: #17a2b8;
  --acp-badge-internal: #6c757d;
  
  /* Layout */
  --acp-sidebar-width: 280px;
  --acp-content-max: 900px;
  --acp-font-mono: 'SF Mono', 'Cascadia Code', Consolas, monospace;
  --acp-font-sans: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --acp-font-size: 16px;
  --acp-line-height: 1.6;
}

/* ============================================
   Dark Mode
   ============================================ */
[data-theme="dark"] {
  --acp-bg: #0d1117;
  --acp-bg-secondary: #161b22;
  --acp-text: #c9d1d9;
  --acp-text-muted: #8b949e;
  --acp-link: #58a6ff;
  --acp-border: #30363d;
  --acp-code-bg: #161b22;
  --acp-code-text: #c9d1d9;
}

/* ============================================
   Theme: Minimal
   ============================================ */
[data-theme="minimal"],
[data-theme="minimal-dark"] {
  --acp-sidebar-width: 220px;
  --acp-bg-secondary: var(--acp-bg);
  --acp-font-size: 15px;
}

/* ============================================
   Theme: API
   ============================================ */
[data-theme="api"],
[data-theme="api-dark"] {
  --acp-sidebar-width: 320px;
  --acp-content-max: 1100px;
  --acp-bg-secondary: #1a1a2e;
}
```

### 4. Template Override System

Layered resolution with partial overrides:

```
Priority (highest to lowest):
1. --template ./path        CLI flag (explicit)
2. acp-templates/           Project root (versioned, team-shared)
3. .acp/templates/          Local overrides (gitignored)
4. Built-in templates       Compiled into binary
```

**Resolution logic:**

```rust
fn resolve_template(name: &str, config: &Config) -> String {
    // 1. CLI flag override
    if let Some(ref template_dir) = config.template_override {
        let path = template_dir.join(name);
        if path.exists() {
            return fs::read_to_string(path).unwrap();
        }
    }
    
    // 2. Project templates (versioned)
    let project_template = Path::new("acp-templates").join(name);
    if project_template.exists() {
        return fs::read_to_string(project_template).unwrap();
    }
    
    // 3. Local templates (gitignored)
    let local_template = Path::new(".acp/templates").join(name);
    if local_template.exists() {
        return fs::read_to_string(local_template).unwrap();
    }
    
    // 4. Built-in
    get_builtin_template(name)
}
```

**Partial override support:**

Users can override just one template file. Unoverridden templates use built-in defaults.

```
acp-templates/
└── html/
    └── module.html.tera   # Only this overridden
                           # base.html.tera, index.html.tera, etc. use built-in
```

### 5. Template Ejection

Export built-in templates for customization:

```bash
# Eject to versioned project directory
acp docs --eject
# Creates: acp-templates/html/*.tera, acp-templates/assets/*

# Eject to local (gitignored) directory
acp docs --eject --local
# Creates: .acp/templates/html/*.tera, .acp/templates/assets/*

# Eject specific format only
acp docs --eject --format markdown
# Creates: acp-templates/markdown/*.tera
```

**Ejection creates:**

```
acp-templates/
├── html/
│   ├── base.html.tera
│   ├── index.html.tera
│   ├── module.html.tera
│   ├── symbol.html.tera
│   ├── domain.html.tera
│   ├── variables.html.tera
│   └── macros/
│       ├── nav.html.tera
│       ├── badges.html.tera
│       ├── params.html.tera
│       └── callouts.html.tera
├── markdown/
│   ├── index.md.tera
│   ├── module.md.tera
│   └── symbol.md.tera
└── assets/
    ├── acp-docs.css
    └── acp-docs.js
```

### 6. Asset Handling

```bash
# Default: copy built-in assets
acp docs
# Creates: docs/assets/acp-docs.css, docs/assets/acp-docs.js

# Skip asset copying (user provides own)
acp docs --no-assets

# Use custom assets directory
acp docs --assets ./my-assets
```

**Asset override resolution:**

```
1. --assets ./path          CLI flag
2. acp-templates/assets/    Project assets
3. .acp/templates/assets/   Local assets
4. Built-in assets          Compiled into binary
```

### 7. Directory Structure

**Source (in acp-cli repo):**

```
acp-cli/
├── src/
│   └── commands/
│       └── docs.rs
├── assets/
│   ├── acp-docs.css
│   └── acp-docs.js
└── templates/
    ├── html/
    │   ├── base.html.tera
    │   ├── index.html.tera
    │   ├── module.html.tera
    │   ├── symbol.html.tera
    │   ├── domain.html.tera
    │   ├── variables.html.tera
    │   └── macros/
    │       ├── nav.html.tera
    │       ├── badges.html.tera
    │       ├── params.html.tera
    │       └── callouts.html.tera
    └── markdown/
        ├── index.md.tera
        ├── module.md.tera
        └── symbol.md.tera
```

**Generated output:**

```
docs/
├── index.html
├── modules/
│   ├── src-auth-session.html
│   └── src-payments-processor.html
├── domains/
│   ├── authentication.html
│   └── payments.html
├── reference/
│   └── variables.html
├── assets/
│   ├── acp-docs.css
│   └── acp-docs.js
└── search.json
```

### 8. Configuration

In `.acp.config.json`:

```json
{
  "docs": {
    "output": "./docs/api",
    "format": "html",
    "theme": "furo",
    "darkMode": true,
    "template": null,
    "assets": null,
    "title": "My Project API",
    "baseUrl": "/api/",
    "features": {
      "search": true,
      "sourceLinks": true,
      "editLinks": false
    }
  }
}
```

### 9. Built-in Template: HTML (Furo)

**base.html.tera:**

```html
<!DOCTYPE html>
<html lang="en" data-theme="{{ config.theme | default(value='furo') }}">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{% block title %}{{ config.title | default(value=cache.project.name) }}{% endblock %}</title>
  <link rel="stylesheet" href="{{ config.baseUrl | default(value='/') }}assets/acp-docs.css">
</head>
<body>
  <div class="acp-container">
    <nav class="acp-sidebar">
      <div class="acp-search">
        <input type="text" placeholder="Search..." aria-label="Search documentation">
      </div>
      {% include "macros/nav.html.tera" %}
    </nav>
    
    <main class="acp-content">
      {% block content %}{% endblock %}
    </main>
  </div>
  
  <script src="{{ config.baseUrl | default(value='/') }}assets/acp-docs.js"></script>
</body>
</html>
```

**module.html.tera:**

```html
{% extends "base.html.tera" %}

{% block title %}{{ file.module | default(value=path) }} - {{ config.title }}{% endblock %}

{% block content %}
<article class="acp-module">
  <header>
    <h1>{{ path }}</h1>
    
    {% if file.module %}
    <p class="module-name">{{ file.module }}</p>
    {% endif %}
    
    <dl class="acp-meta">
      {% if file.domain %}
      <dt>Domain</dt>
      <dd><a href="../domains/{{ file.domain }}.html">{{ file.domain }}</a></dd>
      {% endif %}
      {% if file.owner %}
      <dt>Owner</dt>
      <dd>{{ file.owner }}</dd>
      {% endif %}
      {% if file.layer %}
      <dt>Layer</dt>
      <dd>{{ file.layer }}</dd>
      {% endif %}
    </dl>
  </header>

  {% if file.purpose %}
  <section class="purpose">
    <p>{{ file.purpose }}</p>
  </section>
  {% endif %}

  {# Constraints #}
  {% if file.constraints %}
    {% include "macros/badges.html.tera" %}
  {% endif %}

  {# Symbols #}
  {% if file.symbols %}
  <section class="symbols">
    <h2>Symbols</h2>
    <table>
      <thead>
        <tr><th>Symbol</th><th>Type</th><th>Description</th></tr>
      </thead>
      <tbody>
      {% for sym_key in file.symbols %}
        {% set sym = cache.symbols[sym_key] %}
        <tr>
          <td><a href="#{{ sym.name }}"><code>{{ sym.name }}</code></a></td>
          <td>{{ sym.type }}</td>
          <td>{{ sym.purpose | default(value="") | truncate(length=80) }}</td>
        </tr>
      {% endfor %}
      </tbody>
    </table>
  </section>
  {% endif %}

  {# Inline items (TODO, FIXME, etc.) #}
  {% if file.inline %}
  <section class="action-items">
    <h2>Action Items</h2>
    <table>
      <thead>
        <tr><th>Type</th><th>Line</th><th>Description</th></tr>
      </thead>
      <tbody>
      {% for item in file.inline %}
        <tr class="item-{{ item.type }}">
          <td>
            {% if item.type == "todo" %}📋{% elif item.type == "fixme" %}🔧{% elif item.type == "critical" %}🚨{% endif %}
            {{ item.type | upper }}
          </td>
          <td>{{ item.line }}</td>
          <td>{{ item.text }}</td>
        </tr>
      {% endfor %}
      </tbody>
    </table>
  </section>
  {% endif %}

  <hr>

  {# Symbol details #}
  {% for sym_key in file.symbols %}
    {% set sym = cache.symbols[sym_key] %}
    <section class="symbol" id="{{ sym.name }}">
      <h3>
        <code class="signature">{{ sym.signature | default(value=sym.name) }}</code>
        {% include "macros/badges.html.tera" with context %}
      </h3>
      
      {% if sym.purpose %}
      <p class="purpose">{{ sym.purpose }}</p>
      {% endif %}
      
      {# AI Directive #}
      {% if sym.constraints.directive %}
      <aside class="callout directive">
        <strong>AI Guidance:</strong> {{ sym.constraints.directive }}
      </aside>
      {% endif %}
      
      {# Parameters #}
      {% if sym.params %}
        {% include "macros/params.html.tera" %}
      {% endif %}
      
      {# Returns #}
      {% if sym.returns %}
      <div class="returns">
        <strong>Returns:</strong> <code>{{ sym.returns.type }}</code>
        {% if sym.returns.description %} — {{ sym.returns.description }}{% endif %}
      </div>
      {% endif %}
      
      {# Throws #}
      {% if sym.throws %}
      <div class="throws">
        <strong>Throws:</strong>
        <ul>
        {% for err in sym.throws %}
          <li><code>{{ err.type }}</code> — {{ err.condition }}</li>
        {% endfor %}
        </ul>
      </div>
      {% endif %}
      
      {# Examples (RFC-0009) #}
      {% if sym.annotations.documentation.examples %}
      <section class="examples">
        <h4>Examples</h4>
        {% for example in sym.annotations.documentation.examples %}
        <pre><code>{{ example }}</code></pre>
        {% endfor %}
      </section>
      {% endif %}
    </section>
  {% endfor %}
</article>
{% endblock %}
```

### 10. Built-in Template: Markdown

**module.md.tera:**

```markdown
---
title: {{ file.module | default(value=path) }}
sidebar_label: {{ path | split(pat="/") | last }}
---

# {{ path }}

{% if file.module %}**Module:** {{ file.module }}{% endif %}
{% if file.domain %}**Domain:** [{{ file.domain }}](../domains/{{ file.domain }}.md){% endif %}
{% if file.owner %}**Owner:** {{ file.owner }}{% endif %}

{% if file.purpose %}
{{ file.purpose }}
{% endif %}

{% if file.constraints.lock_level %}
:::{{ file.constraints.lock_level }}
**{{ file.constraints.lock_level | title }}**{% if file.constraints.directive %}: {{ file.constraints.directive }}{% endif %}
:::
{% endif %}

## Symbols

| Symbol | Type | Description |
|--------|------|-------------|
{% for sym_key in file.symbols %}
{% set sym = cache.symbols[sym_key] %}
| [`{{ sym.name }}`](#{{ sym.name | lower | replace(from=" ", to="-") }}) | {{ sym.type }} | {{ sym.purpose | default(value="") | truncate(length=60) }} |
{% endfor %}

{% if file.inline %}
## Action Items

| Type | Line | Description |
|------|------|-------------|
{% for item in file.inline %}
| {{ item.type | upper }} | {{ item.line }} | {{ item.text }} |
{% endfor %}
{% endif %}

---

{% for sym_key in file.symbols %}
{% set sym = cache.symbols[sym_key] %}
## {{ sym.name }}

```{{ file.language | default(value="typescript") }}
{{ sym.signature | default(value=sym.name) }}
```

{{ sym.purpose | default(value="") }}

{% if sym.constraints.directive %}
:::tip AI Guidance
{{ sym.constraints.directive }}
:::
{% endif %}

{% if sym.params %}
### Parameters

| Name | Type | Description |
|------|------|-------------|
{% for param in sym.params %}
| `{{ param.name }}` | `{{ param.type | default(value="any") }}` | {{ param.description | default(value="") }} |
{% endfor %}
{% endif %}

{% if sym.returns %}
**Returns:** `{{ sym.returns.type }}`{% if sym.returns.description %} — {{ sym.returns.description }}{% endif %}
{% endif %}

{% if sym.throws %}
### Throws

{% for err in sym.throws %}
- `{{ err.type }}` — {{ err.condition }}
  {% endfor %}
  {% endif %}

---

{% endfor %}
```

### 11. JavaScript (Minimal)

```javascript
// acp-docs.js
// Minimal progressive enhancement - no framework dependencies

(function() {
  'use strict';

  // ============================================
  // Theme Toggle
  // ============================================
  function initTheme() {
    const saved = localStorage.getItem('acp-theme');
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const theme = saved || (prefersDark ? 'dark' : 'light');
    document.documentElement.dataset.theme = theme;
  }

  function toggleTheme() {
    const root = document.documentElement;
    const current = root.dataset.theme;
    const next = current.includes('dark') ? current.replace('-dark', '').replace('dark', 'light') 
                                          : current.replace('light', 'dark');
    root.dataset.theme = next || 'dark';
    localStorage.setItem('acp-theme', root.dataset.theme);
  }

  // ============================================
  // Search
  // ============================================
  async function initSearch() {
    const input = document.querySelector('.acp-search input');
    if (!input) return;

    let index = [];
    try {
      const response = await fetch('search.json');
      index = await response.json();
    } catch (e) {
      console.warn('Search index not found');
      return;
    }

    const resultsContainer = document.createElement('div');
    resultsContainer.className = 'acp-search-results';
    input.parentNode.appendChild(resultsContainer);

    input.addEventListener('input', debounce((e) => {
      const query = e.target.value.toLowerCase().trim();
      if (query.length < 2) {
        resultsContainer.innerHTML = '';
        return;
      }

      const results = index.filter(item =>
        item.title.toLowerCase().includes(query) ||
        item.content.toLowerCase().includes(query)
      ).slice(0, 10);

      resultsContainer.innerHTML = results.length
        ? results.map(r => `<a href="${r.url}">${r.title}<span>${r.type}</span></a>`).join('')
        : '<span class="no-results">No results found</span>';
    }, 150));

    // Close on click outside
    document.addEventListener('click', (e) => {
      if (!e.target.closest('.acp-search')) {
        resultsContainer.innerHTML = '';
      }
    });
  }

  // ============================================
  // Utilities
  // ============================================
  function debounce(fn, delay) {
    let timeout;
    return function(...args) {
      clearTimeout(timeout);
      timeout = setTimeout(() => fn.apply(this, args), delay);
    };
  }

  // ============================================
  // Sidebar collapse (mobile)
  // ============================================
  function initSidebar() {
    const sidebar = document.querySelector('.acp-sidebar');
    if (!sidebar) return;

    // Add toggle button for mobile
    const toggle = document.createElement('button');
    toggle.className = 'acp-sidebar-toggle';
    toggle.setAttribute('aria-label', 'Toggle navigation');
    toggle.innerHTML = '☰';
    toggle.addEventListener('click', () => {
      sidebar.classList.toggle('open');
    });
    document.body.prepend(toggle);
  }

  // ============================================
  // Init
  // ============================================
  document.addEventListener('DOMContentLoaded', () => {
    initTheme();
    initSearch();
    initSidebar();

    // Expose toggle for theme button
    window.acpToggleTheme = toggleTheme;
  });
})();
```

---

## CLI Commands

```bash
# Generate docs with defaults
acp docs

# Specify theme
acp docs --theme minimal

# Specify output format
acp docs --format markdown

# Eject templates for customization
acp docs --eject                    # To acp-templates/
acp docs --eject --local            # To .acp/templates/
acp docs --eject --format markdown  # Only markdown templates

# Use custom template directory
acp docs --template ./my-templates

# Skip asset generation
acp docs --no-assets

# Custom assets
acp docs --assets ./my-assets
```

---

## Examples

### Default Usage

```bash
$ acp docs

Reading .acp.cache.json...
  Project: my-app
  Files: 24, Symbols: 156, Domains: 4
Using built-in theme: furo
Generating HTML documentation...
  ✓ index.html
  ✓ modules/ (24 files)
  ✓ domains/ (4 files)
  ✓ reference/variables.html
  ✓ assets/acp-docs.css
  ✓ assets/acp-docs.js
  ✓ search.json

Documentation generated in ./docs
  Open ./docs/index.html to view
```

### Custom Theme Override

```bash
# Eject templates
$ acp docs --eject
Created acp-templates/ with 12 template files

# Edit only what you need
$ vim acp-templates/html/base.html.tera

# Regenerate
$ acp docs
Using custom templates from acp-templates/
  Overrides: base.html.tera
  Built-in: module.html.tera, symbol.html.tera, ...
```

### Markdown for Docusaurus

```bash
$ acp docs --format markdown --output ./docs/api

Generating Markdown documentation...
  ✓ docs/api/index.md
  ✓ docs/api/modules/*.md (24 files)
  ✓ docs/api/domains/*.md (4 files)

Add to docusaurus.config.js:
  docs: { path: 'docs/api', ... }
```

---

## Drawbacks

1. **Binary size increase**: Embedded templates/CSS add ~50-100KB
    - *Mitigation*: Minimal impact; templates compress well

2. **Limited customization without ejecting**: Users must eject to deeply customize
    - *Mitigation*: CSS custom properties allow significant theming without ejecting

3. **Template version coupling**: Ejected templates may drift from CLI updates
    - *Mitigation*: Document breaking changes; provide migration guides

---

## Implementation

### Phase 1: Core Templates (1 week)

1. Create HTML templates (Furo theme)
2. Create CSS with custom properties
3. Create minimal JS (search, dark mode)
4. Embed via `include_str!()`

### Phase 2: Template Resolution (3 days)

1. Implement layered resolution logic
2. Add `--template` flag
3. Add `--eject` command
4. Partial override support

### Phase 3: Additional Themes (3 days)

1. Minimal theme variant
2. API theme variant
3. Markdown templates

### Phase 4: Polish (3 days)

1. Responsive design
2. Print styles
3. Accessibility audit
4. Documentation

**Total Effort**: ~2.5 weeks

---

## Future Extensions

1. **Theme marketplace**: Community-contributed themes
2. **Syntax highlighting**: Integrate highlight.js or Prism
3. **Mermaid diagrams**: Render call graphs as diagrams
4. **PDF export**: Print-optimized stylesheet
5. **i18n**: Template localization support

---

## Changelog

| Date       | Change        |
|------------|---------------|
| 2025-12-24 | Initial draft |