#!/usr/bin/env bash

# Verify ACP CLI produces correct output after RFC-001 remediation
#
# This script runs tests to verify:
# 1. Cache generation includes directive fields
# 2. Parser extracts directives correctly
# 3. Commands produce expected output
# 4. Cache validates against schema
#
# Usage: ./verify-cli-directives.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --sample <path>     Path to sample test project
#   --schema <path>     Path to cache schema
#   --quick             Run only essential tests
#   --verbose           Detailed output
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
VERBOSE=false
QUICK_MODE=false
SAMPLE_PROJECT=""
SCHEMA_FILE=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json)
            JSON_MODE=true
            shift
            ;;
        --sample)
            SAMPLE_PROJECT="$2"
            shift 2
            ;;
        --schema)
            SCHEMA_FILE="$2"
            shift 2
            ;;
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            cat << 'EOF'
Usage: verify-cli-directives.sh [OPTIONS]

Verify ACP CLI output after RFC-001 remediation.

OPTIONS:
  --json              Output in JSON format
  --sample <path>     Path to sample test project (default: auto-create)
  --schema <path>     Path to cache schema for validation
  --quick             Run only essential tests
  --verbose           Show detailed test output
  --help, -h          Show this help message

TEST CATEGORIES:
  Cache Generation    - Verifies cache structure
  Directive Parsing   - Verifies annotation extraction
  Symbol Indexing     - Verifies symbol annotations
  Inline Tracking     - Verifies hack/todo markers
  Command Output      - Verifies CLI commands
  Schema Compliance   - Validates against JSON schema

EXIT CODES:
  0 - All tests pass
  1 - Some tests failed
  2 - Error (setup failed, etc.)

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

OUTPUT_DIR="$PROJECT_ROOT/.acp"
TEMP_DIR="$OUTPUT_DIR/verify-temp"
TEST_CACHE="$TEMP_DIR/test-cache.json"

[[ -z "$SAMPLE_PROJECT" ]] && SAMPLE_PROJECT="$TEMP_DIR/test-project"
[[ -z "$SCHEMA_FILE" ]] && SCHEMA_FILE="$PROJECT_ROOT/schemas/v1/cache.schema.json"

# Find CLI binary
CLI_BIN=""
for candidate in \
    "$PROJECT_ROOT/target/release/acp" \
    "$PROJECT_ROOT/target/debug/acp" \
    "$(which acp 2>/dev/null)"
do
    if [[ -x "$candidate" ]]; then
        CLI_BIN="$candidate"
        break
    fi
done

mkdir -p "$TEMP_DIR"

# ============================================================================
# TEST RESULTS
# ============================================================================

declare -A TEST_RESULTS
declare -a TEST_FAILURES

record_test() {
    local test_id="$1"
    local result="$2"  # pass|fail
    local message="$3"

    TEST_RESULTS["$test_id"]="$result"
    if [[ "$result" == "fail" ]]; then
        TEST_FAILURES+=("$test_id|$message")
    fi

    if $VERBOSE; then
        if [[ "$result" == "pass" ]]; then
            echo "  ✓ $test_id: $message"
        else
            echo "  ✗ $test_id: $message"
        fi
    fi
}

# ============================================================================
# TEST PROJECT SETUP
# ============================================================================

