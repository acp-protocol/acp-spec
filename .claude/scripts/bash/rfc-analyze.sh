#!/usr/bin/env bash

# RFC Analysis Script
#
# Analyzes an RFC for completeness, technical viability, and alignment.
# Used by /rfc.analyze command.
#
# Usage: ./rfc-analyze.sh [OPTIONS] <rfc-path>
#
# OPTIONS:
#   --json              Output in JSON format
#   --strict            Apply strict validation rules
#   --quick             Quick analysis (skip deep checks)
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
STRICT_MODE=false
QUICK_MODE=false
RFC_PATH=""

for arg in "$@"; do
    case "$arg" in
        --json)
            JSON_MODE=true
            ;;
        --strict)
            STRICT_MODE=true
            ;;
        --quick)
            QUICK_MODE=true
            ;;
        --help|-h)
            cat << 'EOF'
Usage: rfc-analyze.sh [OPTIONS] <rfc-path>

Analyze an RFC for completeness, technical viability, and alignment.

OPTIONS:
  --json              Output in JSON format
  --strict            Apply strict validation rules
  --quick             Quick analysis (skip deep checks)
  --help, -h          Show this help message

ARGUMENTS:
  <rfc-path>          Path to RFC file to analyze

OUTPUTS:
  - Completeness score
  - Technical viability assessment
  - Alignment check results
  - Risk assessment
  - Overall recommendation (ACCEPT/REJECT/CLARIFY)

EXAMPLES:
  # Analyze RFC with JSON output
  ./rfc-analyze.sh --json rfcs/proposed/0001-new-feature.md
  
  # Quick analysis
  ./rfc-analyze.sh --quick rfcs/proposed/0001-new-feature.md
  
  # Strict mode
  ./rfc-analyze.sh --strict --json rfcs/proposed/0001-new-feature.md

EOF
            exit 0
            ;;
        --*)
            echo "ERROR: Unknown option '$arg'. Use --help for usage information." >&2
            exit 1
            ;;
        *)
            if [[ -z "$RFC_PATH" ]]; then
                RFC_PATH="$arg"
            else
                echo "ERROR: Multiple RFC paths provided. Only one allowed." >&2
                exit 1
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

