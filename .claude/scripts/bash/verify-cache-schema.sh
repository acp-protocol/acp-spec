#!/usr/bin/env bash

# Verify cache schema supports RFC-001 directive requirements
#
# This script checks the ACP cache schema for required fields:
# 1. directive field in annotations
# 2. purpose field in file entries
# 3. symbols array with line numbers
# 4. inline annotations array
# 5. auto_generated flag (optional based on decisions)
#
# Usage: ./verify-cache-schema.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --schema <path>     Path to cache schema file
#   --sample <path>     Path to sample cache to validate
#   --decisions <path>  Path to decisions file for Q08
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
SCHEMA_FILE=""
SAMPLE_CACHE=""
DECISIONS_FILE=".acp/spec-decisions.json"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json)
            JSON_MODE=true
            shift
            ;;
        --schema)
            SCHEMA_FILE="$2"
            shift 2
            ;;
        --sample)
            SAMPLE_CACHE="$2"
            shift 2
            ;;
        --decisions)
            DECISIONS_FILE="$2"
            shift 2
            ;;
        --help|-h)
            cat << 'EOF'
Usage: verify-cache-schema.sh [OPTIONS]

Verify ACP cache schema supports RFC-001 directive requirements.

OPTIONS:
  --json              Output in JSON format
  --schema <path>     Path to cache schema (default: auto-detect)
  --sample <path>     Path to sample cache to validate
  --decisions <path>  Path to decisions file (default: .acp/spec-decisions.json)
  --help, -h          Show this help message

CHECKS:
  - directive field in annotation objects
  - purpose field in file entries
  - symbols array with lines, purpose, signature
  - inline array for inline annotations
  - auto_generated flag (if Q08=C decision)
  - constraint aggregation includes directives

EXIT CODES:
  0 - Schema compatible
  1 - Schema needs updates
  2 - Error (schema not found, etc.)

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

# Find schema file
if [[ -z "$SCHEMA_FILE" ]]; then
    for candidate in \
        "$PROJECT_ROOT/schemas/v1/cache.schema.json" \
        "$PROJECT_ROOT/schema/cache.schema.json" \
        "$PROJECT_ROOT/.acp/cache.schema.json"
    do
        if [[ -f "$candidate" ]]; then
            SCHEMA_FILE="$candidate"
            break
        fi
    done
fi

# ============================================================================
# CHECK FUNCTIONS
# ============================================================================

check_field() {
    local file="$1"
    local field="$2"

    if [[ ! -f "$file" ]]; then
        echo "file_not_found"
        return
    fi

    if grep -q "\"$field\"" "$file" 2>/dev/null; then
        echo "present"
    else
        echo "missing"
    fi
}

check_nested_field() {
    local file="$1"
    local parent="$2"
    local field="$3"

    if [[ ! -f "$file" ]]; then
        echo "file_not_found"
        return
    fi

    # Simple heuristic - check if field appears after parent
    if grep -A50 "\"$parent\"" "$file" 2>/dev/null | grep -q "\"$field\""; then
        echo "present"
    else
        echo "missing"
    fi
}

get_decision() {
    local question_id="$1"

    if [[ -f "$DECISIONS_FILE" ]]; then
        jq -r ".decisions.$question_id.choice // \"\"" "$DECISIONS_FILE" 2>/dev/null
    fi
}

# ============================================================================
# VERIFICATION
# ============================================================================

declare -A CHECKS