setup_test_project() {
    if [[ -d "$SAMPLE_PROJECT" && -f "$SAMPLE_PROJECT/directive-test.ts" ]]; then
        return 0  # Already exists
    fi

    mkdir -p "$SAMPLE_PROJECT"

    # Create directive test file
    cat > "$SAMPLE_PROJECT/directive-test.ts" << 'EOF'
// @acp:lock frozen - MUST NOT modify this file under any circumstances
// @acp:ref https://docs.example.com - Consult documentation before changes
// @acp:owner security-team - Contact for questions

export function criticalFunction() {
    return true;
}
EOF

    # Create symbol test file
    cat > "$SAMPLE_PROJECT/symbol-test.ts" << 'EOF'
// @acp:purpose User authentication and session management module

// @acp:fn validateUser - Validates user credentials against database
// @acp:param userId - The unique user identifier
// @acp:param token - JWT authentication token
// @acp:returns Boolean indicating validation success
// @acp:lock frozen - MUST NOT modify validation logic
export function validateUser(userId: string, token: string): boolean {
    return token.length > 0;
}

// @acp:class UserStore - In-memory user session cache with TTL
export class UserStore {
    // @acp:method get - Retrieve user session by ID
    get(id: string): any { return null; }

    // @acp:method set - Store user session with expiry
    set(id: string, data: any): void { }
}
EOF

    # Create inline test file
    cat > "$SAMPLE_PROJECT/inline-test.ts" << 'EOF'
// @acp:purpose Test file for inline annotations

export function processData() {
    // @acp:hack - Timezone workaround for server clock drift
    //   @acp:hack-expires 2024-06-01
    //   @acp:hack-ticket JIRA-1234
    const offset = new Date().getTimezoneOffset();

    // @acp:todo - Add retry logic for network failures
    fetch("https://api.example.com");

    // @acp:fixme - Memory leak when processing large datasets
    const data = [];

    // @acp:critical - Security boundary, validate all inputs
    validateInput(offset);
}

function validateInput(x: any) {}
EOF

    # Create multiline test file
    cat > "$SAMPLE_PROJECT/multiline-test.ts" << 'EOF'
// @acp:lock restricted - Explain proposed changes and wait for approval.
//   This file handles PCI-compliant payment processing.
//   All changes require security team review.
//   Contact: payments-team@company.com

export function processPayment(amount: number) {
    return amount * 1.1;
}
EOF

    # Create legacy test file (no directives)
    cat > "$SAMPLE_PROJECT/legacy-test.ts" << 'EOF'
// @acp:lock frozen
// @acp:ref https://old-docs.example.com

export function legacyFunction() {
    return "legacy";
}
EOF

    # Create config
    cat > "$SAMPLE_PROJECT/.acp.config.json" << 'EOF'
{
  "$schema": "https://acp-protocol.dev/schemas/v1/config.schema.json",
  "version": "1.0.0",
  "include": ["**/*.ts"],
  "exclude": ["node_modules/**"]
}
EOF
}

# ============================================================================
# TEST FUNCTIONS
# ============================================================================

test_cache_generation() {
    echo "Testing Cache Generation..."

    # Generate cache
    if ! $CLI_BIN index "$SAMPLE_PROJECT" --output "$TEST_CACHE" 2>/dev/null; then
        record_test "CG00" "fail" "CLI index command failed"
        return 1
    fi

    if [[ ! -f "$TEST_CACHE" ]]; then
        record_test "CG00" "fail" "Cache file not created"
        return 1
    fi

    # CG01: Check cache has version
    if jq -e '.version' "$TEST_CACHE" >/dev/null 2>&1; then
        record_test "CG01" "pass" "Cache has version field"
    else
        record_test "CG01" "fail" "Cache missing version field"
    fi

    # CG02: Check files have purpose field capability
    if jq -e '.files | to_entries[].value | has("purpose")' "$TEST_CACHE" >/dev/null 2>&1; then
        record_test "CG02" "pass" "Files support purpose field"
    else
        record_test "CG02" "fail" "Files missing purpose field support"
    fi

    # CG03: Check files have symbols array capability
    if jq -e '.files | to_entries | .[0].value | has("symbols")' "$TEST_CACHE" >/dev/null 2>&1; then
        record_test "CG03" "pass" "Files support symbols array"
    else
        record_test "CG03" "fail" "Files missing symbols array support"
    fi

    # CG04: Check files have inline array capability
    if jq -e '.files | to_entries | .[0].value | has("inline")' "$TEST_CACHE" >/dev/null 2>&1; then
        record_test "CG04" "pass" "Files support inline array"
    else
        record_test "CG04" "fail" "Files missing inline array support"
    fi
}

test_directive_parsing() {
    echo "Testing Directive Parsing..."

    # DP01: Check explicit directive extracted
    local directive=$(jq -r '.annotations["'"$SAMPLE_PROJECT"'/directive-test.ts"][0].directive // empty' "$TEST_CACHE" 2>/dev/null)
    if [[ -n "$directive" && "$directive" != "null" ]]; then
        record_test "DP01" "pass" "Explicit directive extracted"
    else
        record_test "DP01" "fail" "Explicit directive not extracted"
    fi

    # DP02: Check auto-generated directive for legacy annotations
    local auto_gen=$(jq -r '.annotations["'"$SAMPLE_PROJECT"'/legacy-test.ts"][0].auto_generated // false' "$TEST_CACHE" 2>/dev/null)
    if [[ "$auto_gen" == "true" ]]; then
        record_test "DP02" "pass" "Auto-generated directive flag set"
    else
        record_test "DP02" "fail" "Auto-generated directive flag not set"
    fi

    # DP03: Check multiline directive joined
    local multiline_dir=$(jq -r '.annotations["'"$SAMPLE_PROJECT"'/multiline-test.ts"][0].directive // empty' "$TEST_CACHE" 2>/dev/null)
    if [[ "$multiline_dir" == *"payments-team"* ]]; then
        record_test "DP03" "pass" "Multi-line directive joined correctly"
    else
        record_test "DP03" "fail" "Multi-line directive not joined"
    fi
}

