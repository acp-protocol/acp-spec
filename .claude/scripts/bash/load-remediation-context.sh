#!/usr/bin/env bash

# Load remediation context for spec directive updates
#
# This script prepares the environment for spec remediation by:
# 1. Validating decisions file exists
# 2. Loading audit findings
# 3. Identifying files to modify
# 4. Setting up backup directory
#
# Usage: ./load-remediation-context.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --decisions <path>  Path to decisions file
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
DECISIONS_FILE=".acp/spec-decisions.json"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json)
            JSON_MODE=true
            shift
            ;;
        --decisions)
            DECISIONS_FILE="$2"
            shift 2
            ;;
        --help|-h)
            cat << 'EOF'
Usage: load-remediation-context.sh [OPTIONS]

Prepare context for ACP spec remediation.

OPTIONS:
  --json              Output in JSON format
  --decisions <path>  Path to decisions file (default: .acp/spec-decisions.json)
  --help, -h          Show this help message

OUTPUT:
  JSON object with remediation context including:
  - decisions_file: Path to decisions
  - decisions_valid: Whether file is valid
  - audit_findings: Path to audit results
  - files_to_modify: List of spec files needing updates
  - backup_dir: Where backups will be stored

EOF
            exit 0
            ;;
        *)
            echo "ERROR: Unknown option '$1'" >&2
            exit 1
            ;;
    esac
done

# ============================================================================
# CONFIGURATION
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

SPEC_DIR="$PROJECT_ROOT/spec"
CHAPTERS_DIR="$SPEC_DIR/chapters"
OUTPUT_DIR="$PROJECT_ROOT/.acp"
AUDIT_FILE="$OUTPUT_DIR/spec-audit-findings.json"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
BACKUP_DIR="$OUTPUT_DIR/backups/spec-$TIMESTAMP"

# ============================================================================
# VALIDATION
# ============================================================================

decisions_valid=false
if [[ -f "$DECISIONS_FILE" ]]; then
    # Validate JSON syntax
    if jq empty "$DECISIONS_FILE" 2>/dev/null; then
        # Check for required fields
        if jq -e '.decisions' "$DECISIONS_FILE" >/dev/null 2>&1; then
            decisions_valid=true
        fi
    fi
fi

audit_exists=false
[[ -f "$AUDIT_FILE" ]] && audit_exists=true

# ============================================================================
# FILE DISCOVERY
# ============================================================================

files_to_modify=()

# Always include main chapters
[[ -f "$SPEC_DIR/ACP-1.0.md" ]] && files_to_modify+=("spec/ACP-1.0.md")

# Add chapter files based on audit findings
for chapter in "$CHAPTERS_DIR"/*.md; do
    [[ -f "$chapter" ]] && files_to_modify+=("spec/chapters/$(basename "$chapter")")
done

# Get spec version
spec_version="unknown"
if [[ -f "$SPEC_DIR/ACP-1.0.md" ]]; then
    spec_version=$(grep -oP '(?<=Version:\s)[0-9]+\.[0-9]+\.[0-9]+' "$SPEC_DIR/ACP-1.0.md" 2>/dev/null || echo "unknown")
fi

# ============================================================================
# OUTPUT
# ============================================================================

if $JSON_MODE; then
    cat << EOF
{
  "decisions_file": "$DECISIONS_FILE",
  "decisions_valid": $decisions_valid,
  "audit_findings": "$AUDIT_FILE",
  "audit_exists": $audit_exists,
  "spec_version": "$spec_version",
  "target_version": "${spec_version}-revised",
  "files_to_modify": $(printf '%s\n' "${files_to_modify[@]}" | jq -R . | jq -s .),
  "backup_dir": "$BACKUP_DIR",
  "timestamp": "$TIMESTAMP",
  "ready": $([ "$decisions_valid" = true ] && [ "$audit_exists" = true ] && echo "true" || echo "false")
}
EOF
else
    echo "Remediation Context"
    echo "==================="
    echo ""
    echo "Decisions file: $DECISIONS_FILE"
    echo "Decisions valid: $decisions_valid"
    echo "Audit findings: $AUDIT_FILE"
    echo "Audit exists: $audit_exists"
    echo "Spec version: $spec_version"
    echo "Target version: ${spec_version}-revised"
    echo "Backup directory: $BACKUP_DIR"
    echo ""
    echo "Files to modify:"
    for f in "${files_to_modify[@]}"; do
        echo "  - $f"
    done
    echo ""
    if [ "$decisions_valid" = true ] && [ "$audit_exists" = true ]; then
        echo "✓ Ready for remediation"
    else
        echo "✗ Not ready - run audit and clarify first"
    fi
fi