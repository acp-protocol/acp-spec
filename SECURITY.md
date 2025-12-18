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

1. **GitHub Security Advisories** (preferred): [Report a vulnerability](https://github.com/acp-protocol/acp-spec/security/advisories/new)
2. **Email**: security@acp-protocol.dev

### Encrypted Communication

For highly sensitive reports, you may encrypt your message using our PGP key:

- **Key ID**: `[TO BE ADDED]`
- **Fingerprint**: `[TO BE ADDED]`
- **Public Key**: Available at [https://acp-protocol.dev/.well-known/pgp-key.txt](https://acp-protocol.dev/.well-known/pgp-key.txt)

> **Note**: PGP encryption is optional. Unencrypted reports via GitHub Security Advisories are also secure.

### What to Include

Please include the following information in your report:

- Type of vulnerability (e.g., injection, information disclosure, etc.)
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it
- Suggested severity (see [Severity Classification](#severity-classification))

### Severity Classification

Please help us assess the severity using this guide:

| Severity | Description | Example |
|----------|-------------|---------|
| **Critical** | Direct exploitation possible; immediate risk | Cache injection leading to code execution in implementations |
| **High** | Significant security impact; exploitable | Path traversal exposing sensitive files outside project |
| **Medium** | Limited security impact; conditional exploit | Information disclosure requiring specific configuration |
| **Low** | Minimal security impact; theoretical | Attack requiring unlikely conditions or extensive access |
| **Informational** | Security improvement; no direct vulnerability | Hardening suggestion or defense-in-depth enhancement |

## Response Timeline

| Stage | Timeframe |
|-------|-----------|
| **Acknowledgment** | Within 48 hours |
| **Initial Assessment** | Within 7 days |
| **Resolution Target** | Within 30 days (depending on complexity) |

### No Response?

If you haven't received acknowledgment within 72 hours:

1. Check your spam/junk folder for our reply
2. Try the alternative reporting method (email if you used GitHub, or vice versa)
3. Reach out via hello@acp-protocol.dev with subject line "Security Report Follow-up"

## What to Expect

1. **Acknowledgment**: We'll confirm receipt of your report
2. **Assessment**: We'll investigate and determine the severity
3. **Communication**: We'll keep you informed of our progress
4. **Resolution**: We'll develop and test a fix
5. **Disclosure**: We'll coordinate public disclosure with you

### Coordinated Disclosure

We follow a coordinated disclosure process:

- We request a **90-day disclosure window** from initial report to public disclosure
- We will credit you in the security advisory (unless you prefer anonymity)
- We will notify you before any public disclosure
- If we are unable to resolve the issue within 90 days, we will negotiate an extended timeline

## Safe Harbor

We consider security research conducted in accordance with this policy to be:

- **Authorized** concerning any applicable anti-hacking laws (including CFAA)
- **Authorized** concerning any relevant anti-circumvention laws (including DMCA)
- **Exempt** from restrictions in our Terms of Service that would interfere with conducting security research
- **Lawful** and we will not initiate or support legal action against you for accidental, good-faith violations

We will not pursue civil or criminal legal action, or send notices to law enforcement, against researchers who:

- Act in good faith to avoid privacy violations, data destruction, or service interruption
- Only access data necessary to demonstrate the vulnerability
- Do not exploit vulnerabilities beyond proof-of-concept
- Report vulnerabilities promptly and provide reasonable time for remediation
- Do not disclose the issue publicly before coordinated disclosure

**If legal action is initiated by a third party against you** for activities conducted in accordance with this policy, we will take steps to make it known that your actions were authorized by us.

## Bug Bounty

ACP does not currently operate a paid bug bounty program.

We offer:
- Public recognition and acknowledgment for valid reports (with your permission)
- Inclusion in our Security Hall of Fame (if established)
- Our sincere gratitude for helping keep the ACP ecosystem secure

We may consider monetary rewards for exceptional findings on a case-by-case basis, but this is not guaranteed.

## Security Considerations for ACP

### Specification Security

The ACP specification itself doesn't execute code, but implementations should consider:

- **Cache File Integrity**: Cache files could be tampered with to mislead AI tools
- **Path Traversal**: File paths in cache should be validated to prevent directory escape
- **Variable Injection**: Variable expansion should be sanitized to prevent injection
- **Constraint Bypass**: Lock constraints are advisory, not enforced—implementations must not rely on them for security

### Implementation Recommendations

If you're implementing ACP:

1. **Validate all paths** — Don't trust paths in cache files blindly; normalize and verify they're within project boundaries
2. **Sanitize variable expansion** — Prevent injection attacks through variable values
3. **Limit file access** — Respect project boundaries; never follow symlinks outside the project root
4. **Verify cache integrity** — Consider signing or checksumming cache files in high-security environments
5. **Log constraint violations** — Track when constraints are overridden for audit purposes

### AI Tool Integration Security

When integrating ACP with AI tools:

1. **Don't expose sensitive data** — Be careful what gets indexed; exclude secrets, credentials, and PII
2. **Respect lock constraints** — Even if advisory, honor them as indicators of sensitive code
3. **Audit AI actions** — Log what AI tools query and modify
4. **Review before commit** — Human review of AI-generated changes is essential
5. **Limit AI scope** — Consider restricting AI access to specific domains or directories

### Out of Scope

The following are generally NOT considered security vulnerabilities:

- Vulnerabilities in third-party implementations (report to those maintainers)
- Social engineering attacks against ACP maintainers
- Physical security attacks
- Denial of service through malformed input (unless causing resource exhaustion)
- Issues requiring physical access to a user's machine
- Issues in dependencies (report upstream, but let us know if it affects ACP)

## Scope

This security policy covers:

- The ACP specification (`spec/`)
- JSON schemas (`schemas/`)
- Reference CLI implementation (`cli/`)
- Official documentation (`docs/`)
- Official tooling (MCP server, VS Code extension when released)

**Not covered:**
- Third-party ACP implementations (they have their own security policies)
- Community-contributed extensions
- Forks of this repository

## Recognition

We appreciate responsible disclosure and will acknowledge security researchers who report valid vulnerabilities.

### Security Hall of Fame

We maintain a list of researchers who have responsibly disclosed security issues:

<!-- 
Add researchers here as:
- **[Name/Handle](link)** - Brief description of finding (Month Year)
-->

*No entries yet — be the first!*

If you'd like to be acknowledged:
- Let us know your preferred name/handle and optional link
- Indicate if you'd like to be mentioned in the security advisory
- Specify any social media handles you'd like included

## Contact

| Purpose | Contact |
|---------|---------|
| **Security Reports** | security@acp-protocol.dev |
| **General Questions** | hello@acp-protocol.dev |
| **GitHub** | https://github.com/acp-protocol/acp-spec |
| **Security Advisories** | https://github.com/acp-protocol/acp-spec/security/advisories |

---

*This security policy is effective as of December 2024 and may be updated periodically.*
*Last reviewed: December 2024*