test_symbol_indexing() {
    echo "Testing Symbol Indexing..."

    local symbols_file="$SAMPLE_PROJECT/symbol-test.ts"

    # SI01: Check purpose extracted from @acp:purpose
    local purpose=$(jq -r '.files["'"$symbols_file"'"].purpose // empty' "$TEST_CACHE" 2>/dev/null)
    if [[ -n "$purpose" && "$purpose" != "null" ]]; then
        record_test "SI01" "pass" "File purpose extracted"
    else
        record_test "SI01" "fail" "File purpose not extracted"
    fi

    # SI02: Check symbols array populated
    local symbol_count=$(jq '.files["'"$symbols_file"'"].symbols | length // 0' "$TEST_CACHE" 2>/dev/null)
    if [[ "$symbol_count" -gt 0 ]]; then
        record_test "SI02" "pass" "Symbols array populated ($symbol_count symbols)"
    else
        record_test "SI02" "fail" "Symbols array empty"
    fi

    # SI03: Check symbol has lines field
    local has_lines=$(jq '.files["'"$symbols_file"'"].symbols[0].lines // null' "$TEST_CACHE" 2>/dev/null)
    if [[ "$has_lines" != "null" ]]; then
        record_test "SI03" "pass" "Symbol has lines field"
    else
        record_test "SI03" "fail" "Symbol missing lines field"
    fi
}

test_inline_tracking() {
    echo "Testing Inline Tracking..."

    local inline_file="$SAMPLE_PROJECT/inline-test.ts"

    # IT01: Check inline array populated
    local inline_count=$(jq '.files["'"$inline_file"'"].inline | length // 0' "$TEST_CACHE" 2>/dev/null)
    if [[ "$inline_count" -gt 0 ]]; then
        record_test "IT01" "pass" "Inline array populated ($inline_count items)"
    else
        record_test "IT01" "fail" "Inline array empty"
    fi

    # IT02: Check hack has expires field
    local has_expires=$(jq '.files["'"$inline_file"'"].inline[] | select(.type=="hack") | .expires // null' "$TEST_CACHE" 2>/dev/null | head -1)
    if [[ -n "$has_expires" && "$has_expires" != "null" ]]; then
        record_test "IT02" "pass" "Hack has expires field"
    else
        record_test "IT02" "fail" "Hack missing expires field"
    fi

    # IT03: Check hack has ticket field
    local has_ticket=$(jq '.files["'"$inline_file"'"].inline[] | select(.type=="hack") | .ticket // null' "$TEST_CACHE" 2>/dev/null | head -1)
    if [[ -n "$has_ticket" && "$has_ticket" != "null" ]]; then
        record_test "IT03" "pass" "Hack has ticket field"
    else
        record_test "IT03" "fail" "Hack missing ticket field"
    fi
}

test_command_output() {
    echo "Testing Command Output..."

    if $QUICK_MODE; then
        record_test "CO00" "pass" "Command tests skipped (quick mode)"
        return
    fi

    # CO01: Test map command exists
    if $CLI_BIN map --help >/dev/null 2>&1; then
        record_test "CO01" "pass" "'acp map' command exists"
    else
        record_test "CO01" "fail" "'acp map' command not found"
    fi

    # CO02: Test migrate command exists
    if $CLI_BIN migrate --help >/dev/null 2>&1; then
        record_test "CO02" "pass" "'acp migrate' command exists"
    else
        record_test "CO02" "fail" "'acp migrate' command not found"
    fi

    # CO03: Test constraints shows directive
    local constraints_output=$($CLI_BIN constraints "$SAMPLE_PROJECT/directive-test.ts" 2>/dev/null || true)
    if [[ "$constraints_output" == *"directive"* || "$constraints_output" == *"MUST NOT"* ]]; then
        record_test "CO03" "pass" "'acp constraints' shows directive"
    else
        record_test "CO03" "fail" "'acp constraints' missing directive output"
    fi
}

