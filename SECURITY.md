# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in ACP, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of these methods:

1. **Email**: security@acp-protocol.dev
2. **GitHub Security Advisories**: [Report a vulnerability](https://github.com/acp-protocol/acp-spec/security/advisories/new)

### What to Include

Please include the following information in your report:

- Type of vulnerability (e.g., injection, information disclosure, etc.)
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Resolution Target**: Within 30 days (depending on complexity)

### What to Expect

1. **Acknowledgment**: We'll confirm receipt of your report
2. **Assessment**: We'll investigate and determine the severity
3. **Communication**: We'll keep you informed of our progress
4. **Resolution**: We'll develop and test a fix
5. **Disclosure**: We'll coordinate public disclosure with you

### Security Considerations for ACP

#### Specification Security

The ACP specification itself doesn't execute code, but implementations should consider:

- **Cache File Integrity**: Cache files could be tampered with
- **Path Traversal**: File paths in cache should be validated
- **Variable Injection**: Variable expansion should be sanitized
- **Constraint Bypass**: Lock constraints are advisory, not enforced

#### Implementation Recommendations

If you're implementing ACP:

1. **Validate all paths** - Don't trust paths in cache files blindly
2. **Sanitize variable expansion** - Prevent injection attacks
3. **Limit file access** - Respect project boundaries
4. **Log constraint violations** - Track when constraints are overridden

#### AI Tool Integration Security

When integrating ACP with AI tools:

1. **Don't expose sensitive data** - Be careful what gets indexed
2. **Respect lock constraints** - Even if advisory, honor them
3. **Audit AI actions** - Log what AI tools modify
4. **Review before commit** - Human review of AI changes

## Scope

This security policy covers:

- The ACP specification (`spec/`)
- JSON schemas (`schemas/`)
- Reference CLI implementation (`cli/`)
- Documentation (`docs/`)

Third-party implementations have their own security policies.

## Recognition

We appreciate responsible disclosure and will acknowledge security researchers who report valid vulnerabilities (with their permission).

## Contact

- **Security Email**: security@acp-protocol.dev
- **General Contact**: hello@acp-protocol.dev
- **GitHub**: https://github.com/acp-protocol/acp-spec