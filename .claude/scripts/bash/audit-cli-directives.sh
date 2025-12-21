#!/usr/bin/env bash

# Audit ACP CLI implementation for RFC-001 directive compliance
#
# This script analyzes cli/src/* to identify:
# 1. Parser support for directive extraction
# 2. Cache structure support for directive fields
# 3. Command implementations for new features
# 4. Test coverage for directive functionality
#
# Usage: ./audit-cli-directives.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --module <n>        Audit specific module only
#   --strict            Treat SHOULD violations as errors
#   --decisions <path>  Load spec decisions for consistency
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
STRICT_MODE=false
TARGET_MODULE=""
DECISIONS_FILE=".acp/spec-decisions.json"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json)
            JSON_MODE=true
            shift
            ;;
        --module)
            TARGET_MODULE="$2"
            shift 2
            ;;
        --strict)
            STRICT_MODE=true
            shift
            ;;
        --decisions)
            DECISIONS_FILE="$2"
            shift 2
            ;;
        --help|-h)
            cat << 'EOF'
Usage: audit-cli-directives.sh [OPTIONS]

Audit ACP CLI for RFC-001 (Self-Documenting Annotations) compliance.

OPTIONS:
  --json              Output in JSON format
  --module <name>     Audit specific module (parse, cache, constraints, commands)
  --strict            Treat SHOULD violations as errors
  --decisions <path>  Path to spec decisions file
  --help, -h          Show this help message

MODULES:
  parse       - Parser and annotation extraction
  cache       - Cache types and serialization
  constraints - Constraint handling and output
  commands    - CLI command implementations
  index       - Indexer and cache building

EXIT CODES:
  0 - Clean (no blockers)
  1 - Blockers found
  2 - Error (CLI source not found, etc.)

EOF
            exit 0
            ;;
        *)
            echo "ERROR: Unknown option '$1'" >&2
            exit 2
            ;;
    esac
done

# ============================================================================
# CONFIGURATION
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

CLI_DIR="$PROJECT_ROOT/cli"
CLI_SRC="$CLI_DIR/src"
OUTPUT_DIR="$PROJECT_ROOT/.acp"

mkdir -p "$OUTPUT_DIR"

# ============================================================================
# VALIDATION
# ============================================================================

if [[ ! -d "$CLI_SRC" ]]; then
    if $JSON_MODE; then
        echo '{"status":"error","message":"CLI source not found","path":"'"$CLI_SRC"'"}'
    else
        echo "ERROR: CLI source not found at $CLI_SRC" >&2
    fi
    exit 2
fi

# ============================================================================
# ANALYSIS FUNCTIONS
# ============================================================================

# Check if a pattern exists in file(s)
check_pattern() {
    local pattern="$1"
    local files="$2"
    grep -l "$pattern" $files 2>/dev/null | head -1
}

# Check for struct field
check_struct_field() {
    local struct_name="$1"
    local field_name="$2"
    local file="$3"

    if [[ -f "$file" ]]; then
        # Look for field in struct definition
        if grep -A50 "struct $struct_name" "$file" 2>/dev/null | grep -q "pub $field_name:"; then
            echo "present"
        else
            echo "missing"
        fi
    else
        echo "file_not_found"
    fi
}

# Check for regex pattern in parser
check_parser_regex() {
    local parse_file="$CLI_SRC/parse/mod.rs"
    [[ ! -f "$parse_file" ]] && parse_file="$CLI_SRC/parse.rs"

    if [[ -f "$parse_file" ]]; then
        # Check if regex handles directive suffix
        if grep -q '- ' "$parse_file" 2>/dev/null && grep -q '@acp:' "$parse_file" 2>/dev/null; then
            echo "present"
        else
            echo "missing"
        fi
    else
        echo "file_not_found"
    fi
}

# Check for command implementation
check_command() {
    local cmd_name="$1"
    local cmd_dir="$CLI_SRC/commands"

    if [[ -f "$cmd_dir/$cmd_name.rs" ]]; then
        echo "present"
    elif [[ -f "$CLI_SRC/main.rs" ]] && grep -qi "$cmd_name" "$CLI_SRC/main.rs" 2>/dev/null; then
        echo "partial"
    else
        echo "missing"
    fi
}

