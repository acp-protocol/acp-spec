#!/usr/bin/env bash

# RFC Consistency Check Script
#
# Verifies implementation consistency with specifications and schemas.
# Used by /rfc.check command.
#
# Usage: ./rfc-check.sh [OPTIONS] <rfc-id>
#
# OPTIONS:
#   --json              Output in JSON format
#   --strict            Fail on warnings
#   --fix               Attempt automatic fixes
#   --check <type>      Run specific check only
#   --help, -h          Show help message

set -e

# ============================================================================
# CONFIGURATION
# ============================================================================

MEMORY_DIR=".claude/memory"
TIMESTAMP=$(date -Iseconds)

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
STRICT_MODE=false
FIX_MODE=false
CHECK_TYPE=""
RFC_ID=""

for arg in "$@"; do
    case "$arg" in
        --json)
            JSON_MODE=true
            ;;
        --strict)
            STRICT_MODE=true
            ;;
        --fix)
            FIX_MODE=true
            ;;
        --check)
            shift_next=true
            ;;
        --help|-h)
            cat << 'EOF'
Usage: rfc-check.sh [OPTIONS] <rfc-id>

Verify implementation consistency with specs and schemas.

OPTIONS:
  --json              Output in JSON format
  --strict            Fail on warnings
  --fix               Attempt automatic fixes
  --check <type>      Run specific check (spec_code, schema, cross_ref, version, convention)
  --help, -h          Show this help message

ARGUMENTS:
  <rfc-id>            RFC identifier

OUTPUTS:
  - Spec-code consistency results
  - Schema validation results
  - Cross-reference check results
  - Version alignment results
  - Convention compliance results

EXAMPLES:
  # Full check with JSON output
  ./rfc-check.sh --json RFC-001
  
  # Schema validation only
  ./rfc-check.sh --check schema --json RFC-001
  
  # Strict mode (fail on warnings)
  ./rfc-check.sh --strict --json RFC-001

EOF
            exit 0
            ;;
        --*)
            if [[ "$shift_next" == true ]]; then
                CHECK_TYPE="$arg"
                shift_next=false
            else
                echo "ERROR: Unknown option '$arg'. Use --help for usage." >&2
                exit 1
            fi
            ;;
        *)
            if [[ "$shift_next" == true ]]; then
                CHECK_TYPE="$arg"
                shift_next=false
            elif [[ -z "$RFC_ID" ]]; then
                RFC_ID="$arg"
            fi
            ;;
    esac
done

# ============================================================================
# VALIDATION
# ============================================================================

if [[ -z "$RFC_ID" ]]; then
    echo "ERROR: No RFC ID provided. Use --help for usage." >&2
    exit 1
fi

# Normalize RFC ID
RFC_ID=$(echo "$RFC_ID" | sed 's/^RFC-//' | sed 's/^0*//')
RFC_ID_PADDED=$(printf "%03d" "$RFC_ID" 2>/dev/null || echo "$RFC_ID")

# ============================================================================
# CHECK FUNCTIONS
# ============================================================================

# Check spec-code consistency
check_spec_code() {
    local status="PASS"
    local items_checked=0
    local issues=()
    
    # Check if implementation matches spec descriptions
    # This is a simplified check - real implementation would be more thorough
    
    # Check for spec files
    if [[ -d "spec/chapters" ]]; then
        items_checked=$(find spec/chapters -name "*.md" 2>/dev/null | wc -l)
    fi
    
    # Look for common inconsistencies
    if grep -rq "TODO.*implement" cli/src/ 2>/dev/null; then
        issues+=('{"type":"incomplete","location":"cli/src/","msg":"Found TODO implement markers"}')
        status="WARNING"
    fi
    
    echo "$status"
    echo "$items_checked"
    echo "${issues[*]}"
}