test_schema_compliance() {
    echo "Testing Schema Compliance..."

    if [[ ! -f "$SCHEMA_FILE" ]]; then
        record_test "SC00" "fail" "Schema file not found: $SCHEMA_FILE"
        return
    fi

    # SC01: Validate cache against schema
    if $CLI_BIN validate "$TEST_CACHE" --schema "$SCHEMA_FILE" >/dev/null 2>&1; then
        record_test "SC01" "pass" "Cache validates against schema"
    else
        # Try with jsonschema if available
        if command -v jsonschema >/dev/null 2>&1; then
            if jsonschema -i "$TEST_CACHE" "$SCHEMA_FILE" >/dev/null 2>&1; then
                record_test "SC01" "pass" "Cache validates against schema"
            else
                record_test "SC01" "fail" "Cache fails schema validation"
            fi
        else
            record_test "SC01" "fail" "Cannot validate schema (no validator available)"
        fi
    fi
}

# ============================================================================
# OUTPUT
# ============================================================================

output_json() {
    local total=${#TEST_RESULTS[@]}
    local passed=0
    local failed=0

    for result in "${TEST_RESULTS[@]}"; do
        [[ "$result" == "pass" ]] && ((passed++)) || ((failed++))
    done

    local status="pass"
    [[ $failed -gt 0 ]] && status="fail"

    cat << EOF
{
  "status": "$status",
  "cli_version": "$($CLI_BIN --version 2>/dev/null | head -1 || echo "unknown")",
  "tests_run": $total,
  "tests_passed": $passed,
  "tests_failed": $failed,
  "categories": {
    "cache_generation": "$([ "${TEST_RESULTS[CG01]}" == "pass" ] && echo "pass" || echo "fail")",
    "directive_parsing": "$([ "${TEST_RESULTS[DP01]}" == "pass" ] && echo "pass" || echo "fail")",
    "symbol_indexing": "$([ "${TEST_RESULTS[SI01]}" == "pass" ] && echo "pass" || echo "fail")",
    "inline_tracking": "$([ "${TEST_RESULTS[IT01]}" == "pass" ] && echo "pass" || echo "fail")",
    "command_output": "$([ "${TEST_RESULTS[CO01]}" == "pass" ] && echo "pass" || echo "fail")",
    "schema_compliance": "$([ "${TEST_RESULTS[SC01]}" == "pass" ] && echo "pass" || echo "fail")"
  },
  "failures": $(printf '%s\n' "${TEST_FAILURES[@]:-}" | awk -F'|' 'NF{print "{\"test\":\""$1"\",\"message\":\""$2"\"}"}' | jq -s . 2>/dev/null || echo "[]"),
  "sample_cache": "$TEST_CACHE"
}
EOF
}

output_text() {
    local total=${#TEST_RESULTS[@]}
    local passed=0
    local failed=0

    for result in "${TEST_RESULTS[@]}"; do
        [[ "$result" == "pass" ]] && ((passed++)) || ((failed++))
    done

    echo ""
    echo "═══════════════════════════════════════════════════════════════════"
    echo "  CLI Verification Results"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    echo "Tests: $total total, $passed passed, $failed failed"
    echo ""

    if [[ ${#TEST_FAILURES[@]} -gt 0 ]]; then
        echo "Failures:"
        echo "─────────"
        for failure in "${TEST_FAILURES[@]}"; do
            IFS='|' read -r test_id message <<< "$failure"
            echo "  ✗ $test_id: $message"
        done
        echo ""
    fi

    if [[ $failed -eq 0 ]]; then
        echo "✓ All tests passed"
    else
        echo "✗ Some tests failed - see above for details"
    fi

    echo ""
    echo "Sample cache: $TEST_CACHE"
    echo "═══════════════════════════════════════════════════════════════════"
}

# ============================================================================
# MAIN
# ============================================================================

main() {
    # Check for CLI binary
    if [[ -z "$CLI_BIN" ]]; then
        if $JSON_MODE; then
            echo '{"status":"error","message":"CLI binary not found"}'
        else
            echo "ERROR: CLI binary not found. Build with 'cargo build --release'" >&2
        fi
        exit 2
    fi

    # Setup test project
    setup_test_project

    # Run tests
    test_cache_generation
    test_directive_parsing
    test_symbol_indexing
    test_inline_tracking
    test_command_output
    test_schema_compliance

    # Output results
    if $JSON_MODE; then
        output_json
    else
        output_text
    fi

    # Exit code
    local failed=0
    for result in "${TEST_RESULTS[@]}"; do
        [[ "$result" == "fail" ]] && ((failed++))
    done

    [[ $failed -gt 0 ]] && exit 1
    exit 0
}

main "$@"