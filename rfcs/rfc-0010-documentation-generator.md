# RFC-0010: ACP Documentation Generator

- **RFC ID**: 0010
- **Title**: Cache-First Documentation Generator
- **Author**: David (ACP Protocol)
- **Status**: Draft
- **Created**: 2025-12-23
- **Updated**: 2025-12-24
- **Discussion**: [Pending GitHub Discussion]
- **Parent**: RFC-0007 (ACP Complete Documentation Solution)
- **Depends On**: Cache schema completeness (symbols, files, domains, constraints, graph)
- **Related**: RFC-0001 (Self-Documenting Annotations)

---

## Summary

This RFC introduces `acp docs`, a documentation generator that renders human-readable documentation directly from `.acp.cache.json` and `.acp.vars.json` using Jinja-style templates.

**Key insight**: The ACP cache is already a structured doclet format. Documentation generation becomes pure template rendering—no parsing, no transformation, just `cache.json → templates → output`.

**Core command**: `acp docs --format html --output ./docs`

---

## Motivation

### Problem Statement

Currently, ACP annotations are machine-readable (via cache) but not human-readable as documentation. Users wanting API docs must:

1. Use native doc generators (JSDoc, Sphinx) which don't understand ACP directives
2. Manually write separate documentation
3. Keep two systems in sync

### The Cache-First Insight

Traditional documentation generators follow this pipeline:

```
Source Files → Parse AST → Extract Doclets → Build Model → Render Output
```

ACP already does most of this work during `acp index`:

```
Source Files → acp index → .acp.cache.json
```

The cache contains structured, typed documentation data:

| Cache Field               | Doclet Equivalent           |
|---------------------------|-----------------------------|
| `symbols[].purpose`       | `@description`              |
| `symbols[].params`        | `@param`                    |
| `symbols[].returns`       | `@returns`                  |
| `symbols[].throws`        | `@throws`                   |
| `symbols[].signature`     | Type signature              |
| `files[].purpose`         | Module description          |
| `files[].inline`          | TODO/FIXME extraction       |
| `domains[]`               | Package/module grouping     |
| `graph`                   | Call relationships          |
| `constraints[].directive` | AI guidance (unique to ACP) |

**Documentation generation therefore reduces to**:

```
.acp.cache.json → Tera Templates → HTML/Markdown/JSON
```

No re-parsing. No complex transformation. Just template rendering.

### Goals

1. Generate professional documentation from cache data
2. Support HTML, Markdown, and JSON output formats
3. Template-based customization (users provide their own templates)
4. Include ACP-specific content (directives, constraints, call graphs)
5. Generate search indexes and navigation
6. Integrate `.acp.vars.json` as a variable reference section

### Non-Goals

1. Re-parsing source files (cache has everything)
2. Complex plugin APIs with lifecycle hooks
3. Runtime documentation serving
4. Documentation hosting

---

## Detailed Design

### 1. Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     acp docs command                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                       Loader                                │
│  - Load .acp.cache.json (required)                          │
│  - Load .acp.vars.json (optional)                           │
│  - Validate against schemas                                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Tera Template Engine                      │
│  - Load templates (built-in or user-provided)               │
│  - Insert cache + vars as context                           │
│  - Render all pages                                         │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Output Writer                           │
│  - Write rendered files                                     │
│  - Copy static assets                                       │
│  - Generate search index                                    │
└─────────────────────────────────────────────────────────────┘
```

### 2. `acp docs` Command

```bash
# Generate documentation
$ acp docs --output ./docs --format html

# Full options
$ acp docs \
  --output ./docs \           # Output directory (default: ./docs/api)
  --format html|markdown|json # Output format (default: html)
  --template ./my-templates \ # Custom template directory
  --cache .acp.cache.json \   # Cache file path
  --vars .acp.vars.json \     # Variables file path
  --title "MyProject API" \   # Documentation title
  --base-url "/api/" \        # Base URL for links
  --include-internal \        # Include @acp:internal symbols
  --no-search                 # Skip search index generation
