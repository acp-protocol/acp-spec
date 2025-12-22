# RFC Release Notes

<!-- 
TEMPLATE: RFC Release Notes
COMMAND: /rfc.finalize
PURPOSE: Document version changes, updates, and release preparation

INSTRUCTIONS:
- This template is generated after /rfc.test passes
- Version bump type determined by breaking changes in RFC
- Changelog follows Keep a Changelog format (https://keepachangelog.com)
- All version files must be updated consistently
-->

## Release Metadata

| Field | Value |
|-------|-------|
| RFC ID | {{RFC_ID}} |
| RFC Title | {{RFC_TITLE}} |
| Release ID | {{RELEASE_ID}} |
| Previous Version | {{PREVIOUS_VERSION}} |
| New Version | {{NEW_VERSION}} |
| Bump Type | {{BUMP_TYPE}} |
| Release Date | {{RELEASE_DATE}} |
| Prepared By | Claude (via /rfc.finalize) |

---

## Version Summary

### Semantic Version Change

```
{{PREVIOUS_VERSION}} → {{NEW_VERSION}}
         {{VERSION_CHANGE_VISUAL}}
```

<!-- Visual: e.g., "MAJOR bump (breaking changes)" -->

### Bump Type Rationale

**Type: {{BUMP_TYPE}}** (major / minor / patch)

{{BUMP_RATIONALE}}

---

## Changes Overview

### Added

{{#if HAS_ADDED}}
{{ADDED_ITEMS}}
{{else}}
- No new features added
{{/if}}

### Changed

{{#if HAS_CHANGED}}
{{CHANGED_ITEMS}}
{{else}}
- No changes to existing features
{{/if}}

### Deprecated

{{#if HAS_DEPRECATED}}
{{DEPRECATED_ITEMS}}
{{else}}
- No deprecations
{{/if}}

### Removed

{{#if HAS_REMOVED}}
{{REMOVED_ITEMS}}
{{else}}
- No removals
{{/if}}

### Fixed

{{#if HAS_FIXED}}
{{FIXED_ITEMS}}
{{else}}
- No bug fixes
{{/if}}

### Security

{{#if HAS_SECURITY}}
{{SECURITY_ITEMS}}
{{else}}
- No security changes
{{/if}}

---

## Breaking Changes

{{#if HAS_BREAKING_CHANGES}}
**⚠️ This release contains breaking changes**

### Summary

{{BREAKING_SUMMARY}}

### Migration Guide

{{MIGRATION_GUIDE}}

### Affected APIs

| API / Feature | Change | Migration Path |
|---------------|--------|----------------|
{{BREAKING_CHANGES_TABLE}}

### Deprecation Notices

| Item | Deprecated In | Remove In | Replacement |
|------|---------------|-----------|-------------|
{{DEPRECATION_NOTICES}}
{{else}}
✅ No breaking changes in this release.
{{/if}}

---

## Files Updated

### Version Files

| File | Previous | New | Status |
|------|----------|-----|--------|
{{VERSION_FILES_UPDATED}}

### Documentation Files

| File | Update Type | Status |
|------|-------------|--------|
{{DOC_FILES_UPDATED}}

### Schema Files

| Schema | Version | Changes |
|--------|---------|---------|
{{SCHEMA_FILES_UPDATED}}

### RFC Status

| File | Status Change |
|------|---------------|
| {{RFC_FILE}} | {{RFC_STATUS_PREVIOUS}} → Implemented |

---

## Changelog Entry

<!-- This entry will be prepended to CHANGELOG.md -->

```markdown
## [{{NEW_VERSION}}] - {{RELEASE_DATE}}

### Added
{{CHANGELOG_ADDED}}

### Changed
{{CHANGELOG_CHANGED}}

### Deprecated
{{CHANGELOG_DEPRECATED}}

### Removed
{{CHANGELOG_REMOVED}}

### Fixed
{{CHANGELOG_FIXED}}

### Security
{{CHANGELOG_SECURITY}}

### RFC
- Implements [RFC-{{RFC_NUMBER}}](./rfcs/accepted/{{RFC_FILENAME}}): {{RFC_TITLE}}

[{{NEW_VERSION}}]: https://github.com/{{REPO_PATH}}/compare/v{{PREVIOUS_VERSION}}...v{{NEW_VERSION}}
```

---

## Release Checklist

### Pre-Release

- [{{CHECK_TESTS}}] All tests passing
- [{{CHECK_COVERAGE}}] Coverage thresholds met
- [{{CHECK_CONSISTENCY}}] Spec-code consistency verified
- [{{CHECK_DOCS}}] Documentation updated
- [{{CHECK_CHANGELOG}}] Changelog entry created
- [{{CHECK_VERSION}}] Version numbers aligned
- [{{CHECK_BREAKING}}] Breaking changes documented
- [{{CHECK_MIGRATION}}] Migration guide complete (if applicable)

### Version Updates

- [{{CHECK_CARGO}}] cli/Cargo.toml updated
- [{{CHECK_PACKAGE}}] package.json updated (if exists)
- [{{CHECK_SCHEMAS}}] Schema versions updated
- [{{CHECK_SPEC}}] Specification version updated
- [{{CHECK_README}}] README version badge updated

### Documentation

- [{{CHECK_API_DOCS}}] API documentation updated
- [{{CHECK_CLI_DOCS}}] CLI reference updated
- [{{CHECK_ANNOTATION_DOCS}}] Annotation reference updated
- [{{CHECK_RFC_STATUS}}] RFC status updated to Implemented

### Release Artifacts

- [{{CHECK_GIT_TAG}}] Git tag created
- [{{CHECK_GIT_COMMIT}}] Release commit created
- [{{CHECK_RELEASE_NOTES}}] GitHub release notes prepared
- [{{CHECK_ANNOUNCEMENT}}] Announcement draft prepared

---

## Git Operations

### Commit Message

```
{{COMMIT_MESSAGE}}
```

### Tag

```
v{{NEW_VERSION}}
```

### Tag Message

```
{{TAG_MESSAGE}}
```

### Commands (if not using --no-commit)

```bash
# Stage all changes
git add -A

# Create release commit
git commit -m "{{COMMIT_MESSAGE}}"

# Create annotated tag
git tag -a v{{NEW_VERSION}} -m "{{TAG_MESSAGE}}"

# Push (when ready)
git push origin main
git push origin v{{NEW_VERSION}}
```

---

## Announcement Draft

### Title

{{ANNOUNCEMENT_TITLE}}

### Body

{{ANNOUNCEMENT_BODY}}

### Highlights

{{ANNOUNCEMENT_HIGHLIGHTS}}

---

## Post-Release Tasks

- [ ] Push release commit and tag
- [ ] Create GitHub release with notes
- [ ] Publish to crates.io (if applicable)
- [ ] Update documentation website
- [ ] Post announcement to Discord/Twitter
- [ ] Close related GitHub issues
- [ ] Update project roadmap

---

## Rollback Plan

### If Issues Discovered

```bash
# Revert to previous version
git revert HEAD

# Or reset to previous tag
git reset --hard v{{PREVIOUS_VERSION}}

# Force push (with caution)
git push --force origin main
```

### Version Rollback Files

| File | Rollback Value |
|------|----------------|
{{ROLLBACK_FILES}}

---

## Metrics

| Metric | Value |
|--------|-------|
| RFC Processing Time | {{RFC_PROCESSING_TIME}} |
| Implementation Time | {{IMPLEMENTATION_TIME}} |
| Files Changed | {{FILES_CHANGED}} |
| Lines Added | {{LINES_ADDED}} |
| Lines Removed | {{LINES_REMOVED}} |
| Tests Added | {{TESTS_ADDED}} |
| Coverage Delta | {{COVERAGE_DELTA}} |

---

## Handoff Information

<!-- For archival and next RFC -->

```yaml
release_id: {{RELEASE_ID}}
rfc_id: {{RFC_ID}}
previous_version: {{PREVIOUS_VERSION}}
new_version: {{NEW_VERSION}}
bump_type: {{BUMP_TYPE}}
release_date: {{RELEASE_DATE}}
has_breaking_changes: {{HAS_BREAKING_CHANGES}}
status: {{RELEASE_STATUS}}
files_updated:
{{FILES_UPDATED_YAML}}
git:
  commit: {{GIT_COMMIT_SHA}}
  tag: v{{NEW_VERSION}}
  branch: {{GIT_BRANCH}}
next_steps:
  - Push to origin
  - Create GitHub release
  - Announce release
```

---

*Generated by `/rfc.finalize` on {{RELEASE_DATE}}*
*RFC-{{RFC_NUMBER}}: {{RFC_TITLE}}*
*Version {{NEW_VERSION}}*
