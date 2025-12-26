# Contributing to ACP

Thank you for your interest in contributing to the AI Context Protocol! This document provides guidelines for contributing.

## Ways to Contribute

### 1. Specification Feedback

- Open issues for unclear spec sections
- Suggest improvements to annotation syntax
- Report edge cases not covered

### 2. RFCs (Protocol Changes)

All changes to the specification must go through the RFC process:

1. **Check existing RFCs** - Your idea may already be proposed
2. **Open a discussion** - Gauge interest before writing
3. **Write the RFC** - Use the template in `rfcs/TEMPLATE.md`
4. **Submit PR** - To `rfcs/` with status "Draft" in the header
5. **Address feedback** - Iterate based on review
6. **Final Comment Period** - 10 days for last feedback
7. **Merge** - Update status in RFC header to "Accepted" or "Implemented"

### 3. Reference CLI

- Bug fixes
- New language support
- Performance improvements
- Test coverage

### 4. Documentation

- Improve getting started guide
- Add integration guides
- Fix typos and clarifications
- Translate documentation

### 5. Examples

- Add example projects
- Improve existing examples
- Add edge case examples

## Development Setup

### Prerequisites

- Rust 1.70+ (for CLI)
- Node.js 18+ (for tooling)
- Git

### Clone and Build

```bash
git clone https://github.com/[org]/acp-spec.git
cd acp-spec

# Build CLI
cd cli
cargo build

# Run tests
cargo test

# Run with example
cargo run -- index ../examples/typescript-react
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_annotation_parsing

# With output
cargo test -- --nocapture
```

## Code Style

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Add tests for new functionality

### Documentation

- Use clear, concise language
- Include examples for complex features
- Keep lines under 100 characters
- Use ATX-style headers (`#`)

### Commits

- Use [Conventional Commits](https://www.conventionalcommits.org/):
  - `feat:` New feature
  - `fix:` Bug fix
  - `docs:` Documentation
  - `refactor:` Code refactoring
  - `test:` Adding tests
  - `chore:` Maintenance

- Example: `feat(parser): add Python 3.12 pattern matching support`

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feat/my-feature
   ```
3. **Make changes** with tests
4. **Run checks**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```
5. **Commit** with conventional commit message
6. **Push** and open PR
7. **Address review** feedback
8. **Squash and merge** when approved

### PR Requirements

- [ ] Tests pass
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation updated (if applicable)
- [ ] Changelog entry (for user-facing changes)

## RFC Process Details

### When to Write an RFC

- New annotations
- Changes to existing annotation syntax
- New file formats
- Changes to inheritance/cascade rules
- Major new features

### RFC Template

See `rfcs/TEMPLATE.md` for the full template. Key sections:

1. **Summary** - One paragraph overview
2. **Motivation** - Why is this needed?
3. **Detailed Design** - Complete specification
4. **Drawbacks** - Why not do this?
5. **Alternatives** - What else was considered?
6. **Compatibility** - Breaking changes?

### RFC Review

- Maintainers review within 1 week
- Community feedback period: 2 weeks minimum
- Final Comment Period: 10 days
- Acceptance requires maintainer consensus

## Issue Guidelines

### Bug Reports

Include:
- ACP version
- Minimal reproduction
- Expected vs actual behavior
- Environment details

### Feature Requests

Include:
- Use case description
- Proposed solution (if any)
- Alternatives considered

## Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org/). Be respectful, inclusive, and constructive.

## Questions?

- **GitHub Discussions** - General questions
- **Discord** - Real-time chat
- **Issues** - Bugs and features

## License

By contributing, you agree that your contributions will be licensed under the project's license (Apache 2.0 / MIT).

---

Thank you for contributing to ACP! 🎉