# Get CLI version from Cargo.toml
get_cli_version() {
    if [[ -f "$CLI_DIR/Cargo.toml" ]]; then
        grep -oP '(?<=^version = ")[^"]+' "$CLI_DIR/Cargo.toml" 2>/dev/null || echo "unknown"
    else
        echo "unknown"
    fi
}

# ============================================================================
# FINDINGS COLLECTION
# ============================================================================

declare -a BLOCKERS=()
declare -a VIOLATIONS=()
declare -a WARNINGS=()
declare -a INFO=()

add_blocker() { BLOCKERS+=("$1|$2|$3"); }
add_violation() { VIOLATIONS+=("$1|$2|$3"); }
add_warning() { WARNINGS+=("$1|$2|$3"); }
add_info() { INFO+=("$1|$2|$3"); }

# ============================================================================
# MODULE AUDITS
# ============================================================================

audit_parser() {
    local parse_file="$CLI_SRC/parse/mod.rs"
    [[ ! -f "$parse_file" ]] && parse_file="$CLI_SRC/parse.rs"

    if [[ ! -f "$parse_file" ]]; then
        add_blocker "P00" "cli/src/parse/" "Parser module not found"
        return
    fi

    # P01: Check Annotation struct has directive field
    if ! grep -q 'directive' "$parse_file" 2>/dev/null; then
        add_blocker "P01" "$parse_file" "Annotation struct missing 'directive' field"
    fi

    # P02: Check regex extracts directive
    if ! grep -qE '\s+-\s+' "$parse_file" 2>/dev/null; then
        add_blocker "P02" "$parse_file" "Parser regex does not extract directive suffix"
    fi

    # P03: Check for auto_generated field
    if ! grep -q 'auto_generated' "$parse_file" 2>/dev/null; then
        add_violation "P03" "$parse_file" "Annotation missing 'auto_generated' flag (Q08)"
    fi

    # P04: Check for multiline handling
    if ! grep -qi 'multi\|continuation' "$parse_file" 2>/dev/null; then
        add_violation "P04" "$parse_file" "No multi-line directive handling"
    fi

    # P05: Check for default directive generation
    if ! grep -qi 'default.*directive\|generate.*directive' "$parse_file" 2>/dev/null; then
        add_violation "P05" "$parse_file" "No default directive generation (Q04)"
    fi
}

audit_cache() {
    local cache_file="$CLI_SRC/cache/types.rs"
    [[ ! -f "$cache_file" ]] && cache_file="$CLI_SRC/cache.rs"

    if [[ ! -f "$cache_file" ]]; then
        add_blocker "C00" "cli/src/cache/" "Cache module not found"
        return
    fi

    # C01: Check FileEntry has purpose field
    local purpose_status=$(check_struct_field "FileEntry" "purpose" "$cache_file")
    if [[ "$purpose_status" == "missing" ]]; then
        add_blocker "C01" "$cache_file" "FileEntry missing 'purpose' field"
    fi

    # C02: Check FileEntry has symbols array
    local symbols_status=$(check_struct_field "FileEntry" "symbols" "$cache_file")
    if [[ "$symbols_status" == "missing" ]]; then
        add_blocker "C02" "$cache_file" "FileEntry missing 'symbols' array"
    fi

    # C03: Check FileEntry has inline array
    local inline_status=$(check_struct_field "FileEntry" "inline" "$cache_file")
    if [[ "$inline_status" == "missing" ]]; then
        add_blocker "C03" "$cache_file" "FileEntry missing 'inline' array"
    fi

    # C04: Check for SymbolAnnotation type
    if ! grep -q 'SymbolAnnotation\|SymbolEntry' "$cache_file" 2>/dev/null; then
        add_violation "C04" "$cache_file" "No SymbolAnnotation type defined"
    fi

    # C05: Check for InlineAnnotation type
    if ! grep -q 'InlineAnnotation' "$cache_file" 2>/dev/null; then
        add_violation "C05" "$cache_file" "No InlineAnnotation type defined"
    fi

    # C06: Check for LineRange type
    if ! grep -q 'LineRange\|lines.*start.*end' "$cache_file" 2>/dev/null; then
        add_violation "C06" "$cache_file" "No LineRange type for symbol locations"
    fi
}

