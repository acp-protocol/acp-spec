---
description: Finalize an RFC implementation by updating all documentation, bumping version numbers according to semantic versioning, and preparing for release.
handoffs:
  - label: Start New RFC
    agent: rfc.analyze
    prompt: Analyze a new RFC
  - label: View All RFCs
    agent: rfc.status
    prompt: Show status of all RFCs
  - label: Create Release
    agent: project.release
    prompt: Create a release with this RFC
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- RFC ID to finalize
- Version override (e.g., `--version 1.2.0`)
- `--dry-run` to preview changes without applying
- `--no-changelog` to skip changelog update
- `--no-commit` to skip git commit
- `--prerelease <tag>` for prerelease versions (alpha, beta, rc)

## Purpose

RFC finalization completes the implementation lifecycle:

| Step | Action | Artifact |
|------|--------|----------|
| Version Bump | Determine and apply version | Updated version files |
| Changelog | Document changes | CHANGELOG.md |
| Documentation | Update all docs | Various doc files |
| RFC Status | Mark as implemented | RFC file |
| Release Prep | Prepare release notes | Release notes |

**Goal**: Ensure all project artifacts are updated, versioned correctly, and ready for release.

## Outline

1. **Verify Prerequisites**:
   - Load RFC and verify status is Accepted
   - Load test report and verify all tests pass
   - Load consistency check report
   - Confirm implementation complete

2. **Run Finalization Script**:
   ```bash
   .claude/scripts/bash/rfc-finalize.sh --json <rfc-id>
   ```
   
   Parse JSON response:
   ```json
   {
     "rfc_id": "RFC-001",
     "title": "Self-Documenting Annotations",
     "finalization_id": "fin-001-20241221-180000",
     "version": {
       "current": "1.0.0",
       "new": "1.1.0",
       "bump_type": "minor",
       "reason": "New feature without breaking changes"
     },
     "files_to_update": {
       "version_files": [
         {"path": "spec/ACP-1.0.md", "current": "1.0", "new": "1.1"},
         {"path": "Cargo.toml", "current": "0.1.0", "new": "0.2.0"},
         {"path": "schemas/v1/cache.schema.json", "current": "1.0.0", "new": "1.1.0"}
       ],
       "changelog": "CHANGELOG.md",
       "rfc_file": "rfcs/accepted/0001-self-documenting-annotations.md",
       "documentation": [
         "docs/annotation-reference.md",
         "docs/cli-reference.md",
         "README.md"
       ]
     },
     "changelog_entry": {
       "version": "1.1.0",
       "date": "2024-12-21",
       "sections": {
         "Added": ["Directive suffix support", "Multi-line directives"],
         "Changed": ["Annotation syntax extended"],
         "Deprecated": [],
         "Fixed": [],
         "Security": []
       }
     },
     "release_notes_draft": "..."
   }
   ```

3. **Determine Version Bump**:
   
   Apply semantic versioning based on RFC changes:
   
   | Change Type | Version Bump | Example |
   |-------------|--------------|---------|
   | Breaking change | MAJOR | 1.0.0 → 2.0.0 |
   | New feature | MINOR | 1.0.0 → 1.1.0 |
   | Bug fix | PATCH | 1.0.0 → 1.0.1 |
   | Pre-release | PRERELEASE | 1.1.0 → 1.1.0-alpha.1 |

4. **Update Version Files**:
   
   For each version file:
   ```bash
   .claude/scripts/bash/rfc-version-bump.sh --file <path> --from <current> --to <new>
   ```
   
   Common version file locations:
   - `spec/ACP-*.md` (spec version in header)
   - `Cargo.toml` (Rust crate version)
   - `package.json` (Node package version)
   - `schemas/v*/*.schema.json` ($version field)
   - `__version__.py` (Python version)

