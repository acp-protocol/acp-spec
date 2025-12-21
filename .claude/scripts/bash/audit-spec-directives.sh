#!/usr/bin/env bash

# Audit ACP specification for RFC-001 directive compliance
#
# This script analyzes spec/ACP-1.0.md and spec/chapters/* to identify:
# 1. Annotations without directive requirements documented
# 2. Examples missing directive suffixes
# 3. Cache schema gaps for directive fields
# 4. Missing hierarchical annotation documentation
#
# Usage: ./audit-spec-directives.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --strict            Treat SHOULD violations as errors
#   --questions-only    Only output open questions
#   --chapters-only     Only audit chapter files
#   --main-only         Only audit ACP-1.0.md
#   --focus <section>   Focus on specific sections
#   --help, -h          Show help message
#
# EXIT CODES:
#   0 - Clean (no blockers)
#   1 - Blockers found
#   2 - Error (missing files, etc.)

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
STRICT_MODE=false
QUESTIONS_ONLY=false
CHAPTERS_ONLY=false
MAIN_ONLY=false
FOCUS_SECTION=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json)
            JSON_MODE=true
            shift
            ;;
        --strict)
            STRICT_MODE=true
            shift
            ;;
        --questions-only)
            QUESTIONS_ONLY=true
            shift
            ;;
        --chapters-only)
            CHAPTERS_ONLY=true
            shift
            ;;
        --main-only)
            MAIN_ONLY=true
            shift
            ;;
        --focus)
            FOCUS_SECTION="$2"
            shift 2
            ;;
        --help|-h)
            cat << 'EOF'
Usage: audit-spec-directives.sh [OPTIONS]

Audit ACP specification for RFC-001 (Self-Documenting Annotations) compliance.

OPTIONS:
  --json              Output in JSON format for parsing
  --strict            Treat SHOULD violations as errors (exit 1)
  --questions-only    Only output open questions needing decisions
  --chapters-only     Only audit spec/chapters/ files
  --main-only         Only audit spec/ACP-1.0.md
  --focus <section>   Focus on specific section (annotations, cache, constraints)
  --help, -h          Show this help message

OUTPUTS:
  - Blockers (B): MUST requirements from RFC-001 not met
  - Violations (V): SHOULD requirements not met
  - Warnings (W): MAY recommendations not followed
  - Info (I): Observations and suggestions

EXIT CODES:
  0 - Clean (no blockers found)
  1 - Blockers found
  2 - Error (missing files, invalid spec)

EXAMPLES:
  # Full audit with JSON output
  ./audit-spec-directives.sh --json

  # Strict mode (SHOULD violations are errors)
  ./audit-spec-directives.sh --strict

  # Only show open questions
  ./audit-spec-directives.sh --questions-only

  # Focus on annotation section
  ./audit-spec-directives.sh --focus annotations

EOF
            exit 0
            ;;
        --*)
            echo "ERROR: Unknown option '$1'. Use --help for usage." >&2
            exit 2
            ;;
        *)
            echo "ERROR: Unexpected argument '$1'. Use --help for usage." >&2
            exit 2
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
MAIN_SPEC="$SPEC_DIR/ACP-1.0.md"
RFC_FILE="$PROJECT_ROOT/rfc-001-self-documenting-annotations.md"
CACHE_SCHEMA="$PROJECT_ROOT/schemas/v1/cache.schema.json"
OUTPUT_DIR="$PROJECT_ROOT/.acp"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# ============================================================================
# VALIDATION
# ============================================================================