verify_schema() {
    if [[ -z "$SCHEMA_FILE" ]] || [[ ! -f "$SCHEMA_FILE" ]]; then
        CHECKS["schema_found"]="missing"
        return 1
    fi

    CHECKS["schema_found"]="present"

    # Required fields per RFC-001
    CHECKS["directive_field"]=$(check_field "$SCHEMA_FILE" "directive")
    CHECKS["purpose_field"]=$(check_field "$SCHEMA_FILE" "purpose")
    CHECKS["symbols_array"]=$(check_field "$SCHEMA_FILE" "symbols")
    CHECKS["inline_array"]=$(check_field "$SCHEMA_FILE" "inline")
    CHECKS["lines_field"]=$(check_field "$SCHEMA_FILE" "lines")

    # Nested checks
    CHECKS["symbol_purpose"]=$(check_nested_field "$SCHEMA_FILE" "symbols" "purpose")
    CHECKS["symbol_signature"]=$(check_nested_field "$SCHEMA_FILE" "symbols" "signature")
    CHECKS["symbol_lines"]=$(check_nested_field "$SCHEMA_FILE" "symbols" "lines")
    CHECKS["constraint_directive"]=$(check_nested_field "$SCHEMA_FILE" "constraints" "directive")

    # Optional based on Q08 decision
    local q08_decision=$(get_decision "Q08")
    if [[ "$q08_decision" == "C" ]]; then
        CHECKS["auto_generated_flag"]=$(check_field "$SCHEMA_FILE" "auto_generated")
        CHECKS["auto_generated_required"]=true
    else
        CHECKS["auto_generated_flag"]="not_required"
        CHECKS["auto_generated_required"]=false
    fi
}

validate_sample() {
    if [[ -z "$SAMPLE_CACHE" ]] || [[ ! -f "$SAMPLE_CACHE" ]]; then
        CHECKS["sample_validated"]="no_sample"
        return
    fi

    # Basic JSON validation
    if ! jq empty "$SAMPLE_CACHE" 2>/dev/null; then
        CHECKS["sample_validated"]="invalid_json"
        return
    fi

    # Check for directive fields in sample
    if jq -e '.annotations | to_entries[].value[]?.directive' "$SAMPLE_CACHE" >/dev/null 2>&1; then
        CHECKS["sample_has_directives"]="present"
    else
        CHECKS["sample_has_directives"]="missing"
    fi

    # Check for purpose field
    if jq -e '.files | to_entries[].value.purpose' "$SAMPLE_CACHE" >/dev/null 2>&1; then
        CHECKS["sample_has_purpose"]="present"
    else
        CHECKS["sample_has_purpose"]="missing"
    fi

    CHECKS["sample_validated"]="valid"
}

# ============================================================================
# STATUS DETERMINATION
# ============================================================================

determine_status() {
    local required_missing=0

    for key in directive_field purpose_field symbols_array inline_array; do
        if [[ "${CHECKS[$key]}" == "missing" ]]; then
            ((required_missing++))
        fi
    done

    if [[ "${CHECKS[schema_found]}" == "missing" ]]; then
        echo "error"
    elif [[ $required_missing -gt 0 ]]; then
        echo "needs_update"
    else
        echo "compatible"
    fi
}

collect_required_changes() {
    local changes=()

    [[ "${CHECKS[directive_field]}" == "missing" ]] && \
        changes+=("Add 'directive' field to annotation schema")

    [[ "${CHECKS[purpose_field]}" == "missing" ]] && \
        changes+=("Add 'purpose' field to file entries")

    [[ "${CHECKS[symbols_array]}" == "missing" ]] && \
        changes+=("Add 'symbols' array for symbol-level annotations")

    [[ "${CHECKS[inline_array]}" == "missing" ]] && \
        changes+=("Add 'inline' array for inline annotations")

    [[ "${CHECKS[symbol_lines]}" == "missing" ]] && \
        changes+=("Add 'lines' object to symbol entries")

    [[ "${CHECKS[constraint_directive]}" == "missing" ]] && \
        changes+=("Add 'directive' to constraint aggregation")

    [[ "${CHECKS[auto_generated_required]}" == "true" ]] && \
        [[ "${CHECKS[auto_generated_flag]}" == "missing" ]] && \
        changes+=("Add 'auto_generated' flag per Q08 decision")

    printf '%s\n' "${changes[@]}"
}

