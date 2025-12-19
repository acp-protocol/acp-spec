# Protocol Versioning Specification

**ACP Version**: 1.0.0
**Document Version**: 1.0.0
**Last Updated**: 2024-12-18
**Status**: Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Semantic Versioning](#2-semantic-versioning)
3. [File Format Versioning](#3-file-format-versioning)
4. [Compatibility Rules](#4-compatibility-rules)
5. [Version Negotiation](#5-version-negotiation)
6. [Migration](#6-migration)
7. [Deprecation Policy](#7-deprecation-policy)
8. [Extension Versioning](#8-extension-versioning)

---

## 1. Overview

### 1.1 Purpose

This specification defines how ACP versions its protocol, file formats, and extensions. Proper versioning enables:

- **Interoperability** — Tools can communicate version requirements
- **Compatibility** — Older tools can work with newer files (within limits)
- **Evolution** — Protocol can grow without breaking existing implementations
- **Clarity** — Clear expectations about what features are available

### 1.2 Version Locations

Versions appear in multiple places:

| Location | Purpose | Example |
|----------|---------|---------|
| Specification document | Protocol version | `ACP 1.0.0` |
| `.acp.cache.json` | Cache file format version | `"version": "1.0.0"` |
| `.acp.vars.json` | Variables file format version | `"version": "1.0.0"` |
| `.acp.config.json` | Config file format version | `"version": "1.0.0"` |
| JSON Schema `$id` | Schema version | `https://acp-protocol.dev/schemas/v1/cache.schema.json` |
| Implementation claims | Tool conformance | `Implements ACP 1.0.0 Level 2` |

### 1.3 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Semantic Versioning

ACP uses [Semantic Versioning 2.0.0](https://semver.org/) (SemVer) for all version numbers.

### 2.1 Version Format

```
MAJOR.MINOR.PATCH
```

**Examples:**
- `1.0.0` — First stable release
- `1.1.0` — Added features, backward compatible
- `1.1.1` — Bug fixes only
- `2.0.0` — Breaking changes

### 2.2 Version Components

#### Major Version (X.0.0)

Incremented for **breaking changes** that require modifications to existing implementations or files.

**Breaking changes include:**
- Removing required fields from file formats
- Changing the meaning of existing annotations
- Removing annotation namespaces
- Changing constraint behavior semantics
- Modifying query interface contracts
- Removing conformance level requirements

**Example:** If `@acp:lock frozen` behavior changes from "never modify" to "rarely modify," that's a breaking change requiring major version bump.

#### Minor Version (1.X.0)

Incremented for **backward-compatible additions** that don't break existing implementations.

**Backward-compatible additions include:**
- Adding new optional fields to file formats
- Adding new annotation namespaces
- Adding new constraint types
- Adding new query methods
- Adding new conformance level features
- Extending existing annotations with optional parameters

**Example:** Adding `@acp:perf` annotation namespace is a minor version change.

#### Patch Version (1.0.X)

Incremented for **backward-compatible fixes** that don't add features.

**Fixes include:**
- Clarifying specification language
- Fixing errors in examples
- Correcting typos
- Improving documentation
- Fixing schema validation bugs

**Example:** Clarifying that `@acp:lock` values are case-sensitive is a patch change.

### 2.3 Pre-release Versions

Pre-release versions MAY use suffixes:

| Suffix | Meaning | Stability |
|--------|---------|-----------|
| `-alpha` | Early development | Unstable, may change |
| `-beta` | Feature complete | Testing, may change |
| `-rc.N` | Release candidate | Stable unless issues found |

**Examples:**
- `1.0.0-alpha` — Early 1.0 development
- `1.0.0-beta.1` — First beta of 1.0
- `1.0.0-rc.1` — First release candidate
- `1.0.0` — Stable release

### 2.4 Version Precedence

Versions are compared according to SemVer rules:

```
1.0.0-alpha < 1.0.0-beta < 1.0.0-rc.1 < 1.0.0 < 1.0.1 < 1.1.0 < 2.0.0
```

---

## 3. File Format Versioning

### 3.1 Version Field

All ACP JSON files MUST include a `version` field at the root level:

```json
{
  "version": "1.0.0",
  ...
}
```

**Requirements:**
- MUST be a valid SemVer string
- MUST be the first field in the JSON object (RECOMMENDED)
- MUST match the ACP specification version used to generate the file

### 3.2 Cache File Version

The cache file version indicates:
- Which ACP specification was used to generate it
- What fields and structures are present
- What semantics apply to the data

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-18T15:30:00Z",
  ...
}
```

### 3.3 Config File Version

The config file version indicates:
- Which configuration options are valid
- How options are interpreted
- Default values that apply

```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts"],
  ...
}
```

### 3.4 Variables File Version

The variables file version indicates:
- Variable definition format
- Expansion semantics
- Available modifiers

```json
{
  "version": "1.0.0",
  "variables": { ... }
}
```

### 3.5 Schema Versioning

JSON Schemas are versioned in their `$id` URI:

```json
{
  "$schema": "https://json-schema.org/draft-07/schema#",
  "$id": "https://acp-protocol.dev/schemas/v1/cache.schema.json",
  ...
}
```

**Schema URL format:**
```
https://acp-protocol.dev/schemas/v{MAJOR}/{filename}.schema.json
```

**Examples:**
- `https://acp-protocol.dev/schemas/v1/cache.schema.json`
- `https://acp-protocol.dev/schemas/v1/config.schema.json`
- `https://acp-protocol.dev/schemas/v2/cache.schema.json` (future)

---

## 4. Compatibility Rules

### 4.1 Implementation Compatibility

Implementations SHOULD follow these compatibility rules:

| File Version | Implementation Version | Behavior |
|--------------|----------------------|----------|
| Same major, same/older minor | Supported | Full support |
| Same major, newer minor | Supported | MAY miss new features, SHOULD warn |
| Older major | MAY support | Provide migration path or error |
| Newer major | MUST NOT support | Error with clear message |

### 4.2 Major Version Compatibility

Files with different major versions are **incompatible**.

**Implementation behavior:**
```
Cache version: 2.0.0
Implementation: ACP 1.x

Error: Incompatible ACP version.
  Cache requires ACP 2.x, but this tool implements ACP 1.x.
  Please upgrade to an ACP 2.x compatible tool.
```

Implementations MUST:
- Detect major version mismatch
- Refuse to process incompatible files
- Provide clear error message with upgrade path

### 4.3 Minor Version Compatibility

Files with same major but different minor versions are **compatible**.

#### Newer File, Older Implementation

```
Cache version: 1.2.0
Implementation: ACP 1.1.0

Warning: Cache was generated with ACP 1.2.0.
  This tool implements ACP 1.1.0 and may not support all features.
  Consider upgrading for full support.
```

Implementations SHOULD:
- Process the file (it's backward compatible)
- Warn about potential missing features
- Ignore unknown fields gracefully

#### Older File, Newer Implementation

```
Cache version: 1.0.0
Implementation: ACP 1.2.0

OK: Cache is compatible. Full support available.
```

Implementations MUST:
- Process older files without warning
- Apply current semantics to existing fields
- Not require fields added in later versions

### 4.4 Patch Version Compatibility

Patch versions are always compatible within the same major.minor.

```
Cache version: 1.0.1
Implementation: ACP 1.0.0

OK: Fully compatible.
```

### 4.5 Unknown Fields

Implementations MUST handle unknown fields gracefully:

**For reading:**
- MUST ignore unknown top-level fields
- MUST ignore unknown nested fields
- MUST NOT fail on unknown fields
- MAY log warnings about unknown fields

**For writing:**
- MUST preserve unknown fields when updating files
- MUST NOT add fields not defined in claimed version

### 4.6 Compatibility Matrix

| Scenario | Result | Implementation Behavior |
|----------|--------|------------------------|
| `1.0.0` file, `1.0.0` impl | ✓ Full | Normal operation |
| `1.0.0` file, `1.2.0` impl | ✓ Full | Normal operation |
| `1.2.0` file, `1.0.0` impl | ⚠ Partial | Warn, ignore new fields |
| `1.0.0` file, `2.0.0` impl | ? Depends | May support with migration |
| `2.0.0` file, `1.0.0` impl | ✗ None | Error, refuse to process |

---

## 5. Version Negotiation

### 5.1 Detection Algorithm

When an implementation encounters an ACP file:

```
1. READ version field from file
2. PARSE version as SemVer (MAJOR.MINOR.PATCH)
3. COMPARE with implementation's supported version:
   
   IF file.MAJOR > impl.MAJOR:
     ERROR "Incompatible version, upgrade required"
   
   ELSE IF file.MAJOR < impl.MAJOR:
     IF impl.supports_legacy(file.MAJOR):
       WARN "Legacy version, consider regenerating"
       PROCESS with legacy mode
     ELSE:
       ERROR "Unsupported legacy version"
   
   ELSE IF file.MINOR > impl.MINOR:
     WARN "Newer minor version, some features may be unsupported"
     PROCESS normally, ignore unknown fields
   
   ELSE:
     PROCESS normally
```

### 5.2 Version Reporting

Implementations SHOULD report version information:

**CLI:**
```bash
$ acp --version
acp 1.2.0 (ACP Specification 1.0.0, Level 2)
```

**Programmatic:**
```json
{
  "tool_version": "1.2.0",
  "acp_version": "1.0.0",
  "conformance_level": 2
}
```

### 5.3 Multiple Version Support

Implementations MAY support multiple specification versions:

```bash
$ acp index --acp-version 1.0.0
$ acp index --acp-version 1.1.0
```

When supporting multiple versions:
- MUST clearly indicate which version is being used
- MUST generate files with correct version field
- SHOULD default to latest stable version

---

## 6. Migration

### 6.1 Migration Triggers

Migration is needed when:
- Opening a file from an older major version
- Upgrading a project to a new major version
- Converting between incompatible formats

### 6.2 Migration Strategies

#### Automatic Migration

For minor changes that can be automatically converted:

```bash
$ acp migrate .acp.cache.json
Migrating from ACP 1.0.0 to 1.2.0...
  - Adding new field 'stats.annotation_coverage' with default value
  - Converting 'lock_level' values to lowercase
Done. Backup saved to .acp.cache.json.bak
```

#### Manual Migration

For breaking changes requiring human decision:

```bash
$ acp migrate .acp.cache.json --to 2.0.0
Migration to ACP 2.0.0 requires manual changes:

1. @acp:lock behavior has changed:
   - Old: 'restricted' meant "ask before changing"
   - New: 'restricted' means "explain changes first"
   Action: Review all 'restricted' annotations

2. Variable syntax changed from $VAR to ${VAR}:
   - Found 15 variables in .acp.vars.json
   Action: Run 'acp migrate-vars' to convert

3. New required field 'project.repository':
   Action: Add repository URL to .acp.config.json

Run with --interactive for guided migration.
```

### 6.3 Migration Best Practices

**Before migration:**
1. Back up all ACP files
2. Review migration notes for your version jump
3. Test in a branch first
4. Ensure all team members can upgrade

**During migration:**
1. Run migration tool with `--dry-run` first
2. Review proposed changes
3. Run actual migration
4. Verify results with `acp validate`

**After migration:**
1. Test that AI tools still work correctly
2. Verify constraints are respected
3. Check query results match expectations
4. Commit migrated files

### 6.4 Backward Migration

Migrating to older versions is generally NOT supported:
- New features can't be represented in older formats
- Data loss is likely
- Semantics may have changed

If needed, implementations MAY provide limited backward migration:

```bash
$ acp migrate .acp.cache.json --to 1.0.0
Warning: Backward migration may lose data.
  - Field 'stats.annotation_coverage' will be removed
  - New constraint types will be converted to 'normal'
Continue? [y/N]
```

---

## 7. Deprecation Policy

### 7.1 Deprecation Process

Features are deprecated before removal:

```
Version 1.0.0: Feature introduced
Version 1.2.0: Feature deprecated (warning issued)
Version 2.0.0: Feature removed
```

**Minimum deprecation period:** One minor version before removal in next major.

### 7.2 Deprecation Notices

Deprecated features are marked in:

1. **Specification:** With `[DEPRECATED]` label
2. **JSON Schema:** With `deprecated: true`
3. **Implementation:** With runtime warnings

**Specification example:**
```markdown
#### @acp:old-annotation [DEPRECATED]

**Deprecated in:** 1.2.0
**Removed in:** 2.0.0
**Replacement:** Use @acp:new-annotation instead

This annotation is deprecated because...
```

**Schema example:**
```json
{
  "old_field": {
    "type": "string",
    "deprecated": true,
    "description": "DEPRECATED: Use new_field instead"
  }
}
```

### 7.3 Deprecation Warnings

Implementations SHOULD warn on deprecated features:

```
Warning: @acp:old-annotation is deprecated (since ACP 1.2.0)
  Will be removed in ACP 2.0.0
  Use @acp:new-annotation instead
  Location: src/auth/session.ts:15
```

### 7.4 Deprecation Timeline

| Phase | Duration | Behavior |
|-------|----------|----------|
| Active | Until deprecated | Full support, recommended |
| Deprecated | 1+ minor versions | Supported with warnings |
| Removed | After next major | Not supported, error |

### 7.5 Currently Deprecated

**ACP 1.0.0:** No deprecated features.

Future deprecations will be listed here with:
- Feature name
- Deprecation version
- Removal version
- Migration path

---

## 8. Extension Versioning

### 8.1 Extension Namespace Format

Extensions use the `x-` prefix with vendor identifier:

```
@acp:x-{vendor}-{feature}
```

**Examples:**
- `@acp:x-acme-internal`
- `@acp:x-github-actions`
- `@acp:x-mycompany-workflow`

### 8.2 Extension Version Independence

Extensions are versioned independently of the ACP specification:

- Extensions MAY have their own version numbers
- Extensions MUST NOT depend on specific ACP patch versions
- Extensions SHOULD specify minimum ACP major.minor version

### 8.3 Extension Compatibility

Extensions MUST:
- Be preserved in cache files (pass-through)
- Not interfere with core ACP functionality
- Not use reserved namespaces

Extensions SHOULD:
- Document their version requirements
- Provide migration paths for breaking changes
- Follow SemVer for their own versioning

### 8.4 Reserved Namespace Protection

The following patterns are reserved for future ACP use:

- `@acp:x-acp-*` — Reserved for official extensions
- `@acp:perf` — Reserved for performance annotations
- `@acp:security` — Reserved for security annotations
- `@acp:access` — Reserved for access control annotations

**If a future ACP version uses a namespace matching your extension:**
- You MUST migrate to a different namespace
- No automatic conflict resolution is provided
- Choose vendor-specific names to avoid conflicts

### 8.5 Extension Documentation

Extensions SHOULD be documented with:

```markdown
# @acp:x-mycompany-feature Extension

**Version:** 1.0.0
**Requires:** ACP >= 1.0.0
**Author:** My Company

## Description
What this extension does...

## Syntax
@acp:x-mycompany-feature <value>

## Parameters
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| value | string | Yes | The feature value |

## Examples
@acp:x-mycompany-feature "example"

## Changelog
- 1.0.0: Initial release
```

---

## Appendix A: Version History

### ACP 1.0.0 (Current)

**Release Date:** 2024-XX-XX

**Features:**
- Core annotation system
- Constraint system (lock, style, behavior, quality)
- Variable system
- Cache file format
- Three conformance levels
- jq, CLI, and MCP query interfaces

**Breaking Changes:** N/A (initial release)

### Future Versions

Planned for future versions (subject to change):

**ACP 1.1.0:**
- Full debug session specification
- Full hack tracking specification
- Performance annotations (`@acp:perf`)

**ACP 2.0.0:**
- TBD based on community feedback

---

## Appendix B: Version Checking Code

### JavaScript/TypeScript

```typescript
function checkVersion(fileVersion: string, implVersion: string): VersionCheck {
  const file = parseSemVer(fileVersion);
  const impl = parseSemVer(implVersion);
  
  if (file.major > impl.major) {
    return { compatible: false, error: 'Upgrade required' };
  }
  
  if (file.major < impl.major) {
    return { compatible: false, error: 'Legacy version not supported' };
  }
  
  if (file.minor > impl.minor) {
    return { compatible: true, warning: 'Some features may be unsupported' };
  }
  
  return { compatible: true };
}
```

### Python

```python
from packaging import version

def check_version(file_version: str, impl_version: str) -> dict:
    file_v = version.parse(file_version)
    impl_v = version.parse(impl_version)
    
    if file_v.major > impl_v.major:
        return {'compatible': False, 'error': 'Upgrade required'}
    
    if file_v.major < impl_v.major:
        return {'compatible': False, 'error': 'Legacy version not supported'}
    
    if file_v.minor > impl_v.minor:
        return {'compatible': True, 'warning': 'Some features may be unsupported'}
    
    return {'compatible': True}
```

### Rust

```rust
use semver::Version;

fn check_version(file_version: &str, impl_version: &str) -> VersionCheck {
    let file_v = Version::parse(file_version).unwrap();
    let impl_v = Version::parse(impl_version).unwrap();
    
    if file_v.major > impl_v.major {
        return VersionCheck::Incompatible("Upgrade required");
    }
    
    if file_v.major < impl_v.major {
        return VersionCheck::Incompatible("Legacy version not supported");
    }
    
    if file_v.minor > impl_v.minor {
        return VersionCheck::PartiallyCompatible("Some features may be unsupported");
    }
    
    VersionCheck::Compatible
}
```

---

## Appendix C: Related Documents

- [Introduction](01-introduction.md) — Overview and conformance levels
- [Terminology](02-terminology.md) — Definitions
- [Cache Format](03-cache-format.md) — Cache file structure
- [Config Format](04-config-format.md) — Configuration options
- [RFC Template](../rfcs/TEMPLATE.md) — Process for proposing changes

---

*End of Protocol Versioning Specification*