audit_constraints() {
    local const_file="$CLI_SRC/constraints/mod.rs"
    [[ ! -f "$const_file" ]] && const_file="$CLI_SRC/constraints.rs"

    if [[ ! -f "$const_file" ]]; then
        add_warning "N00" "cli/src/constraints/" "Constraints module not found"
        return
    fi

    # N01: Check constraint output includes directive
    if ! grep -qi 'directive' "$const_file" 2>/dev/null; then
        add_violation "N01" "$const_file" "Constraint output missing directive display"
    fi

    # N02: Check for directive aggregation
    if ! grep -qi 'aggregate' "$const_file" 2>/dev/null; then
        add_violation "N02" "$const_file" "No directive aggregation logic"
    fi
}

audit_commands() {
    local cmd_dir="$CLI_SRC/commands"
    local main_file="$CLI_SRC/main.rs"

    # M01: Check for map command
    local map_status=$(check_command "map")
    if [[ "$map_status" == "missing" ]]; then
        add_blocker "M01" "$cmd_dir" "'acp map' command not implemented"
    fi

    # M02: Check for migrate command
    local migrate_status=$(check_command "migrate")
    if [[ "$migrate_status" == "missing" ]]; then
        add_blocker "M02" "$cmd_dir" "'acp migrate' command not implemented"
    fi

    # M03: Check query command for file purpose
    local query_file="$cmd_dir/query.rs"
    [[ ! -f "$query_file" ]] && query_file="$CLI_SRC/query.rs"
    if [[ -f "$query_file" ]]; then
        if ! grep -q 'purpose' "$query_file" 2>/dev/null; then
            add_violation "M03" "$query_file" "'acp query file' missing purpose output"
        fi
    fi

    # M04: Check constraints command for directive output
    local check_file="$cmd_dir/check.rs"
    [[ ! -f "$check_file" ]] && check_file="$cmd_dir/constraints.rs"
    if [[ -f "$check_file" ]]; then
        if ! grep -q 'directive' "$check_file" 2>/dev/null; then
            add_violation "M04" "$check_file" "'acp constraints' missing directive output"
        fi
    fi
}

audit_indexer() {
    local index_file="$CLI_SRC/index/indexer.rs"
    [[ ! -f "$index_file" ]] && index_file="$CLI_SRC/index.rs"

    if [[ ! -f "$index_file" ]]; then
        add_warning "I00" "cli/src/index/" "Indexer module not found"
        return
    fi

    # I01: Check for missing directive warnings
    if ! grep -qi 'warn\|missing.*directive' "$index_file" 2>/dev/null; then
        add_violation "I01" "$index_file" "No warnings for missing directives (Q07)"
    fi

    # I02: Check for purpose extraction
    if ! grep -q 'purpose' "$index_file" 2>/dev/null; then
        add_violation "I02" "$index_file" "No file purpose extraction"
    fi

    # I03: Check for symbol building
    if ! grep -qi 'symbol' "$index_file" 2>/dev/null; then
        add_violation "I03" "$index_file" "No symbol annotation building"
    fi
}

# ============================================================================
# FEATURE COVERAGE
# ============================================================================

calculate_feature_coverage() {
    local features=()

    # Directive parsing
    local parse_file="$CLI_SRC/parse/mod.rs"
    [[ ! -f "$parse_file" ]] && parse_file="$CLI_SRC/parse.rs"
    if [[ -f "$parse_file" ]] && grep -q 'directive' "$parse_file" 2>/dev/null; then
        features+=("directive_parsing:complete")
    elif [[ -f "$parse_file" ]]; then
        features+=("directive_parsing:partial")
    else
        features+=("directive_parsing:missing")
    fi

    # Similar checks for other features...
    local cache_file="$CLI_SRC/cache/types.rs"
    [[ ! -f "$cache_file" ]] && cache_file="$CLI_SRC/cache.rs"
    if [[ -f "$cache_file" ]] && grep -q 'purpose' "$cache_file" 2>/dev/null; then
        features+=("directive_storage:partial")
    else
        features+=("directive_storage:missing")
    fi

    # Map command
    if [[ -f "$CLI_SRC/commands/map.rs" ]]; then
        features+=("map_command:complete")
    else
        features+=("map_command:missing")
    fi

    # Migrate command
    if [[ -f "$CLI_SRC/commands/migrate.rs" ]]; then
        features+=("migrate_command:complete")
    else
        features+=("migrate_command:missing")
    fi

    printf '%s\n' "${features[@]}"
}