validate_paths() {
    local errors=()

    [[ ! -d "$SPEC_DIR" ]] && errors+=("Spec directory not found: $SPEC_DIR")
    [[ ! -f "$MAIN_SPEC" ]] && errors+=("Main spec not found: $MAIN_SPEC")
    [[ ! -d "$CHAPTERS_DIR" ]] && errors+=("Chapters directory not found: $CHAPTERS_DIR")

    if [[ ${#errors[@]} -gt 0 ]]; then
        if $JSON_MODE; then
            echo '{"status":"error","errors":'"$(printf '%s\n' "${errors[@]}" | jq -R . | jq -s .)"'}'
        else
            for err in "${errors[@]}"; do
                echo "ERROR: $err" >&2
            done
        fi
        exit 2
    fi
}

# ============================================================================
# ANALYSIS FUNCTIONS
# ============================================================================

# Extract spec version from main spec file
get_spec_version() {
    grep -oP '(?<=Version:\s)[0-9]+\.[0-9]+\.[0-9]+' "$MAIN_SPEC" 2>/dev/null || echo "unknown"
}

# Count annotation types documented in spec
count_annotation_types() {
    local file="$1"
    grep -c '@acp:' "$file" 2>/dev/null || echo "0"
}

# Check if annotation has directive documented
check_directive_documented() {
    local file="$1"
    local annotation="$2"
    grep -q "@acp:$annotation.*-" "$file" 2>/dev/null
}

# Count examples in file
count_examples() {
    local file="$1"
    grep -c '```' "$file" 2>/dev/null | awk '{print int($1/2)}'
}

# Count examples with directives
count_examples_with_directives() {
    local file="$1"
    # Look for code blocks containing @acp: with directive suffix
    grep -A5 '```' "$file" 2>/dev/null | grep -c '@acp:.*-' || echo "0"
}

# Check for directive syntax documentation
check_directive_syntax() {
    local file="$1"
    grep -q "space-dash-space\|' - '\| - " "$file" 2>/dev/null
}

# Check for hierarchical annotation documentation
check_hierarchical_docs() {
    local file="$1"
    local found=0
    grep -qi "file-level" "$file" 2>/dev/null && ((found++)) || true
    grep -qi "symbol-level" "$file" 2>/dev/null && ((found++)) || true
    grep -qi "inline" "$file" 2>/dev/null && ((found++)) || true
    echo "$found"
}

# Check cache schema for directive field
check_cache_directive_field() {
    if [[ -f "$CACHE_SCHEMA" ]]; then
        grep -q '"directive"' "$CACHE_SCHEMA" 2>/dev/null && echo "present" || echo "missing"
    else
        echo "schema_not_found"
    fi
}

# ============================================================================
# FINDINGS COLLECTION
# ============================================================================

declare -a BLOCKERS=()
declare -a VIOLATIONS=()
declare -a WARNINGS=()
declare -a INFO=()
declare -a QUESTIONS=()

add_blocker() {
    BLOCKERS+=("$1|$2|$3")  # id|location|message
}

add_violation() {
    VIOLATIONS+=("$1|$2|$3")
}

add_warning() {
    WARNINGS+=("$1|$2|$3")
}

add_info() {
    INFO+=("$1|$2|$3")
}

add_question() {
    QUESTIONS+=("$1|$2|$3|$4")  # id|question|options|impact
}

# ============================================================================
# AUDIT FUNCTIONS
# ============================================================================

audit_chapter_02() {
    local file="$CHAPTERS_DIR/02-annotation-syntax.md"
    [[ ! -f "$file" ]] && file="$CHAPTERS_DIR/05-annotations.md"

    if [[ ! -f "$file" ]]; then
        add_blocker "B001" "Chapter 2/5" "Annotation syntax chapter not found"
        return
    fi

    # Check directive syntax documentation
    if ! check_directive_syntax "$file"; then
        add_blocker "B002" "$file" "Directive suffix syntax not documented (RFC-001 Section 1.1)"
    fi

    # Check multi-line directive format
    if ! grep -qi "multi-line" "$file" 2>/dev/null; then
        add_violation "V001" "$file" "Multi-line directive format not documented (RFC-001 Section 1.2)"
    fi

    # Check standard directive table
    if ! grep -q "Recommended Directive" "$file" 2>/dev/null; then
        add_blocker "B003" "$file" "Standard directive language table missing (RFC-001 Section 1.3)"
    fi

    # Check hierarchical annotation levels
    local hier_count=$(check_hierarchical_docs "$file")
    if [[ $hier_count -lt 3 ]]; then
        add_violation "V002" "$file" "Hierarchical annotation levels incomplete ($hier_count/3 documented)"
    fi

    # Check example coverage
    local total_examples=$(count_examples "$file")
    local examples_with_directives=$(count_examples_with_directives "$file")
    if [[ $total_examples -gt 0 && $examples_with_directives -lt $((total_examples / 2)) ]]; then
        add_warning "W001" "$file" "Less than 50% of examples include directive suffixes ($examples_with_directives/$total_examples)"
    fi
}

audit_chapter_03() {
    local file="$CHAPTERS_DIR/03-cache-format.md"

    if [[ ! -f "$file" ]]; then
        add_blocker "B004" "Chapter 3" "Cache format chapter not found"
        return
    fi

    # Check directive field documentation
    if ! grep -q '"directive"' "$file" 2>/dev/null; then
        add_blocker "B005" "$file" "Cache directive field not documented (RFC-001 Section 4.1)"
    fi

    # Check purpose field
    if ! grep -q '"purpose"' "$file" 2>/dev/null; then
        add_violation "V003" "$file" "Cache purpose field not documented (RFC-001 Section 5.8)"
    fi

    # Check symbols array
    if ! grep -q '"symbols"' "$file" 2>/dev/null; then
        add_violation "V004" "$file" "Symbol-level caching not documented (RFC-001 Section 5.8)"
    fi

    # Check inline array
    if ! grep -q '"inline"' "$file" 2>/dev/null; then
        add_violation "V005" "$file" "Inline annotation caching not documented (RFC-001 Section 5.8)"
    fi
}

audit_chapter_constraints() {
    local file="$CHAPTERS_DIR/06-constraints.md"
    [[ ! -f "$file" ]] && file="$CHAPTERS_DIR/05-constraints.md"

    if [[ ! -f "$file" ]]; then
        add_warning "W002" "Chapter 5/6" "Constraints chapter not found"
        return
    fi

    # Check directive in constraint output
    if ! grep -q "directive" "$file" 2>/dev/null; then
        add_violation "V006" "$file" "Constraint output missing directive display (RFC-001 Section 4.2)"
    fi
}

audit_bootstrap_section() {
    local found=false

    for chapter in "$CHAPTERS_DIR"/*.md; do
        if grep -qi "bootstrap" "$chapter" 2>/dev/null; then
            found=true

            # Check minimal bootstrap
            if ! grep -q "minimal bootstrap\|~40 tokens" "$chapter" 2>/dev/null; then
                add_violation "V007" "$chapter" "Minimal bootstrap prompt not documented (RFC-001 Section 2)"
            fi
            break
        fi
    done

    if ! $found; then
        add_info "I001" "spec/chapters/" "No bootstrap documentation found - may be in main spec"
    fi
}

audit_new_annotation_types() {
    # RFC-001 required new annotations
    local new_annotations=(
        "purpose"
        "fn"
        "class"
        "method"
        "param"
        "returns"
        "throws"
        "critical"
        "todo"
        "fixme"
        "perf"
    )

    local missing=()

    for annotation in "${new_annotations[@]}"; do
        local found=false
        for file in "$CHAPTERS_DIR"/*.md "$MAIN_SPEC"; do
            if grep -q "@acp:$annotation" "$file" 2>/dev/null; then
                found=true
                break
            fi
        done

        if ! $found; then
            missing+=("@acp:$annotation")
        fi
    done

    if [[ ${#missing[@]} -gt 0 ]]; then
        add_violation "V008" "spec/" "Missing new annotation types: ${missing[*]} (RFC-001 Implementation Checklist)"
    fi
}

audit_cache_schema() {
    local status=$(check_cache_directive_field)

    case "$status" in
        "present")
            add_info "I002" "$CACHE_SCHEMA" "Cache schema includes directive field"
            ;;
        "missing")
            add_blocker "B006" "$CACHE_SCHEMA" "Cache schema missing directive field (RFC-001 Section 4.1)"
            ;;
        "schema_not_found")
            add_warning "W003" "schemas/v1/" "Cache schema file not found"
            ;;
    esac
}

collect_open_questions() {
    add_question "Q01" "Should directive suffix be strictly required?" "A:Required|B:Optional" "high"
    add_question "Q02" "Maximum directive length?" "A:No limit|B:500 soft|C:500 hard" "medium"
    add_question "Q03" "Conflicting directive resolution?" "A:File wins|B:Most restrictive|C:Most specific" "high"
    add_question "Q04" "Auto-generated directive format?" "A:Generic|B:Full|C:Empty" "medium"
    add_question "Q05" "Directive localization?" "A:No|B:Optional tag|C:Full i18n" "low"
    add_question "Q06" "Symbol annotation inheritance?" "A:No|B:Direct children|C:All descendants" "medium"
    add_question "Q07" "CLI warning behavior?" "A:Silent|B:Summary|C:Per-annotation|D:First-N" "low"
    add_question "Q08" "Cache storage of generated directives?" "A:Both|B:Generated only|C:Flag" "medium"
}

# ============================================================================
# OUTPUT FUNCTIONS
# ============================================================================

output_json() {
    local spec_version=$(get_spec_version)
    local status="clean"
    [[ ${#BLOCKERS[@]} -gt 0 ]] && status="blockers"
    [[ ${#VIOLATIONS[@]} -gt 0 && "$status" == "clean" ]] && status="findings"

    cat << EOF
{
  "status": "$status",
  "spec_version": "$spec_version",
  "rfc_version": "001",
  "files_audited": $(find "$SPEC_DIR" -name "*.md" -type f 2>/dev/null | jq -R . | jq -s .),
  "findings": {
    "blockers": $(printf '%s\n' "${BLOCKERS[@]:-}" | awk -F'|' '{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
    "violations": $(printf '%s\n' "${VIOLATIONS[@]:-}" | awk -F'|' '{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
    "warnings": $(printf '%s\n' "${WARNINGS[@]:-}" | awk -F'|' '{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
    "info": $(printf '%s\n' "${INFO[@]:-}" | awk -F'|' '{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]")
  },
  "questions": $(printf '%s\n' "${QUESTIONS[@]:-}" | awk -F'|' '{print "{\"id\":\""$1"\",\"question\":\""$2"\",\"options\":\""$3"\",\"impact\":\""$4"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
  "coverage": {
    "blockers_count": ${#BLOCKERS[@]},
    "violations_count": ${#VIOLATIONS[@]},
    "warnings_count": ${#WARNINGS[@]},
    "info_count": ${#INFO[@]}
  }
}
EOF
}

output_text() {
    echo "═══════════════════════════════════════════════════════════════════"
    echo "  ACP Spec Directive Audit Report"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    echo "Spec Version: $(get_spec_version)"
    echo "RFC Reference: RFC-001 (Self-Documenting Annotations)"
    echo ""

    # Summary
    echo "┌─────────────────────────────────────────────────────────────────┐"
    echo "│ SUMMARY                                                         │"
    echo "├─────────────────────────────────────────────────────────────────┤"
    printf "│ Blockers:    %-3d   (MUST fix before release)                  │\n" ${#BLOCKERS[@]}
    printf "│ Violations:  %-3d   (SHOULD fix)                               │\n" ${#VIOLATIONS[@]}
    printf "│ Warnings:    %-3d   (MAY fix)                                  │\n" ${#WARNINGS[@]}
    printf "│ Info:        %-3d   (Observations)                             │\n" ${#INFO[@]}
    echo "└─────────────────────────────────────────────────────────────────┘"
    echo ""

    # Blockers
    if [[ ${#BLOCKERS[@]} -gt 0 ]]; then
        echo "BLOCKERS (Must Fix):"
        echo "────────────────────"
        for finding in "${BLOCKERS[@]}"; do
            IFS='|' read -r id location message <<< "$finding"
            echo "  [$id] $location"
            echo "        $message"
        done
        echo ""
    fi

    # Violations
    if [[ ${#VIOLATIONS[@]} -gt 0 ]]; then
        echo "VIOLATIONS (Should Fix):"
        echo "────────────────────────"
        for finding in "${VIOLATIONS[@]}"; do
            IFS='|' read -r id location message <<< "$finding"
            echo "  [$id] $location"
            echo "        $message"
        done
        echo ""
    fi

    # Warnings
    if [[ ${#WARNINGS[@]} -gt 0 ]]; then
        echo "WARNINGS (May Fix):"
        echo "───────────────────"
        for finding in "${WARNINGS[@]}"; do
            IFS='|' read -r id location message <<< "$finding"
            echo "  [$id] $location"
            echo "        $message"
        done
        echo ""
    fi

    # Questions
    if [[ ${#QUESTIONS[@]} -gt 0 ]] && ! $QUESTIONS_ONLY; then
        echo "OPEN QUESTIONS (Need Decision):"
        echo "───────────────────────────────"
        for q in "${QUESTIONS[@]}"; do
            IFS='|' read -r id question options impact <<< "$q"
            echo "  [$id] $question"
            echo "        Options: $options"
            echo "        Impact: $impact"
        done
        echo ""
    fi

    echo "═══════════════════════════════════════════════════════════════════"
}

output_questions_only() {
    if $JSON_MODE; then
        printf '%s\n' "${QUESTIONS[@]:-}" | \
            awk -F'|' '{print "{\"id\":\""$1"\",\"question\":\""$2"\",\"options\":\""$3"\",\"impact\":\""$4"\"}"}' | \
            jq -s '{questions: .}'
    else
        echo "Open Questions Requiring Decision:"
        echo "═══════════════════════════════════"
        for q in "${QUESTIONS[@]}"; do
            IFS='|' read -r id question options impact <<< "$q"
            echo ""
            echo "[$id] $question"
            echo "     Options: $options"
            echo "     Impact: $impact"
        done
    fi
}

# ============================================================================
# MAIN
# ============================================================================

main() {
    validate_paths

    # Run audits based on options
    if ! $CHAPTERS_ONLY; then
        # Audit main spec (minimal checks)
        if [[ -f "$MAIN_SPEC" ]]; then
            local main_examples=$(count_examples "$MAIN_SPEC")
            local main_directives=$(count_examples_with_directives "$MAIN_SPEC")
            if [[ $main_examples -gt 5 && $main_directives -lt $((main_examples / 3)) ]]; then
                add_warning "W004" "$MAIN_SPEC" "Main spec examples lack directives ($main_directives/$main_examples)"
            fi
        fi
    fi

    if ! $MAIN_ONLY; then
        audit_chapter_02
        audit_chapter_03
        audit_chapter_constraints
        audit_bootstrap_section
        audit_new_annotation_types
    fi

    audit_cache_schema
    collect_open_questions

    # Save findings to file
    output_json > "$OUTPUT_DIR/spec-audit-findings.json"

    # Output results
    if $QUESTIONS_ONLY; then
        output_questions_only
    elif $JSON_MODE; then
        output_json
    else
        output_text
    fi

    # Exit code
    if [[ ${#BLOCKERS[@]} -gt 0 ]]; then
        exit 1
    elif $STRICT_MODE && [[ ${#VIOLATIONS[@]} -gt 0 ]]; then
        exit 1
    fi

    exit 0
}

main "$@"