//! Guardrail annotations for AI behavior control
//!
//! This module handles parsing, storage, and enforcement of guardrail annotations
//! that control how AI systems interact with code.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

/// All guardrail annotations for a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileGuardrails {
    /// Constraint annotations
    #[serde(default, skip_serializing_if = "Constraints::is_empty")]
    pub constraints: Constraints,
    
    /// AI behavior control
    #[serde(default, skip_serializing_if = "AIBehavior::is_empty")]
    pub ai_behavior: AIBehavior,
    
    /// Temporary/experimental markers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub temporary: Vec<TemporaryMarker>,
    
    /// Active troubleshooting attempts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attempts: Vec<Attempt>,
    
    /// Checkpoints for rollback
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoints: Vec<Checkpoint>,
    
    /// Review requirements
    #[serde(default, skip_serializing_if = "ReviewRequirements::is_empty")]
    pub review: ReviewRequirements,
    
    /// Quality markers
    #[serde(default, skip_serializing_if = "QualityMarkers::is_empty")]
    pub quality: QualityMarkers,
}

impl FileGuardrails {
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
            && self.ai_behavior.is_empty()
            && self.temporary.is_empty()
            && self.attempts.is_empty()
            && self.checkpoints.is_empty()
            && self.review.is_empty()
            && self.quality.is_empty()
    }
}

// =============================================================================
// Constraints
// =============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Constraints {
    /// Style guide (e.g., "tailwindcss-v4")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleGuide>,
    
    /// Framework requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameworks: Vec<FrameworkRequirement>,
    
    /// Compatibility requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compat: Vec<String>,
    
    /// Hard requirements that must be maintained
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<String>,
    
    /// Explicitly forbidden patterns
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbids: Vec<String>,
    
    /// Required patterns to follow
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patterns: Vec<String>,
    
    /// Linting rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lint: Vec<String>,
    
    /// Test requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_required: Vec<String>,
}