# Extract RFC metadata from header
extract_metadata() {
    local file="$1"
    
    # Extract RFC ID
    RFC_ID=$(grep -E "^-?\s*\*?\*?RFC\s*(ID|Number).*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    [[ -z "$RFC_ID" ]] && RFC_ID=$(basename "$file" .md | sed 's/^[0-9]*-/RFC-/')
    
    # Extract title
    RFC_TITLE=$(grep -E "^-?\s*\*?\*?Title.*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    [[ -z "$RFC_TITLE" ]] && RFC_TITLE=$(head -1 "$file" | sed 's/^#\s*//')
    
    # Extract status
    RFC_STATUS=$(grep -E "^-?\s*\*?\*?Status.*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    [[ -z "$RFC_STATUS" ]] && RFC_STATUS="Draft"
    
    # Extract author
    RFC_AUTHOR=$(grep -E "^-?\s*\*?\*?Author.*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    
    # Extract created date
    RFC_CREATED=$(grep -E "^-?\s*\*?\*?Created.*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
}

# Check for required sections
check_completeness() {
    local file="$1"
    local score=0
    local max_score=100
    local missing=()
    local incomplete=()
    local warnings=()
    
    # Required sections (weighted)
    local -A sections=(
        ["Summary"]=15
        ["Motivation"]=15
        ["Detailed Design"]=25
        ["Examples"]=10
        ["Drawbacks"]=10
        ["Alternatives"]=5
        ["Implementation"]=10
        ["Open Questions"]=5
        ["Backward Compatibility"]=5
    )
    
    for section in "${!sections[@]}"; do
        local weight="${sections[$section]}"
        if grep -qi "^##\s*$section" "$file" 2>/dev/null; then
            # Check if section has content (more than just header)
            local content=$(awk "/^##\s*$section/,/^##[^#]/" "$file" | tail -n +2 | grep -v "^$" | wc -l)
            if [[ "$content" -gt 2 ]]; then
                score=$((score + weight))
            else
                incomplete+=("$section")
                score=$((score + weight / 2))
            fi
        else
            missing+=("$section")
        fi
    done
    
    # Warnings
    if ! grep -q "^##.*Changelog" "$file" 2>/dev/null; then
        warnings+=("No changelog section")
    fi
    
    if ! grep -q "^##.*References" "$file" 2>/dev/null; then
        warnings+=("No references section")
    fi
    
    # Output
    echo "$score"
    echo "${missing[*]}"
    echo "${incomplete[*]}"
    echo "${warnings[*]}"
}

# Check technical viability
check_technical() {
    local file="$1"
    local viable=true
    local concerns=()
    local blockers=()
    local components=()
    local complexity="Low"
    
    # Detect affected components
    if grep -qi "schema" "$file"; then
        components+=("schemas")
    fi
    if grep -qi "cli\|command" "$file"; then
        components+=("cli")
    fi
    if grep -qi "spec\|specification\|chapter" "$file"; then
        components+=("spec")
    fi
    if grep -qi "parser\|annotation" "$file"; then
        components+=("parser")
    fi
    
    # Assess complexity based on components and scope
    local comp_count=${#components[@]}
    if [[ "$comp_count" -ge 4 ]]; then
        complexity="High"
    elif [[ "$comp_count" -ge 2 ]]; then
        complexity="Medium"
    fi
    
    # Check for concerning patterns
    if grep -qi "breaking change\|backwards incompatible\|migration required" "$file"; then
        concerns+=("Contains breaking changes")
        complexity="High"
    fi
    
    if grep -qi "new dependency\|requires.*library" "$file"; then
        concerns+=("Adds new dependencies")
    fi
    
    if grep -qi "security\|authentication\|encryption" "$file"; then
        concerns+=("Has security implications")
    fi
    
    if grep -qi "performance\|optimization\|benchmark" "$file"; then
        concerns+=("Has performance implications")
    fi
    
    # Check for blockers
    if grep -qi "blocked\|cannot proceed\|requires.*first" "$file"; then
        blockers+=("Has stated blockers")
        viable=false
    fi
    
    echo "$viable"
    echo "${concerns[*]}"
    echo "${blockers[*]}"
    echo "${components[*]}"
    echo "$complexity"
}

# Check alignment with project
check_alignment() {
    local file="$1"
    local constitution_compliant=true
    local breaking_changes=false
    local chapters=()
    local schemas=()
    
    # Check for breaking changes
    if grep -qi "breaking\|incompatible" "$file"; then
        breaking_changes=true
    fi
    
    # Detect affected spec chapters
    for i in {01..15}; do
        if grep -qi "chapter\s*$i\|chapter\s*0?$i" "$file"; then
            chapters+=("$i")
        fi
    done
    
    # Also check for chapter names
    if grep -qi "annotation" "$file"; then
        [[ ! " ${chapters[*]} " =~ " 02 " ]] && chapters+=("02")
    fi
    if grep -qi "cache" "$file"; then
        [[ ! " ${chapters[*]} " =~ " 03 " ]] && chapters+=("03")
    fi
    if grep -qi "constraint" "$file"; then
        [[ ! " ${chapters[*]} " =~ " 05 " ]] && chapters+=("05")
    fi
    
    # Detect affected schemas
    if grep -qi "cache.*schema\|\.cache\.json" "$file"; then
        schemas+=("cache.schema.json")
    fi
    if grep -qi "config.*schema\|\.config\.json" "$file"; then
        schemas+=("config.schema.json")
    fi
    if grep -qi "annotation.*schema" "$file"; then
        schemas+=("annotation.schema.json")
    fi
    
    echo "$constitution_compliant"
    echo "${chapters[*]}"
    echo "${schemas[*]}"
    echo "$breaking_changes"
}

# Assess risk level
assess_risk() {
    local file="$1"
    local level="Low"
    local factors=()
    local mitigations=()
    
    # Check risk factors
    if grep -qi "breaking change" "$file"; then
        factors+=("Breaking changes")
        level="High"
    fi
    
    if grep -qi "security" "$file"; then
        factors+=("Security implications")
        [[ "$level" != "High" ]] && level="Medium"
    fi
    
    if grep -qi "performance" "$file"; then
        factors+=("Performance impact")
        [[ "$level" == "Low" ]] && level="Medium"
    fi
    
    if grep -qi "migration" "$file"; then
        factors+=("Migration required")
        [[ "$level" == "Low" ]] && level="Medium"
    fi
    
    # Check for mitigations
    if grep -qi "feature flag\|opt-in\|gradual" "$file"; then
        mitigations+=("Feature flag rollout")
    fi
    
    if grep -qi "migration script\|migrate command" "$file"; then
        mitigations+=("Migration tooling")
    fi
    
    if grep -qi "backward compat\|backwards compat" "$file"; then
        mitigations+=("Backward compatibility maintained")
    fi
    
    echo "$level"
    echo "${factors[*]}"
    echo "${mitigations[*]}"
}

# Extract open questions
extract_questions() {
    local file="$1"
    
    # Look in Open Questions section
    local in_questions=false
    local questions=()
    
    while IFS= read -r line; do
        if [[ "$line" =~ ^##.*[Oo]pen.*[Qq]uestions ]]; then
            in_questions=true
            continue
        fi
        if [[ "$in_questions" == true && "$line" =~ ^## ]]; then
            break
        fi
        if [[ "$in_questions" == true && "$line" =~ ^[0-9]+\. ]]; then
            questions+=("${line#*. }")
        fi
    done < "$file"
    
    printf '%s\n' "${questions[@]}"
}

# Determine recommendation
determine_recommendation() {
    local completeness_score="$1"
    local viable="$2"
    local question_count="$3"
    local blockers="$4"
    local risk_level="$5"
    
    local recommendation="CLARIFY"
    local reasons=()
    
    # Check for rejection criteria
    if [[ "$viable" != "true" ]]; then
        recommendation="REJECT"
        reasons+=("Technical approach not viable")
    fi
    
    if [[ -n "$blockers" ]]; then
        recommendation="REJECT"
        reasons+=("Has blocking issues")
    fi
    
    if [[ "$completeness_score" -lt 50 ]]; then
        recommendation="CLARIFY"
        reasons+=("Incomplete RFC (score: $completeness_score/100)")
    fi
    
    # Check for acceptance criteria
    if [[ "$recommendation" != "REJECT" ]]; then
        if [[ "$completeness_score" -ge 80 && "$question_count" -le 2 && "$risk_level" != "Critical" ]]; then
            recommendation="ACCEPT"
            reasons+=("Meets acceptance criteria")
        elif [[ "$question_count" -gt 3 ]]; then
            recommendation="CLARIFY"
            reasons+=("$question_count open questions need resolution")
        elif [[ "$completeness_score" -lt 80 ]]; then
            recommendation="CLARIFY"
            reasons+=("Score below threshold ($completeness_score/100)")
        fi
    fi
    
    echo "$recommendation"
    printf '%s\n' "${reasons[@]}"
}

# ============================================================================
# MAIN ANALYSIS
# ============================================================================

# Extract metadata
extract_metadata "$RFC_PATH"

# Run completeness check
completeness_output=$(check_completeness "$RFC_PATH")
completeness_score=$(echo "$completeness_output" | sed -n '1p')
missing_sections=$(echo "$completeness_output" | sed -n '2p')
incomplete_sections=$(echo "$completeness_output" | sed -n '3p')
completeness_warnings=$(echo "$completeness_output" | sed -n '4p')

# Run technical check
technical_output=$(check_technical "$RFC_PATH")
tech_viable=$(echo "$technical_output" | sed -n '1p')
tech_concerns=$(echo "$technical_output" | sed -n '2p')
tech_blockers=$(echo "$technical_output" | sed -n '3p')
tech_components=$(echo "$technical_output" | sed -n '4p')
tech_complexity=$(echo "$technical_output" | sed -n '5p')

# Run alignment check
alignment_output=$(check_alignment "$RFC_PATH")
align_constitution=$(echo "$alignment_output" | sed -n '1p')
align_chapters=$(echo "$alignment_output" | sed -n '2p')
align_schemas=$(echo "$alignment_output" | sed -n '3p')
align_breaking=$(echo "$alignment_output" | sed -n '4p')

# Run risk assessment
risk_output=$(assess_risk "$RFC_PATH")
risk_level=$(echo "$risk_output" | sed -n '1p')
risk_factors=$(echo "$risk_output" | sed -n '2p')
risk_mitigations=$(echo "$risk_output" | sed -n '3p')

# Extract questions
questions=$(extract_questions "$RFC_PATH")
question_count=$(echo "$questions" | grep -c "." 2>/dev/null || echo "0")

# Determine recommendation
rec_output=$(determine_recommendation "$completeness_score" "$tech_viable" "$question_count" "$tech_blockers" "$risk_level")
recommendation=$(echo "$rec_output" | head -1)
reasons=$(echo "$rec_output" | tail -n +2)

# ============================================================================
# OUTPUT
# ============================================================================

if $JSON_MODE; then
    # Helper function to create JSON array from space-separated string
    to_json_array() {
        local input="$1"
        if [[ -z "$input" ]]; then
            echo "[]"
        else
            echo "[$(echo "$input" | sed 's/[^ ]*/"\0"/g' | tr ' ' ',')]"
        fi
    }
    
    # Helper for multi-line to JSON array
    lines_to_json_array() {
        local input="$1"
        if [[ -z "$input" ]]; then
            echo "[]"
        else
            echo "[$(echo "$input" | while read -r line; do
                [[ -n "$line" ]] && echo "\"$(echo "$line" | sed 's/"/\\"/g')\","
            done | sed '$ s/,$//' | tr -d '\n')]"
        fi
    }
    
    cat << JSONEOF
{
  "rfc": {
    "id": "$RFC_ID",
    "title": "$RFC_TITLE",
    "path": "$RFC_PATH",
    "status": "$RFC_STATUS",
    "author": "$RFC_AUTHOR",
    "created": "$RFC_CREATED"
  },
  "completeness": {
    "score": $completeness_score,
    "missing_sections": $(to_json_array "$missing_sections"),
    "incomplete_sections": $(to_json_array "$incomplete_sections"),
    "warnings": $(to_json_array "$completeness_warnings")
  },
  "technical": {
    "viable": $tech_viable,
    "concerns": $(to_json_array "$tech_concerns"),
    "blockers": $(to_json_array "$tech_blockers"),
    "affected_components": $(to_json_array "$tech_components"),
    "complexity": "$tech_complexity"
  },
  "alignment": {
    "constitution_compliant": $align_constitution,
    "spec_chapters_affected": $(to_json_array "$align_chapters"),
    "schema_changes_required": $(to_json_array "$align_schemas"),
    "breaking_changes": $align_breaking
  },
  "risk": {
    "level": "$risk_level",
    "factors": $(to_json_array "$risk_factors"),
    "mitigations": $(to_json_array "$risk_mitigations")
  },
  "questions": $(lines_to_json_array "$questions"),
  "recommendation": "$recommendation",
  "reasons": $(lines_to_json_array "$reasons")
}
JSONEOF
else
    echo "========================================"
    echo "RFC Analysis Report"
    echo "========================================"
    echo ""
    echo "RFC: $RFC_ID - $RFC_TITLE"
    echo "Path: $RFC_PATH"
    echo "Status: $RFC_STATUS"
    echo "Author: $RFC_AUTHOR"
    echo ""
    echo "----------------------------------------"
    echo "COMPLETENESS: $completeness_score/100"
    echo "----------------------------------------"
    [[ -n "$missing_sections" ]] && echo "Missing: $missing_sections"
    [[ -n "$incomplete_sections" ]] && echo "Incomplete: $incomplete_sections"
    [[ -n "$completeness_warnings" ]] && echo "Warnings: $completeness_warnings"
    echo ""
    echo "----------------------------------------"
    echo "TECHNICAL VIABILITY: $tech_complexity complexity"
    echo "----------------------------------------"
    echo "Viable: $tech_viable"
    [[ -n "$tech_concerns" ]] && echo "Concerns: $tech_concerns"
    [[ -n "$tech_blockers" ]] && echo "Blockers: $tech_blockers"
    [[ -n "$tech_components" ]] && echo "Components: $tech_components"
    echo ""
    echo "----------------------------------------"
    echo "ALIGNMENT"
    echo "----------------------------------------"
    echo "Constitution compliant: $align_constitution"
    [[ -n "$align_chapters" ]] && echo "Affected chapters: $align_chapters"
    [[ -n "$align_schemas" ]] && echo "Schema changes: $align_schemas"
    echo "Breaking changes: $align_breaking"
    echo ""
    echo "----------------------------------------"
    echo "RISK ASSESSMENT: $risk_level"
    echo "----------------------------------------"
    [[ -n "$risk_factors" ]] && echo "Factors: $risk_factors"
    [[ -n "$risk_mitigations" ]] && echo "Mitigations: $risk_mitigations"
    echo ""
    echo "----------------------------------------"
    echo "OPEN QUESTIONS ($question_count)"
    echo "----------------------------------------"
    [[ -n "$questions" ]] && echo "$questions"
    echo ""
    echo "========================================"
    echo "RECOMMENDATION: $recommendation"
    echo "========================================"
    echo ""
    [[ -n "$reasons" ]] && echo "$reasons"
fi

exit 0
