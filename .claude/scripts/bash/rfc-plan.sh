#!/usr/bin/env bash

# RFC Planning Script
#
# Generates an implementation plan structure from an accepted RFC.
# Used by /rfc.refine command.
#
# Usage: ./rfc-plan.sh [OPTIONS] <rfc-path>
#
# OPTIONS:
#   --json              Output in JSON format
#   --analysis <path>   Path to analysis report
#   --detailed          Generate detailed task breakdown
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
DETAILED_MODE=false
RFC_PATH=""
ANALYSIS_PATH=""

for arg in "$@"; do
    case "$arg" in
        --json)
            JSON_MODE=true
            ;;
        --detailed)
            DETAILED_MODE=true
            ;;
        --analysis)
            shift_next=true
            ;;
        --help|-h)
            cat << 'EOF'
Usage: rfc-plan.sh [OPTIONS] <rfc-path>

Generate implementation plan from an accepted RFC.

OPTIONS:
  --json              Output in JSON format
  --analysis <path>   Path to analysis report
  --detailed          Generate detailed task breakdown
  --help, -h          Show this help message

ARGUMENTS:
  <rfc-path>          Path to RFC file

OUTPUTS:
  - Components affected
  - Task breakdown by phase
  - Dependency analysis
  - Effort estimates

EXAMPLES:
  # Generate plan with JSON output
  ./rfc-plan.sh --json rfcs/accepted/0001-feature.md
  
  # With analysis report
  ./rfc-plan.sh --json --analysis .claude/memory/rfc-analysis-001.md rfcs/accepted/0001-feature.md

EOF
            exit 0
            ;;
        --*)
            if [[ "$shift_next" == true ]]; then
                ANALYSIS_PATH="$arg"
                shift_next=false
            else
                echo "ERROR: Unknown option '$arg'. Use --help for usage." >&2
                exit 1
            fi
            ;;
        *)
            if [[ "$shift_next" == true ]]; then
                ANALYSIS_PATH="$arg"
                shift_next=false
            elif [[ -z "$RFC_PATH" ]]; then
                RFC_PATH="$arg"
            fi
            ;;
    esac
done

# ============================================================================
# VALIDATION
# ============================================================================

if [[ -z "$RFC_PATH" ]]; then
    echo "ERROR: No RFC path provided. Use --help for usage." >&2
    exit 1
fi

if [[ ! -f "$RFC_PATH" ]]; then
    if $JSON_MODE; then
        echo '{"error":"RFC file not found","path":"'"$RFC_PATH"'"}'
    else
        echo "ERROR: RFC file not found: $RFC_PATH" >&2
    fi
    exit 1
fi

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