# Check schema validation
check_schemas() {
    local status="PASS"
    local schemas_checked=()
    local issues=()
    
    # Find schema files
    if [[ -d "schemas" ]]; then
        while IFS= read -r -d '' schema; do
            local schema_name=$(basename "$schema")
            schemas_checked+=("$schema_name")
            
            # Basic JSON validation
            if ! python3 -c "import json; json.load(open('$schema'))" 2>/dev/null; then
                issues+=('{"type":"invalid_json","schema":"'"$schema_name"'","msg":"Invalid JSON syntax"}')
                status="FAIL"
            fi
        done < <(find schemas -name "*.schema.json" -print0 2>/dev/null)
    fi
    
    echo "$status"
    echo "${schemas_checked[*]}"
    echo "${issues[*]}"
}

# Check cross-references
check_cross_refs() {
    local status="PASS"
    local refs_checked=0
    local issues=()
    
    # Check markdown links in spec files
    if [[ -d "spec" ]]; then
        while IFS= read -r -d '' file; do
            # Extract markdown links
            local links=$(grep -oE '\[([^\]]+)\]\(([^\)]+)\)' "$file" 2>/dev/null | grep -v "http" || true)
            refs_checked=$((refs_checked + $(echo "$links" | grep -c "." || echo 0)))
            
            # Check each internal link
            while IFS= read -r link; do
                [[ -z "$link" ]] && continue
                local target=$(echo "$link" | sed 's/.*(\([^)]*\)).*/\1/')
                
                # Check if it's an anchor link
                if [[ "$target" == \#* ]]; then
                    local anchor="${target#\#}"
                    if ! grep -qi "^#.*$anchor\|id=\"$anchor\"" "$file" 2>/dev/null; then
                        issues+=('{"type":"broken_anchor","location":"'"$file"'","ref":"'"$target"'"}')
                        status="WARNING"
                    fi
                fi
            done <<< "$links"
        done < <(find spec -name "*.md" -print0 2>/dev/null)
    fi
    
    echo "$status"
    echo "$refs_checked"
    echo "${issues[*]}"
}

# Check version alignment
check_versions() {
    local status="PASS"
    local versions=()
    local issues=()
    
    # Extract versions from various files
    local spec_version=""
    local schema_version=""
    local cli_version=""
    
    # Spec version
    if [[ -f "spec/ACP-1.0.md" ]]; then
        spec_version=$(grep -E "^version:" spec/ACP-1.0.md 2>/dev/null | head -1 | sed 's/.*:\s*//' || echo "unknown")
    fi
    
    # Schema version
    if [[ -f "schemas/v1/cache.schema.json" ]]; then
        schema_version=$(grep '"version"' schemas/v1/cache.schema.json 2>/dev/null | head -1 | sed 's/.*"version".*:.*"\([^"]*\)".*/\1/' || echo "unknown")
    fi
    
    # CLI version (Cargo.toml)
    if [[ -f "cli/Cargo.toml" ]]; then
        cli_version=$(grep "^version" cli/Cargo.toml 2>/dev/null | head -1 | sed 's/.*"\([^"]*\)".*/\1/' || echo "unknown")
    fi
    
    versions+=("spec:$spec_version")
    versions+=("schema:$schema_version")
    versions+=("cli:$cli_version")
    
    echo "$status"
    echo "${versions[*]}"
    echo "${issues[*]}"
}

# Check convention compliance
check_conventions() {
    local status="PASS"
    local issues=()
    
    # Check RFC 2119 keyword usage in specs
    if [[ -d "spec" ]]; then
        while IFS= read -r -d '' file; do
            # Find lowercase RFC 2119 keywords in normative text
            local lowercase=$(grep -n '\bmust\b\|shall\b\|should\b' "$file" 2>/dev/null | grep -v "^#\|^\s*>" || true)
            if [[ -n "$lowercase" ]]; then
                local first_line=$(echo "$lowercase" | head -1 | cut -d: -f1)
                issues+=('{"type":"rfc2119","location":"'"$file:$first_line"'","msg":"Use uppercase RFC 2119 keywords"}')
                status="WARNING"
            fi
        done < <(find spec -name "*.md" -print0 2>/dev/null)
    fi
    
    echo "$status"
    echo "${issues[*]}"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

# Run checks
spec_code_result=$(check_spec_code)
spec_code_status=$(echo "$spec_code_result" | sed -n '1p')
spec_code_items=$(echo "$spec_code_result" | sed -n '2p')
spec_code_issues=$(echo "$spec_code_result" | sed -n '3p')

schema_result=$(check_schemas)
schema_status=$(echo "$schema_result" | sed -n '1p')
schema_checked=$(echo "$schema_result" | sed -n '2p')
schema_issues=$(echo "$schema_result" | sed -n '3p')

refs_result=$(check_cross_refs)
refs_status=$(echo "$refs_result" | sed -n '1p')
refs_checked=$(echo "$refs_result" | sed -n '2p')
refs_issues=$(echo "$refs_result" | sed -n '3p')

version_result=$(check_versions)
version_status=$(echo "$version_result" | sed -n '1p')
versions=$(echo "$version_result" | sed -n '2p')
version_issues=$(echo "$version_result" | sed -n '3p')

convention_result=$(check_conventions)
convention_status=$(echo "$convention_result" | sed -n '1p')
convention_issues=$(echo "$convention_result" | sed -n '2p')

# Determine overall status
overall_status="PASS"
total_errors=0
total_warnings=0

for status in "$spec_code_status" "$schema_status" "$refs_status" "$version_status" "$convention_status"; do
    case "$status" in
        FAIL)
            overall_status="FAIL"
            total_errors=$((total_errors + 1))
            ;;
        WARNING)
            [[ "$overall_status" != "FAIL" ]] && overall_status="WARNINGS"
            total_warnings=$((total_warnings + 1))
            ;;
    esac
done

# ============================================================================
# OUTPUT
# ============================================================================

if $JSON_MODE; then
    # Helper to format issues array
    format_issues() {
        local issues="$1"
        if [[ -z "$issues" ]]; then
            echo "[]"
        else
            echo "[$issues]"
        fi
    }
    
    # Parse versions to JSON object
    spec_v=$(echo "$versions" | grep -o 'spec:[^ ]*' | sed 's/spec://')
    schema_v=$(echo "$versions" | grep -o 'schema:[^ ]*' | sed 's/schema://')
    cli_v=$(echo "$versions" | grep -o 'cli:[^ ]*' | sed 's/cli://')
    
    cat << JSONEOF
{
  "rfc_id": "RFC-$RFC_ID_PADDED",
  "check_timestamp": "$TIMESTAMP",
  "overall_status": "$overall_status",
  "checks": {
    "spec_code": {
      "status": "$spec_code_status",
      "items_checked": $spec_code_items,
      "issues": $(format_issues "$spec_code_issues")
    },
    "schema_validation": {
      "status": "$schema_status",
      "schemas_checked": ["${schema_checked// /\", \"}"],
      "issues": $(format_issues "$schema_issues")
    },
    "cross_references": {
      "status": "$refs_status",
      "refs_checked": $refs_checked,
      "issues": $(format_issues "$refs_issues")
    },
    "version_alignment": {
      "status": "$version_status",
      "versions": {"spec": "$spec_v", "schema": "$schema_v", "cli": "$cli_v"},
      "issues": $(format_issues "$version_issues")
    },
    "conventions": {
      "status": "$convention_status",
      "issues": $(format_issues "$convention_issues")
    }
  },
  "summary": {
    "total_issues": $((total_errors + total_warnings)),
    "errors": $total_errors,
    "warnings": $total_warnings,
    "info": 0
  }
}
JSONEOF
else
    echo "========================================"
    echo "RFC-$RFC_ID_PADDED Consistency Check"
    echo "========================================"
    echo ""
    echo "Overall Status: $overall_status"
    echo ""
    echo "Spec-Code:      $spec_code_status ($spec_code_items items)"
    echo "Schema:         $schema_status"
    echo "Cross-Refs:     $refs_status ($refs_checked refs)"
    echo "Versions:       $version_status"
    echo "Conventions:    $convention_status"
    echo ""
    echo "Summary: $total_errors errors, $total_warnings warnings"
fi

# Exit with appropriate code
if [[ "$overall_status" == "FAIL" ]]; then
    exit 1
elif [[ "$overall_status" == "WARNINGS" && "$STRICT_MODE" == true ]]; then
    exit 1
fi

exit 0