5. **Update Changelog**:
   
   Generate changelog entry following Keep a Changelog format:
   
   ```markdown
   ## [1.1.0] - 2024-12-21
   
   ### Added
   - Directive suffix support for self-documenting annotations ([RFC-001])
   - Multi-line directive continuation with indentation
   - Standard directive recommendations for common annotations
   
   ### Changed
   - Annotation syntax extended with ` - ` directive separator
   - Cache format includes new `directive` field
   
   ### Migration
   - Existing annotations without directives will use default text
   - Run `acp migrate` to add directive suffixes to existing annotations
   
   [RFC-001]: ./rfcs/accepted/0001-self-documenting-annotations.md
   ```

6. **Update RFC Status**:
   
   Update the RFC file:
   - Change status from "Accepted" to "Implemented"
   - Add implementation date
   - Add release version
   - Add implementation notes

7. **Update Documentation**:
   
   For each affected doc file:
   - Update version references
   - Add new feature documentation
   - Update examples
   - Check cross-references

8. **Generate Finalization Report**:
   
   Create `.claude/memory/rfc-finalized-{id}.md`:
   
   ```markdown
   # Finalization Report: RFC-001
   
   **Finalized**: 2024-12-21 18:00:00
   **Version**: 1.0.0 → 1.1.0
   
   ## Version Updates
   | File | Previous | New |
   |------|----------|-----|
   | spec/ACP-1.0.md | 1.0 | 1.1 |
   | Cargo.toml | 0.1.0 | 0.2.0 |
   | cache.schema.json | 1.0.0 | 1.1.0 |
   
   ## Documentation Updates
   - ✓ docs/annotation-reference.md
   - ✓ docs/cli-reference.md
   - ✓ README.md
   
   ## Changelog Entry
   [Full changelog entry...]
   
   ## Release Notes Draft
   [Release notes content...]
   
   ## Next Steps
   1. Review changes
   2. Create git tag
   3. Publish release
   ```

## Semantic Versioning Decision Tree

```
RFC introduces breaking changes?
├─ Yes → MAJOR bump (X.0.0)
│   Examples:
│   - Removing annotations
│   - Changing cache format incompatibly
│   - Changing CLI commands
│
└─ No → RFC adds new features?
    ├─ Yes → MINOR bump (x.Y.0)
    │   Examples:
    │   - New annotation types
    │   - New CLI commands
    │   - New schema fields (additive)
    │
    └─ No → PATCH bump (x.y.Z)
        Examples:
        - Bug fixes
        - Documentation only
        - Performance improvements
```

### Breaking Change Indicators

| Change | Breaking? | Reason |
|--------|-----------|--------|
| Remove annotation type | Yes | Existing code breaks |
| Add annotation type | No | Additive change |
| Change annotation syntax | Yes | Existing annotations invalid |
| Add optional schema field | No | Additive change |
| Change required schema field | Yes | Existing data invalid |
| Change CLI command name | Yes | Scripts break |
| Add CLI command | No | Additive change |
| Change default behavior | Maybe | Depends on impact |

## Changelog Format

Follow [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0] - 2024-12-21

### Added
- Feature description ([RFC-XXX])

### Changed
- Change description

### Deprecated
- Deprecation notice

### Removed
- Removal notice

### Fixed
- Bug fix description

### Security
- Security fix description

[Unreleased]: https://github.com/user/repo/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/user/repo/compare/v1.0.0...v1.1.0
```

## Documentation Update Checklist

### Core Documentation

- [ ] README.md - Feature summary, version badge
- [ ] CONTRIBUTING.md - Development process (if changed)
- [ ] SECURITY.md - Security considerations (if applicable)

### Technical Documentation

- [ ] docs/annotation-reference.md - New/changed annotations
- [ ] docs/cli-reference.md - New/changed commands
- [ ] docs/schema-reference.md - Schema changes
- [ ] docs/migration-guide.md - Migration instructions

### Specification

- [ ] spec/ACP-*.md - Version and content updates
- [ ] spec/chapters/*.md - Affected chapters

### Examples

- [ ] examples/*.md - Updated examples
- [ ] examples/code/*.* - Code examples

## RFC Status Update

Update the RFC header:

```markdown
---
RFC ID: RFC-001
Title: Self-Documenting Annotations
Author: Author Name
Status: Implemented  # Changed from Accepted
Created: 2024-12-01
Updated: 2024-12-21
Implemented: 2024-12-21  # New field
Release: 1.1.0  # New field
---
```

Add implementation notes section:

```markdown
## Implementation Notes