impl Constraints {
    pub fn is_empty(&self) -> bool {
        self.style.is_none()
            && self.frameworks.is_empty()
            && self.compat.is_empty()
            && self.requires.is_empty()
            && self.forbids.is_empty()
            && self.patterns.is_empty()
            && self.lint.is_empty()
            && self.test_required.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleGuide {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkRequirement {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,
}

// =============================================================================
// AI Behavior Control
// =============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AIBehavior {
    /// AI should not modify this code
    #[serde(default)]
    pub readonly: bool,
    
    /// Reason for readonly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_reason: Option<String>,
    
    /// AI should be extra careful
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub careful: Vec<String>,
    
    /// AI should ask before modifying
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ask_before: Vec<String>,
    
    /// Additional context for AI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    
    /// How AI should approach modifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approach: Option<String>,
    
    /// External references AI should consult
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<String>,
}

impl AIBehavior {
    pub fn is_empty(&self) -> bool {
        !self.readonly
            && self.readonly_reason.is_none()
            && self.careful.is_empty()
            && self.ask_before.is_empty()
            && self.context.is_none()
            && self.approach.is_none()
            && self.references.is_empty()
    }
}

// =============================================================================
// Temporary Markers
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryMarker {
    /// Type of temporary code
    pub kind: TemporaryKind,
    
    /// Description/reason
    pub description: String,
    
    /// Line number or range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<[usize; 2]>,
    
    /// Expiration condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemporaryKind {
    Hack,
    Experiment,
    Debug,
    Wip,
    Temporary,
}

// =============================================================================
// Troubleshooting Attempts
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attempt {
    /// Unique attempt identifier
    pub id: String,
    
    /// What issue this is attempting to fix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_issue: Option<String>,
    
    /// Description of the attempt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Current status
    pub status: AttemptStatus,
    
    /// Failure reason if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    
    /// Lines affected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<[usize; 2]>,
    
    /// Original code (for revert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    
    /// Conditions that should trigger revert
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub revert_if: Vec<String>,
    
    /// Change reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_reason: Option<String>,
    
    /// Timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttemptStatus {
    Active,
    Testing,
    Failed,
    Verified,
    Reverted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Checkpoint name
    pub name: String,
    
    /// File hash at checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

// =============================================================================
// Review Requirements
// =============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewRequirements {
    /// Types of review required
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    
    /// Specific reviewers needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    
    /// AI-generated code markers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ai_generated: Vec<AIGeneratedMarker>,
    
    /// Human verification status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_verified: Option<HumanVerification>,
}

impl ReviewRequirements {
    pub fn is_empty(&self) -> bool {
        self.required.is_empty()
            && self.reviewers.is_empty()
            && self.ai_generated.is_empty()
            && self.human_verified.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGeneratedMarker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default)]
    pub needs_review: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<[usize; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanVerification {
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

// =============================================================================
// Quality Markers
// =============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityMarkers {
    /// Technical debt items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tech_debt: Vec<TechDebtItem>,
    
    /// Complexity warnings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub complexity: Vec<ComplexityMarker>,
    
    /// Code smells
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub smells: Vec<String>,
    
    /// Test coverage info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<String>,
}

impl QualityMarkers {
    pub fn is_empty(&self) -> bool {
        self.tech_debt.is_empty()
            && self.complexity.is_empty()
            && self.smells.is_empty()
            && self.coverage.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtItem {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMarker {
    pub level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

// =============================================================================
// Parser
// =============================================================================

/// Parses guardrail annotations from source code
pub struct GuardrailParser {
    patterns: GuardrailPatterns,
}

struct GuardrailPatterns {
    style: Regex,
    framework: Regex,
    compat: Regex,
    requires: Regex,
    forbids: Regex,
    pattern: Regex,
    ai_readonly: Regex,
    ai_careful: Regex,
    ai_ask: Regex,
    ai_context: Regex,
    ai_approach: Regex,
    ai_reference: Regex,
    hack: Regex,
    experiment: Regex,
    debug: Regex,
    wip: Regex,
    temporary: Regex,
    attempt: Regex,
    attempt_start: Regex,
    attempt_end: Regex,
    checkpoint: Regex,
    revert_if: Regex,
    original: Regex,
    change_reason: Regex,
    review_required: Regex,
    review_by: Regex,
    ai_generated: Regex,
    human_verified: Regex,
    tech_debt: Regex,
    complexity: Regex,
    smell: Regex,
    coverage: Regex,
    test_required: Regex,
    lint: Regex,
}

impl GuardrailParser {
    pub fn new() -> Self {
        Self {
            patterns: GuardrailPatterns {
                style: Regex::new(r"@acp:style\s+(\S+)(?:\s+(\S+))?").unwrap(),
                framework: Regex::new(r"@acp:framework\s+(\S+)(?:@(\S+))?(?:\s+(\S+))?").unwrap(),
                compat: Regex::new(r"@acp:compat\s+(.+)").unwrap(),
                requires: Regex::new(r"@acp:requires\s+(.+)").unwrap(),
                forbids: Regex::new(r"@acp:forbids\s+(.+)").unwrap(),
                pattern: Regex::new(r"@acp:pattern\s+(.+)").unwrap(),
                ai_readonly: Regex::new(r"@acp:ai-readonly(?:\s+reason:(.+))?").unwrap(),
                ai_careful: Regex::new(r"@acp:ai-careful\s+(.+)").unwrap(),
                ai_ask: Regex::new(r"@acp:ai-ask\s+(.+)").unwrap(),
                ai_context: Regex::new(r"@acp:ai-context\s+(.+)").unwrap(),
                ai_approach: Regex::new(r"@acp:ai-approach\s+(.+)").unwrap(),
                ai_reference: Regex::new(r"@acp:ai-reference\s+(.+)").unwrap(),
                hack: Regex::new(r"@acp:hack\s+(.+)").unwrap(),
                experiment: Regex::new(r"@acp:experiment\s+(.+)").unwrap(),
                debug: Regex::new(r"@acp:debug\s+(.+)").unwrap(),
                wip: Regex::new(r"@acp:wip\s+(.+)").unwrap(),
                temporary: Regex::new(r"@acp:temporary\s+(.+)").unwrap(),
                attempt: Regex::new(r"@acp:attempt\s+id:(\S+)(?:\s+for:(\S+))?(?:\s+(.+))?").unwrap(),
                attempt_start: Regex::new(r"@acp:attempt-start\s+id:(\S+)(?:\s+for:(\S+))?(?:\s+description:(.+))?").unwrap(),
                attempt_end: Regex::new(r"@acp:attempt-end\s+id:(\S+)(?:\s+status:(\S+))?").unwrap(),
                checkpoint: Regex::new(r"@acp:checkpoint\s+name:(\S+)(?:\s+hash:(\S+))?").unwrap(),
                revert_if: Regex::new(r"@acp:revert-if\s+(.+)").unwrap(),
                original: Regex::new(r"@acp:original\s*(.*)").unwrap(),
                change_reason: Regex::new(r"@acp:change-reason\s+(.+)").unwrap(),
                review_required: Regex::new(r"@acp:review-required\s+(.+)").unwrap(),
                review_by: Regex::new(r"@acp:review-by\s+(.+)").unwrap(),
                ai_generated: Regex::new(r"@acp:ai-generated(?:\s+model:(\S+))?(?:\s+date:(\S+))?").unwrap(),
                human_verified: Regex::new(r"@acp:human-verified(?:\s+by:(\S+))?(?:\s+date:(\S+))?").unwrap(),
                tech_debt: Regex::new(r"@acp:tech-debt\s+(.+)").unwrap(),
                complexity: Regex::new(r"@acp:complexity\s+(.+)").unwrap(),
                smell: Regex::new(r"@acp:smell\s+(.+)").unwrap(),
                coverage: Regex::new(r"@acp:coverage\s+(.+)").unwrap(),
                test_required: Regex::new(r"@acp:test-required\s+(.+)").unwrap(),
                lint: Regex::new(r"@acp:lint\s+(.+)").unwrap(),
            },
        }
    }

    /// Parse all guardrails from source content
    pub fn parse(&self, content: &str) -> FileGuardrails {
        let mut guardrails = FileGuardrails::default();

        for (line_num, line) in content.lines().enumerate() {
            self.parse_line(line, line_num + 1, &mut guardrails);
        }

        guardrails
    }

    fn parse_line(&self, line: &str, _line_num: usize, g: &mut FileGuardrails) {
        // Style
        if let Some(cap) = self.patterns.style.captures(line) {
            g.constraints.style = Some(StyleGuide {
                name: cap.get(1).unwrap().as_str().to_string(),
                url: cap.get(2).map(|m| m.as_str().to_string()),
            });
        }

        // Framework
        if let Some(cap) = self.patterns.framework.captures(line) {
            g.constraints.frameworks.push(FrameworkRequirement {
                name: cap.get(1).unwrap().as_str().to_string(),
                version: cap.get(2).map(|m| m.as_str().to_string()),
                docs_url: cap.get(3).map(|m| m.as_str().to_string()),
            });
        }

        // Requires
        if let Some(cap) = self.patterns.requires.captures(line) {
            let items: Vec<_> = cap.get(1).unwrap().as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            g.constraints.requires.extend(items);
        }

        // Forbids
        if let Some(cap) = self.patterns.forbids.captures(line) {
            let items: Vec<_> = cap.get(1).unwrap().as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            g.constraints.forbids.extend(items);
        }

        // AI Readonly
        if let Some(cap) = self.patterns.ai_readonly.captures(line) {
            g.ai_behavior.readonly = true;
            g.ai_behavior.readonly_reason = cap.get(1).map(|m| m.as_str().to_string());
        }

        // AI Careful
        if let Some(cap) = self.patterns.ai_careful.captures(line) {
            g.ai_behavior.careful.push(cap.get(1).unwrap().as_str().to_string());
        }

        // AI Ask
        if let Some(cap) = self.patterns.ai_ask.captures(line) {
            g.ai_behavior.ask_before.push(cap.get(1).unwrap().as_str().to_string());
        }

        // AI Context
        if let Some(cap) = self.patterns.ai_context.captures(line) {
            let ctx = cap.get(1).unwrap().as_str().to_string();
            if let Some(existing) = &mut g.ai_behavior.context {
                existing.push('\n');
                existing.push_str(&ctx);
            } else {
                g.ai_behavior.context = Some(ctx);
            }
        }

        // AI Reference
        if let Some(cap) = self.patterns.ai_reference.captures(line) {
            g.ai_behavior.references.push(cap.get(1).unwrap().as_str().to_string());
        }

        // Hack
        if let Some(cap) = self.patterns.hack.captures(line) {
            g.temporary.push(TemporaryMarker {
                kind: TemporaryKind::Hack,
                description: cap.get(1).unwrap().as_str().to_string(),
                lines: None,
                until: None,
            });
        }

        // Attempt
        if let Some(cap) = self.patterns.attempt_start.captures(line) {
            g.attempts.push(Attempt {
                id: cap.get(1).unwrap().as_str().to_string(),
                for_issue: cap.get(2).map(|m| m.as_str().to_string()),
                description: cap.get(3).map(|m| m.as_str().to_string()),
                status: AttemptStatus::Active,
                failure_reason: None,
                lines: None,
                original: None,
                revert_if: vec![],
                change_reason: None,
                timestamp: None,
            });
        }

        // Checkpoint
        if let Some(cap) = self.patterns.checkpoint.captures(line) {
            g.checkpoints.push(Checkpoint {
                name: cap.get(1).unwrap().as_str().to_string(),
                hash: cap.get(2).map(|m| m.as_str().to_string()),
                description: None,
                timestamp: None,
            });
        }

        // Review required
        if let Some(cap) = self.patterns.review_required.captures(line) {
            let items: Vec<_> = cap.get(1).unwrap().as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            g.review.required.extend(items);
        }

        // Tech debt
        if let Some(cap) = self.patterns.tech_debt.captures(line) {
            g.quality.tech_debt.push(TechDebtItem {
                description: cap.get(1).unwrap().as_str().to_string(),
                priority: None,
                effort: None,
            });
        }

        // Test required
        if let Some(cap) = self.patterns.test_required.captures(line) {
            let items: Vec<_> = cap.get(1).unwrap().as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            g.constraints.test_required.extend(items);
        }
    }
}

impl Default for GuardrailParser {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Enforcer
// =============================================================================

/// Result of checking guardrails against proposed changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailCheck {
    pub passed: bool,
    pub violations: Vec<Violation>,
    pub warnings: Vec<Warning>,
    pub required_actions: Vec<RequiredAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub rule: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub rule: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredAction {
    pub action: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Enforces guardrails on proposed changes
pub struct GuardrailEnforcer;

impl GuardrailEnforcer {
    /// Check if AI should modify this file
    pub fn can_modify(guardrails: &FileGuardrails) -> GuardrailCheck {
        let mut check = GuardrailCheck {
            passed: true,
            violations: vec![],
            warnings: vec![],
            required_actions: vec![],
        };

        // Check readonly
        if guardrails.ai_behavior.readonly {
            check.passed = false;
            check.violations.push(Violation {
                rule: "ai-readonly".to_string(),
                message: format!(
                    "File is marked as AI-readonly{}",
                    guardrails.ai_behavior.readonly_reason
                        .as_ref()
                        .map(|r| format!(": {}", r))
                        .unwrap_or_default()
                ),
                severity: Severity::Error,
            });
        }

        // Check ai-careful
        for careful in &guardrails.ai_behavior.careful {
            check.warnings.push(Warning {
                rule: "ai-careful".to_string(),
                message: format!("Extra caution required: {}", careful),
            });
        }

        // Check ai-ask
        for ask in &guardrails.ai_behavior.ask_before {
            check.required_actions.push(RequiredAction {
                action: "ask-user".to_string(),
                reason: format!("Must ask before: {}", ask),
            });
        }

        // Check review requirements
        for review in &guardrails.review.required {
            check.required_actions.push(RequiredAction {
                action: "flag-for-review".to_string(),
                reason: format!("Requires {} review", review),
            });
        }

        check
    }

    /// Check if proposed changes violate constraints
    pub fn check_changes(
        guardrails: &FileGuardrails,
        proposed_content: &str,
    ) -> GuardrailCheck {
        let mut check = Self::can_modify(guardrails);

        // Check forbids
        for forbidden in &guardrails.constraints.forbids {
            if proposed_content.contains(forbidden) {
                check.passed = false;
                check.violations.push(Violation {
                    rule: "forbids".to_string(),
                    message: format!("Contains forbidden pattern: {}", forbidden),
                    severity: Severity::Error,
                });
            }
        }

        // Check test requirements
        if !guardrails.constraints.test_required.is_empty() {
            check.required_actions.push(RequiredAction {
                action: "write-tests".to_string(),
                reason: format!(
                    "Tests required: {}",
                    guardrails.constraints.test_required.join(", ")
                ),
            });
        }

        check
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_readonly() {
        let parser = GuardrailParser::new();
        let content = "// @acp:ai-readonly reason:security-audited\nfunction secure() {}";
        let guardrails = parser.parse(content);
        
        assert!(guardrails.ai_behavior.readonly);
        assert_eq!(
            guardrails.ai_behavior.readonly_reason,
            Some("security-audited".to_string())
        );
    }

    #[test]
    fn test_parse_forbids() {
        let parser = GuardrailParser::new();
        let content = "// @acp:forbids eval, Function, inline-styles";
        let guardrails = parser.parse(content);
        
        assert_eq!(guardrails.constraints.forbids.len(), 3);
        assert!(guardrails.constraints.forbids.contains(&"eval".to_string()));
    }

    #[test]
    fn test_enforcer_readonly() {
        let guardrails = FileGuardrails {
            ai_behavior: AIBehavior {
                readonly: true,
                readonly_reason: Some("test".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let check = GuardrailEnforcer::can_modify(&guardrails);
        assert!(!check.passed);
        assert_eq!(check.violations.len(), 1);
    }
}