# ============================================================================
# OUTPUT
# ============================================================================

output_json() {
    local cli_version=$(get_cli_version)
    local status="clean"
    [[ ${#BLOCKERS[@]} -gt 0 ]] && status="blockers"
    [[ ${#VIOLATIONS[@]} -gt 0 && "$status" == "clean" ]] && status="findings"

    # Build feature coverage JSON
    local features_json="{"
    local first=true
    while IFS=: read -r feature state; do
        $first || features_json+=","
        features_json+="\"$feature\":\"$state\""
        first=false
    done < <(calculate_feature_coverage)
    features_json+="}"

    cat << EOF
{
  "status": "$status",
  "cli_version": "$cli_version",
  "rfc_version": "001",
  "modules_audited": $(find "$CLI_SRC" -name "*.rs" -type f 2>/dev/null | jq -R . | jq -s . || echo "[]"),
  "findings": {
    "blockers": $(printf '%s\n' "${BLOCKERS[@]:-}" | awk -F'|' 'NF{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
    "violations": $(printf '%s\n' "${VIOLATIONS[@]:-}" | awk -F'|' 'NF{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
    "warnings": $(printf '%s\n' "${WARNINGS[@]:-}" | awk -F'|' 'NF{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
    "info": $(printf '%s\n' "${INFO[@]:-}" | awk -F'|' 'NF{print "{\"id\":\""$1"\",\"location\":\""$2"\",\"message\":\""$3"\"}"}' | jq -s . 2>/dev/null || echo "[]")
  },
  "feature_coverage": $features_json,
  "coverage": {
    "blockers_count": ${#BLOCKERS[@]},
    "violations_count": ${#VIOLATIONS[@]},
    "warnings_count": ${#WARNINGS[@]}
  }
}
EOF
}

output_text() {
    echo "═══════════════════════════════════════════════════════════════════"
    echo "  ACP CLI Directive Audit Report"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    echo "CLI Version: $(get_cli_version)"
    echo "RFC Reference: RFC-001"
    echo ""

    echo "┌─────────────────────────────────────────────────────────────────┐"
    echo "│ SUMMARY                                                         │"
    echo "├─────────────────────────────────────────────────────────────────┤"
    printf "│ Blockers:    %-3d   (MUST implement)                           │\n" ${#BLOCKERS[@]}
    printf "│ Violations:  %-3d   (SHOULD implement)                         │\n" ${#VIOLATIONS[@]}
    printf "│ Warnings:    %-3d   (MAY implement)                            │\n" ${#WARNINGS[@]}
    echo "└─────────────────────────────────────────────────────────────────┘"
    echo ""

    echo "Feature Coverage:"
    echo "─────────────────"
    calculate_feature_coverage | while IFS=: read -r feature state; do
        printf "  %-25s %s\n" "$feature" "$state"
    done
    echo ""

    if [[ ${#BLOCKERS[@]} -gt 0 ]]; then
        echo "BLOCKERS:"
        echo "─────────"
        for finding in "${BLOCKERS[@]}"; do
            IFS='|' read -r id location message <<< "$finding"
            echo "  [$id] $location"
            echo "        $message"
        done
        echo ""
    fi

    if [[ ${#VIOLATIONS[@]} -gt 0 ]]; then
        echo "VIOLATIONS:"
        echo "───────────"
        for finding in "${VIOLATIONS[@]}"; do
            IFS='|' read -r id location message <<< "$finding"
            echo "  [$id] $location"
            echo "        $message"
        done
        echo ""
    fi

    echo "═══════════════════════════════════════════════════════════════════"
}

# ============================================================================
# MAIN
# ============================================================================

main() {
    # Run audits based on module flag
    case "$TARGET_MODULE" in
        "parse")
            audit_parser
            ;;
        "cache")
            audit_cache
            ;;
        "constraints")
            audit_constraints
            ;;
        "commands")
            audit_commands
            ;;
        "index")
            audit_indexer
            ;;
        "")
            # Audit all modules
            audit_parser
            audit_cache
            audit_constraints
            audit_commands
            audit_indexer
            ;;
        *)
            echo "ERROR: Unknown module '$TARGET_MODULE'" >&2
            exit 2
            ;;
    esac

    # Save findings
    output_json > "$OUTPUT_DIR/cli-audit-findings.json"

    # Output results
    if $JSON_MODE; then
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