# ============================================================================
# OUTPUT
# ============================================================================

output_json() {
    local status=$(determine_status)
    local changes=$(collect_required_changes | jq -R . | jq -s .)

    cat << EOF
{
  "schema_path": "$SCHEMA_FILE",
  "schema_version": "$(jq -r '.version // "unknown"' "$SCHEMA_FILE" 2>/dev/null || echo "unknown")",
  "rfc_reference": "RFC-001",
  "checks": {
    "directive_field": "${CHECKS[directive_field]:-unknown}",
    "purpose_field": "${CHECKS[purpose_field]:-unknown}",
    "symbols_array": "${CHECKS[symbols_array]:-unknown}",
    "inline_array": "${CHECKS[inline_array]:-unknown}",
    "lines_field": "${CHECKS[lines_field]:-unknown}",
    "symbol_purpose": "${CHECKS[symbol_purpose]:-unknown}",
    "symbol_signature": "${CHECKS[symbol_signature]:-unknown}",
    "symbol_lines": "${CHECKS[symbol_lines]:-unknown}",
    "constraint_directive": "${CHECKS[constraint_directive]:-unknown}",
    "auto_generated_flag": "${CHECKS[auto_generated_flag]:-unknown}"
  },
  "overall_status": "$status",
  "required_changes": $changes,
  "sample_cache_valid": $([ "${CHECKS[sample_validated]}" == "valid" ] && echo "true" || echo "false"),
  "migration_needed": $([ "$status" == "needs_update" ] && echo "true" || echo "false")
}
EOF
}

output_text() {
    local status=$(determine_status)

    echo "═══════════════════════════════════════════════════════════════════"
    echo "  Cache Schema Verification Report"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    echo "Schema: $SCHEMA_FILE"
    echo "Status: $status"
    echo ""

    echo "┌─────────────────────────────────────────────────────────────────┐"
    echo "│ RFC-001 Required Fields                                         │"
    echo "├─────────────────────────────────────────────────────────────────┤"
    printf "│ directive field:       %-8s                                │\n" "${CHECKS[directive_field]:-?}"
    printf "│ purpose field:         %-8s                                │\n" "${CHECKS[purpose_field]:-?}"
    printf "│ symbols array:         %-8s                                │\n" "${CHECKS[symbols_array]:-?}"
    printf "│ inline array:          %-8s                                │\n" "${CHECKS[inline_array]:-?}"
    printf "│ lines field:           %-8s                                │\n" "${CHECKS[lines_field]:-?}"
    echo "├─────────────────────────────────────────────────────────────────┤"
    echo "│ Nested Fields                                                   │"
    echo "├─────────────────────────────────────────────────────────────────┤"
    printf "│ symbol.purpose:        %-8s                                │\n" "${CHECKS[symbol_purpose]:-?}"
    printf "│ symbol.signature:      %-8s                                │\n" "${CHECKS[symbol_signature]:-?}"
    printf "│ symbol.lines:          %-8s                                │\n" "${CHECKS[symbol_lines]:-?}"
    printf "│ constraint.directive:  %-8s                                │\n" "${CHECKS[constraint_directive]:-?}"
    echo "└─────────────────────────────────────────────────────────────────┘"
    echo ""

    if [[ "$status" == "needs_update" ]]; then
        echo "Required Changes:"
        echo "─────────────────"
        collect_required_changes | while read -r change; do
            echo "  • $change"
        done
        echo ""
    fi

    echo "═══════════════════════════════════════════════════════════════════"
}

# ============================================================================
# MAIN
# ============================================================================

main() {
    verify_schema
    validate_sample

    if $JSON_MODE; then
        output_json
    else
        output_text
    fi

    # Exit code based on status
    local status=$(determine_status)
    case "$status" in
        compatible) exit 0 ;;
        needs_update) exit 1 ;;
        error) exit 2 ;;
    esac
}

main "$@"