Implemented in version 1.1.0 on 2024-12-21.

### Changes from Original Proposal

1. **Directive length**: Limited to 500 characters (was unlimited)
2. **Default directives**: Added auto-generation for legacy annotations

### Related Commits

- abc1234: Implement directive parser
- def5678: Update cache schema
- ghi9012: Add migration command

### Future Considerations

- May add localization support in future RFC
- Performance optimization for large codebases
```

## Scripts Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `rfc-finalize.sh` | Orchestrate finalization | Main command |
| `rfc-version-bump.sh` | Update version in files | Version step |
| `rfc-changelog.sh` | Generate changelog entry | Changelog step |
| `rfc-doc-update.sh` | Update documentation | Doc step |
| `rfc-release-notes.sh` | Generate release notes | Release prep |

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "Tests not passed" | Test failures exist | Run /rfc.test first |
| "Consistency issues" | Check errors exist | Run /rfc.check first |
| "Invalid version" | Malformed version string | Use semver format |
| "Changelog conflict" | Merge conflict in changelog | Resolve manually |
| "Doc not found" | Referenced doc missing | Check file paths |

## Output Files

| File | Purpose | Location |
|------|---------|----------|
| Finalization Report | Summary of all changes | `.claude/memory/rfc-finalized-{id}.md` |
| Release Notes | Draft release notes | `.claude/memory/rfc-release-notes-{id}.md` |
| Version Log | Version change record | `.claude/memory/rfc-versions-{id}.json` |

## Git Operations

If git is available and `--no-commit` not specified:

1. **Stage all changes**:
   ```bash
   git add -A
   ```

2. **Create commit**:
   ```bash
   git commit -m "feat: Implement RFC-001 Self-Documenting Annotations

   - Add directive suffix support for annotations
   - Update cache schema with directive field
   - Add migration command for existing annotations
   
   Implements: RFC-001
   Version: 1.1.0"
   ```

3. **Create tag** (optional):
   ```bash
   git tag -a v1.1.0 -m "Version 1.1.0 - RFC-001 Implementation"
   ```

## Completion Criteria

### Finalization Complete When:
- [ ] All version files updated
- [ ] Changelog entry added
- [ ] RFC status updated to Implemented
- [ ] All documentation updated
- [ ] Release notes drafted
- [ ] Git commit created (if applicable)
- [ ] Finalization report generated

## Post-Finalization Steps

After finalization, consider:

1. **Review all changes** before pushing
2. **Create release** on GitHub/GitLab
3. **Publish packages** (npm, crates.io, PyPI)
4. **Announce release** to community
5. **Update project board** / close issues
6. **Archive RFC artifacts** in `.claude/memory/archive/`

## Summary Output

At completion, display summary:

```
╔══════════════════════════════════════════════════════════════╗
║                    RFC-001 FINALIZED                         ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  Title: Self-Documenting Annotations                         ║
║  Version: 1.0.0 → 1.1.0 (minor)                              ║
║  Date: 2024-12-21                                            ║
║                                                              ║
║  ✓ Version files updated (3 files)                           ║
║  ✓ Changelog updated                                         ║
║  ✓ RFC status: Implemented                                   ║
║  ✓ Documentation updated (4 files)                           ║
║  ✓ Release notes drafted                                     ║
║  ✓ Git commit created                                        ║
║                                                              ║
║  Report: .claude/memory/rfc-finalized-001.md                 ║
║                                                              ║
║  Next: Review changes, create release, announce              ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

## Complete RFC Lifecycle Summary

This command completes the RFC lifecycle:

```
RFC Lifecycle Complete
══════════════════════

1. /rfc.analyze    ✓ Analyzed and accepted
2. /rfc.refine     ✓ Refined into plan
3. /rfc.implement  ✓ Implemented all tasks
4. /rfc.check      ✓ Consistency verified
5. /rfc.test       ✓ All tests passed
6. /rfc.finalize   ✓ Documentation and version updated

RFC-001 is now complete and ready for release.
```