```

**Note**: No `--include/--exclude` for source files. The cache already reflects what was indexed.

### 3. Template System

Using [Tera](https://keats.github.io/tera/) (Rust's Jinja2 equivalent), templates receive the full cache as context:

```jinja
{# module.html.tera #}
{% extends "base.html.tera" %}

{% block content %}
<article class="module">
  <h1>{{ file.path }}</h1>
  
  {% if file.purpose %}
  <p class="purpose">{{ file.purpose }}</p>
  {% endif %}
  
  {% if file.constraints.lock_level %}
  <aside class="constraint constraint-{{ file.constraints.lock_level }}">
    <span class="badge">{{ file.constraints.lock_level | title }}</span>
    {{ file.constraints.directive }}
  </aside>
  {% endif %}
  
  <h2>Symbols</h2>
  {% for sym_key in file.symbols %}
    {% set sym = cache.symbols[sym_key] %}
    <section class="symbol" id="{{ sym.name }}">
      <h3><code>{{ sym.signature }}</code></h3>
      <p>{{ sym.purpose }}</p>
      
      {% if sym.params %}
      <table class="params">
        <thead><tr><th>Name</th><th>Type</th><th>Description</th></tr></thead>
        <tbody>
        {% for param in sym.params %}
          <tr>
            <td><code>{{ param.name }}</code></td>
            <td><code>{{ param.type }}</code></td>
            <td>{{ param.description }}</td>
          </tr>
        {% endfor %}
        </tbody>
      </table>
      {% endif %}
    </section>
  {% endfor %}
</article>
{% endblock %}
```

**Template context**:
```rust
let mut ctx = Context::new();
ctx.insert("cache", &cache);      // Full cache object
ctx.insert("vars", &vars);        // Variables (optional)
ctx.insert("config", &config);    // Docs config
ctx.insert("file", &current_file); // Current file being rendered
```

### 4. Template Directory Structure

```
templates/
├── html/                    # HTML format templates
│   ├── base.html.tera       # Base layout
│   ├── index.html.tera      # Homepage
│   ├── module.html.tera     # File/module page
│   ├── symbol.html.tera     # Symbol detail page
│   ├── domain.html.tera     # Domain overview
│   ├── variables.html.tera  # Variable reference
│   ├── macros/
│   │   ├── constraint.html.tera
│   │   ├── params.html.tera
│   │   └── nav.html.tera
│   └── assets/
│       ├── style.css
│       └── search.js
├── markdown/                # Markdown format templates
│   ├── index.md.tera
│   ├── module.md.tera
│   └── symbol.md.tera
└── json/                    # JSON format (minimal templating)
    └── docs.json.tera
```

### 5. Custom Templates

Users can override any template by providing a template directory:

```bash
acp docs --template ./my-templates --format html
```

The loader merges user templates with built-in defaults:
1. Check user template directory first
2. Fall back to built-in templates
3. Partial overrides allowed (e.g., only override `module.html.tera`)

### 6. Output Structure

```
docs/
├── index.html              # Overview with stats
├── modules/
│   ├── src-auth-session.html
│   └── src-payments-processor.html
├── domains/
│   ├── authentication.html
│   └── payments.html
├── reference/
│   └── variables.html      # From .acp.vars.json
├── search.json             # Search index
└── assets/
    ├── style.css
    └── search.js
```

### 7. Variable Reference

The `.acp.vars.json` file generates a dedicated reference section:

```jinja
{# variables.html.tera #}
<h1>Variable Reference</h1>
<p>ACP variables provide token-efficient references to code elements.</p>

<h2>Symbol Variables</h2>
<table>
  <thead><tr><th>Variable</th><th>Target</th><th>Description</th></tr></thead>
  <tbody>
  {% for name, var in vars.variables %}
    {% if var.type == "symbol" %}
    <tr>
      <td><code>${{ name }}</code></td>
      <td><a href="../modules/{{ var.value | replace(from=":", to="-") }}.html">{{ var.value }}</a></td>
      <td>{{ var.description | default(value="") }}</td>
    </tr>
    {% endif %}
  {% endfor %}
  </tbody>
</table>
```

### 8. Search Index

Generated as a JSON file for client-side search:

```json
{
  "index": [
    {
      "id": "src/auth/session.ts:validateSession",
      "title": "validateSession",
      "content": "Validates JWT token and returns session data",
      "type": "function",
      "module": "src/auth/session.ts",
      "domain": "authentication",
      "url": "modules/src-auth-session.html#validateSession"
    }
  ]
}
```

Built from cache iteration—no additional parsing.

### 9. Call Graph Visualization

The `cache.graph` data enables call relationship documentation:

```jinja
{% if cache.graph.forward[sym_key] %}
<section class="call-graph">
  <h3>Calls</h3>
  <ul>
  {% for callee in cache.graph.forward[sym_key] %}
    <li><a href="#{{ callee | split(pat=":") | last }}">{{ callee }}</a></li>
  {% endfor %}
  </ul>
</section>
{% endif %}

{% if cache.graph.reverse[sym_key] %}
<section class="called-by">
  <h3>Called By</h3>
  <ul>
  {% for caller in cache.graph.reverse[sym_key] %}
    <li><a href="#{{ caller | split(pat=":") | last }}">{{ caller }}</a></li>
  {% endfor %}
  </ul>
</section>
{% endif %}
```

### 10. Inline Items (TODO/FIXME)

Surface action items from `files[].inline`:

```jinja
{% if file.inline %}
<section class="action-items">
  <h2>Action Items</h2>
  <table>
    <thead><tr><th>Type</th><th>Line</th><th>Description</th></tr></thead>
    <tbody>
    {% for item in file.inline %}
      <tr class="item-{{ item.type }}">
        <td>{{ item.type | upper }}</td>
        <td>{{ item.line }}</td>
        <td>{{ item.text }}</td>
      </tr>
    {% endfor %}
    </tbody>
  </table>
</section>
{% endif %}
```

---

## Configuration

In `.acp.config.json`:

```json
{
  "docs": {
    "output": "./docs/api",
    "format": "html",
    "template": null,
    "title": "MyProject API",
    "baseUrl": "/api/",
    
    "visibility": {
      "internal": false,
      "deprecated": true
    },
    
    "features": {
      "search": true,
      "callGraph": true,
      "actionItems": true,
      "variables": true
    }
  }
}
```

---

## Output Formats

### HTML

Full-featured documentation with:
- Responsive sidebar navigation
- Symbol search
- Dark/light mode toggle
- Constraint badges with colors
- Collapsible sections

### Markdown

For static site generators (Docusaurus, VitePress, MkDocs):

```jinja
{# module.md.tera #}
---
title: {{ file.module | default(value=file.path) }}
sidebar_label: {{ file.path | split(pat="/") | last }}
---

# {{ file.path }}

{{ file.purpose | default(value="") }}

{% if file.constraints.directive %}
:::caution Constraint
**{{ file.constraints.lock_level | title }}**: {{ file.constraints.directive }}
:::
{% endif %}

## Symbols

| Symbol | Type | Description |
|--------|------|-------------|
{% for sym_key in file.symbols %}
{% set sym = cache.symbols[sym_key] %}
| [`{{ sym.name }}`](#{{ sym.name | lower }}) | {{ sym.type }} | {{ sym.purpose | default(value="") | truncate(length=60) }} |
{% endfor %}
```

### JSON

Machine-readable format for external tool integration:

```json
{
  "meta": {
    "version": "1.0.0",
    "generated_at": "{{ cache.generated_at }}",
    "git_commit": "{{ cache.git_commit }}"
  },
  "project": {{ cache.project | json_encode() }},
  "modules": [...],
  "symbols": [...],
  "domains": [...],
  "variables": [...],
  "search_index": [...]
}
```

---

## Examples

### Generate HTML Documentation

```bash
$ acp docs --output ./docs --format html

Reading .acp.cache.json...
  Found 12 files, 156 symbols, 3 domains
Reading .acp.vars.json...
  Found 8 variables
Rendering templates...
  index.html
  modules/src-auth-session.html
  modules/src-payments-processor.html
  domains/authentication.html
  domains/payments.html
  reference/variables.html
Generating search index...
Copying assets...

✓ Documentation generated in ./docs
  Open ./docs/index.html to view
```

### Generate Markdown for VitePress

```bash
$ acp docs --output ./docs/api --format markdown

Reading .acp.cache.json...
Rendering markdown templates...

✓ Generated 18 markdown files in ./docs/api
```

### Use Custom Templates

```bash
$ acp docs --template ./company-theme --format html

Using custom templates from ./company-theme
  Found: base.html.tera, module.html.tera (2 overrides)
  Using built-in: symbol.html.tera, domain.html.tera, variables.html.tera
...
```

---

## Comparison: Cache-First vs Traditional

| Aspect           | Traditional Doc Gen   | ACP Cache-First                       |
|------------------|-----------------------|---------------------------------------| 
| Parsing          | Re-parse every build  | Already done by `acp index`           |
| Language support | Per-language plugins  | Unified via cache schema              |
| Transformation   | AST → doclet → model  | Cache = model (no transformation)     |
| Customization    | Plugin API with hooks | Template files                        |
| Constraints      | Not supported         | First-class (`constraints.directive`) |
| Call graphs      | Usually separate tool | From `cache.graph`                    |
| Variables        | N/A                   | From `.acp.vars.json`                 |
| Incremental      | Complex diffing       | Cache has `git_commit`                |
| Implementation   | Thousands of LOC      | ~300 LOC + templates                  |

---

## Rust Implementation

```rust
use tera::{Tera, Context};
use serde_json::Value;

pub struct DocsGenerator {
    tera: Tera,
    cache: Value,
    vars: Option<Value>,
    config: DocsConfig,
}

impl DocsGenerator {
    pub fn new(template_dir: Option<&Path>) -> Result<Self> {
        let tera = match template_dir {
            Some(dir) => {
                let mut tera = Tera::new(&format!("{}/**/*.tera", dir.display()))?;
                // Merge with built-in templates
                tera.extend(&Self::builtin_templates()?)?;
                tera
            }
            None => Self::builtin_templates()?,
        };
        
        Ok(Self { tera, cache: Value::Null, vars: None, config: DocsConfig::default() })
    }
    
    pub fn load(&mut self, cache_path: &Path, vars_path: Option<&Path>) -> Result<()> {
        self.cache = serde_json::from_str(&fs::read_to_string(cache_path)?)?;
        
        if let Some(vp) = vars_path {
            if vp.exists() {
                self.vars = Some(serde_json::from_str(&fs::read_to_string(vp)?)?);
            }
        }
        Ok(())
    }
    
    pub fn generate(&self, output_dir: &Path, format: Format) -> Result<()> {
        fs::create_dir_all(output_dir)?;
        
        let base_ctx = self.base_context();
        
        // Render index
        self.render_to_file(&base_ctx, "index", output_dir, format)?;
        
        // Render each file/module
        if let Some(files) = self.cache.get("files").and_then(|f| f.as_object()) {
            let modules_dir = output_dir.join("modules");
            fs::create_dir_all(&modules_dir)?;
            
            for (path, file) in files {
                let mut ctx = base_ctx.clone();
                ctx.insert("file", file);
                ctx.insert("path", path);
                
                let filename = path.replace("/", "-").replace(".", "-");
                self.render_to_file(&ctx, "module", &modules_dir.join(&filename), format)?;
            }
        }
        
        // Render domains
        // Render variables
        // Generate search index
        // Copy assets
        
        Ok(())
    }
    
    fn base_context(&self) -> Context {
        let mut ctx = Context::new();
        ctx.insert("cache", &self.cache);
        ctx.insert("vars", &self.vars);
        ctx.insert("config", &self.config);
        ctx
    }
    
    fn render_to_file(&self, ctx: &Context, template: &str, path: &Path, format: Format) -> Result<()> {
        let template_name = format!("{}/{}.{}.tera", format.dir(), template, format.ext());
        let content = self.tera.render(&template_name, ctx)?;
        fs::write(path.with_extension(format.ext()), content)?;
        Ok(())
    }
}
```

---

## Drawbacks

1. **Template learning curve**: Users must learn Tera/Jinja syntax
   - *Mitigation*: Ship excellent default templates; most users won't customize

2. **Less flexible than plugin API**: No lifecycle hooks
   - *Mitigation*: Templates can do 95% of customization; complex needs can preprocess cache

3. **Depends on cache completeness**: Cache must have all doc fields
   - *Mitigation*: This is already the design direction per RFC-0001

---

## Implementation

### Phase 1: Core Generator (1 week)

1. Tera integration with built-in templates
2. Cache + vars loading
3. HTML renderer with default theme
4. Markdown renderer
5. Basic CLI

### Phase 2: Polish (1 week)

1. Search index generation
2. Custom template loading
3. Asset copying
4. JSON output format
5. Call graph rendering

**Total Effort**: ~2 weeks

---

## Alternatives Considered

### 1. TypeScript Plugin API

The original RFC proposed a full plugin system with lifecycle hooks:

```typescript
interface AcpDocsPlugin {
  onInit?(context: PluginContext): Promise<void>;
  onBeforeGenerate?(modules: Module[]): Promise<Module[]>;
  renderSymbol?(symbol: Symbol): Promise<string>;
}
```

**Rejected because**:
- Requires JavaScript runtime in Rust CLI
- Complex to implement and maintain
- Templates provide equivalent customization with less complexity

### 2. Re-parse Source Files

Generate docs by parsing source files directly.

**Rejected because**:
- Duplicates work already done by `acp index`
- Cache IS the parsed representation
- Violates single-source-of-truth principle

### 3. Integrate with Existing Doc Generators

Add ACP output to JSDoc, Sphinx, etc.

**Rejected because**:
- Each tool has different plugin architectures
- Maintenance burden across multiple ecosystems
- Doesn't leverage cache-first advantage

---

## Future Extensions

1. **Docusaurus/MkDocs plugins**: Read cache directly in those ecosystems
2. **Watch mode**: Re-render on cache changes
3. **Mermaid diagrams**: Render call graphs as diagrams
4. **Source links**: Link to GitHub/GitLab source lines
5. **Version comparison**: Diff docs between git commits

---

## Changelog

| Date       | Change                                                  |
|------------|---------------------------------------------------------|
| 2025-12-23 | Initial draft (split from RFC-0007)                     |
| 2025-12-24 | Revised to cache-first architecture with Tera templates |