# Extract RFC ID from file
get_rfc_id() {
    local file="$1"
    local id=$(grep -E "^-?\s*\*?\*?RFC\s*(ID|Number).*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    [[ -z "$id" ]] && id=$(basename "$file" .md | sed 's/^[0-9]*-/RFC-/')
    echo "$id"
}

# Extract title
get_rfc_title() {
    local file="$1"
    local title=$(grep -E "^-?\s*\*?\*?Title.*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    [[ -z "$title" ]] && title=$(head -1 "$file" | sed 's/^#\s*//')
    echo "$title"
}

# Detect affected components
detect_components() {
    local file="$1"
    local components=()
    local spec_chapters=()
    local schema_files=()
    local cli_modules=()
    local doc_files=()
    
    # Spec chapters
    if grep -qi "annotation\|syntax" "$file"; then
        spec_chapters+=("02-annotation-syntax.md")
    fi
    if grep -qi "cache\|index" "$file"; then
        spec_chapters+=("03-cache-format.md")
    fi
    if grep -qi "constraint\|lock" "$file"; then
        spec_chapters+=("05-constraints.md")
    fi
    if grep -qi "variable" "$file"; then
        spec_chapters+=("04-variables.md")
    fi
    if grep -qi "query\|search" "$file"; then
        spec_chapters+=("06-query-interface.md")
    fi
    
    # Schema files
    if grep -qi "cache.*schema\|cache.*format" "$file"; then
        schema_files+=("cache.schema.json")
    fi
    if grep -qi "config.*schema\|configuration" "$file"; then
        schema_files+=("config.schema.json")
    fi
    if grep -qi "annotation.*schema" "$file"; then
        schema_files+=("annotation.schema.json")
    fi
    
    # CLI modules
    if grep -qi "parser\|parse" "$file"; then
        cli_modules+=("parse/annotations.rs")
    fi
    if grep -qi "index\|indexer" "$file"; then
        cli_modules+=("commands/index.rs")
    fi
    if grep -qi "query" "$file"; then
        cli_modules+=("commands/query.rs")
    fi
    if grep -qi "constraint" "$file"; then
        cli_modules+=("commands/constraints.rs")
    fi
    
    # Documentation
    if grep -qi "annotation" "$file"; then
        doc_files+=("annotation-reference.md")
    fi
    if grep -qi "cli\|command" "$file"; then
        doc_files+=("cli-reference.md")
    fi
    
    echo "spec:${spec_chapters[*]}"
    echo "schemas:${schema_files[*]}"
    echo "cli:${cli_modules[*]}"
    echo "docs:${doc_files[*]}"
}

# Analyze dependencies
analyze_dependencies() {
    local file="$1"
    local external=()
    local internal=()
    local blocking=()
    
    # Check for external dependencies
    if grep -qi "tree-sitter\|treesitter" "$file"; then
        external+=("tree-sitter")
    fi
    if grep -qi "regex\|regexp" "$file"; then
        external+=("regex-library")
    fi
    
    # Check for internal dependencies
    if grep -qi "schema.*first\|spec.*first" "$file"; then
        internal+=("spec changes before implementation")
    fi
    if grep -qi "cache.*first\|format.*first" "$file"; then
        internal+=("cache format before CLI")
    fi
    
    # Default internal dependencies
    internal+=("schema changes before code changes")
    
    echo "external:${external[*]}"
    echo "internal:${internal[*]}"
    echo "blocking:${blocking[*]}"
}

# Estimate effort
estimate_effort() {
    local file="$1"
    
    local spec_hours="1-2"
    local schema_hours="0.5-1"
    local cli_hours="4-8"
    local tests_hours="2-4"
    local docs_hours="1-2"
    
    # Adjust based on complexity signals
    if grep -qi "breaking change\|migration" "$file"; then
        cli_hours="8-12"
        tests_hours="4-6"
    fi
    
    if grep -qi "new command\|new subcommand" "$file"; then
        cli_hours="6-10"
    fi
    
    echo "spec:$spec_hours"
    echo "schemas:$schema_hours"
    echo "cli:$cli_hours"
    echo "tests:$tests_hours"
    echo "docs:$docs_hours"
}

# Check for breaking changes
check_breaking_changes() {
    local file="$1"
    local has_breaking=false
    local migration_required=false
    local strategy=""
    
    if grep -qi "breaking change\|backwards incompatible\|not compatible" "$file"; then
        has_breaking=true
    fi
    
    if grep -qi "migration\|migrate\|upgrade path" "$file"; then
        migration_required=true
        strategy=$(grep -i "migration" "$file" | head -1 | sed 's/.*migration\s*//' | cut -c1-80)
    fi
    
    echo "$has_breaking"
    echo "$migration_required"
    echo "$strategy"
}

# ============================================================================
# MAIN ANALYSIS
# ============================================================================

RFC_ID=$(get_rfc_id "$RFC_PATH")
RFC_TITLE=$(get_rfc_title "$RFC_PATH")

# Detect components
components_output=$(detect_components "$RFC_PATH")
spec_chapters=$(echo "$components_output" | grep "^spec:" | sed 's/^spec://')
schema_files=$(echo "$components_output" | grep "^schemas:" | sed 's/^schemas://')
cli_modules=$(echo "$components_output" | grep "^cli:" | sed 's/^cli://')
doc_files=$(echo "$components_output" | grep "^docs:" | sed 's/^docs://')

# Analyze dependencies
deps_output=$(analyze_dependencies "$RFC_PATH")
external_deps=$(echo "$deps_output" | grep "^external:" | sed 's/^external://')
internal_deps=$(echo "$deps_output" | grep "^internal:" | sed 's/^internal://')
blocking_deps=$(echo "$deps_output" | grep "^blocking:" | sed 's/^blocking://')

# Estimate effort
effort_output=$(estimate_effort "$RFC_PATH")
spec_effort=$(echo "$effort_output" | grep "^spec:" | sed 's/^spec://')
schema_effort=$(echo "$effort_output" | grep "^schemas:" | sed 's/^schemas://')
cli_effort=$(echo "$effort_output" | grep "^cli:" | sed 's/^cli://')
tests_effort=$(echo "$effort_output" | grep "^tests:" | sed 's/^tests://')
docs_effort=$(echo "$effort_output" | grep "^docs:" | sed 's/^docs://')

# Check breaking changes
breaking_output=$(check_breaking_changes "$RFC_PATH")
has_breaking=$(echo "$breaking_output" | sed -n '1p')
migration_required=$(echo "$breaking_output" | sed -n '2p')
migration_strategy=$(echo "$breaking_output" | sed -n '3p')

# Calculate total effort (rough estimate)
total_min=8
total_max=20

# ============================================================================
# OUTPUT
# ============================================================================

if $JSON_MODE; then
    # Helper function
    to_json_array() {
        local input="$1"
        if [[ -z "$input" || "$input" == " " ]]; then
            echo "[]"
        else
            echo "[$(echo "$input" | xargs -n1 | sed 's/.*/"\0"/' | tr '\n' ',' | sed 's/,$//')]"
        fi
    }
    
    cat << JSONEOF
{
  "rfc_id": "$RFC_ID",
  "title": "$RFC_TITLE",
  "components_affected": [
    {"component": "spec", "chapters": $(to_json_array "$spec_chapters")},
    {"component": "schemas", "files": $(to_json_array "$schema_files")},
    {"component": "cli", "modules": $(to_json_array "$cli_modules")},
    {"component": "docs", "files": $(to_json_array "$doc_files")}
  ],
  "dependencies": {
    "external": $(to_json_array "$external_deps"),
    "internal": $(to_json_array "$internal_deps"),
    "blocking": $(to_json_array "$blocking_deps")
  },
  "estimated_effort": {
    "spec": "$spec_effort hours",
    "schemas": "$schema_effort hours",
    "cli": "$cli_effort hours",
    "tests": "$tests_effort hours",
    "docs": "$docs_effort hours",
    "total": "$total_min-$total_max hours"
  },
  "suggested_phases": [
    {"phase": 1, "name": "Foundation", "tasks": ["Update spec chapters", "Update schemas"]},
    {"phase": 2, "name": "Implementation", "tasks": ["Implement parser changes", "Update CLI commands"]},
    {"phase": 3, "name": "Validation", "tasks": ["Write unit tests", "Write integration tests"]},
    {"phase": 4, "name": "Documentation", "tasks": ["Update user docs", "Write migration guide"]}
  ],
  "breaking_changes": {
    "has_breaking": $has_breaking,
    "migration_required": $migration_required,
    "migration_strategy": "$migration_strategy"
  }
}
JSONEOF
else
    echo "========================================"
    echo "RFC Implementation Plan"
    echo "========================================"
    echo ""
    echo "RFC: $RFC_ID - $RFC_TITLE"
    echo ""
    echo "----------------------------------------"
    echo "AFFECTED COMPONENTS"
    echo "----------------------------------------"
    echo "Spec chapters: $spec_chapters"
    echo "Schemas: $schema_files"
    echo "CLI modules: $cli_modules"
    echo "Documentation: $doc_files"
    echo ""
    echo "----------------------------------------"
    echo "DEPENDENCIES"
    echo "----------------------------------------"
    echo "External: $external_deps"
    echo "Internal: $internal_deps"
    echo ""
    echo "----------------------------------------"
    echo "EFFORT ESTIMATES"
    echo "----------------------------------------"
    echo "Spec: $spec_effort hours"
    echo "Schemas: $schema_effort hours"
    echo "CLI: $cli_effort hours"
    echo "Tests: $tests_effort hours"
    echo "Docs: $docs_effort hours"
    echo "Total: $total_min-$total_max hours"
    echo ""
    echo "----------------------------------------"
    echo "BREAKING CHANGES"
    echo "----------------------------------------"
    echo "Has breaking changes: $has_breaking"
    echo "Migration required: $migration_required"
    [[ -n "$migration_strategy" ]] && echo "Strategy: $migration_strategy"
fi

exit 0
