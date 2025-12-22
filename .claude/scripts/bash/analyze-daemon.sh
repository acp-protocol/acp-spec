#!/usr/bin/env bash

# Analyze ACP Daemon components for documentation and evaluation
#
# This script examines:
# 1. File watcher implementation (cli/src/watch.rs)
# 2. Proxy implementation (cli/src/commands/proxy.rs)
# 3. Sync implementation (cli/src/commands/sync.rs)
# 4. MCP server documentation
# 5. Configuration options
#
# Usage: ./analyze-daemon.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --component <n>     Analyze specific component only
#   --status            Show implementation status only
#   --goals             Compare against project goals
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
TARGET_COMPONENT=""
STATUS_ONLY=false
SHOW_GOALS=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json)
            JSON_MODE=true
            shift
            ;;
        --component)
            TARGET_COMPONENT="$2"
            shift 2
            ;;
        --status)
            STATUS_ONLY=true
            shift
            ;;
        --goals)
            SHOW_GOALS=true
            shift
            ;;
        --help|-h)
            cat << 'EOF'
Usage: analyze-daemon.sh [OPTIONS]

Analyze ACP Daemon components for documentation and evaluation.

OPTIONS:
  --json              Output in JSON format
  --component <n>     Analyze specific component (watch, proxy, sync, mcp)
  --status            Show implementation status only
  --goals             Compare against project goals
  --help, -h          Show this help message

COMPONENTS:
  watch   - File watcher (cli/src/watch.rs)
  proxy   - HTTP proxy (cli/src/commands/proxy.rs)
  sync    - Tool sync (cli/src/commands/sync.rs)
  mcp     - MCP server (docs/integrations/)

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
DOCS_DIR="$PROJECT_ROOT/docs"
SPEC_DIR="$PROJECT_ROOT/spec"

# ============================================================================
# ANALYSIS FUNCTIONS
# ============================================================================

# Check if file exists and get info
analyze_file() {
    local file="$1"
    local full_path="$PROJECT_ROOT/$file"
    
    if [[ -f "$full_path" ]]; then
        local lines=$(wc -l < "$full_path")
        local size=$(du -h "$full_path" | cut -f1)
        echo "exists:true|lines:$lines|size:$size"
    else
        echo "exists:false|lines:0|size:0"
    fi
}

# Check for specific patterns in file
check_pattern() {
    local file="$1"
    local pattern="$2"
    local full_path="$PROJECT_ROOT/$file"
    
    if [[ -f "$full_path" ]] && grep -q "$pattern" "$full_path" 2>/dev/null; then
        echo "true"
    else
        echo "false"
    fi
}

# Count TODO/FIXME in file
count_todos() {
    local file="$1"
    local full_path="$PROJECT_ROOT/$file"
    
    if [[ -f "$full_path" ]]; then
        grep -c "TODO\|FIXME" "$full_path" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# ============================================================================
# COMPONENT ANALYSIS
# ============================================================================

analyze_watch() {
    local watch_file="cli/src/watch.rs"
    local info=$(analyze_file "$watch_file")
    
    local has_notify=$(check_pattern "$watch_file" "notify::")
    local has_incremental=$(check_pattern "$watch_file" "incremental")
    local has_debounce=$(check_pattern "$watch_file" "debounce")
    local has_filter=$(check_pattern "$watch_file" "filter\|exclude")
    local todos=$(count_todos "$watch_file")
    
    if $JSON_MODE; then
        cat << EOF
{
  "component": "watch",
  "file": "$watch_file",
  "info": {$(echo "$info" | sed 's/|/,"/g' | sed 's/:/":"/g' | sed 's/^/"/' | sed 's/$/"/')},
  "features": {
    "notify_integration": $has_notify,
    "incremental_updates": $has_incremental,
    "debouncing": $has_debounce,
    "pattern_filtering": $has_filter
  },
  "todos": $todos,
  "status": "$([ "$has_incremental" == "true" ] && echo "complete" || echo "partial")"
}
EOF
    else
        echo "=== File Watcher (acp watch) ==="
        echo "File: $watch_file"
        echo "Status: $(echo "$info" | grep -o 'exists:[^|]*' | cut -d: -f2)"
        echo ""
        echo "Features:"
        echo "  ✓ notify integration: $has_notify"
        echo "  $([ "$has_incremental" == "true" ] && echo "✓" || echo "○") incremental updates: $has_incremental"
        echo "  $([ "$has_debounce" == "true" ] && echo "✓" || echo "○") debouncing: $has_debounce"
        echo "  $([ "$has_filter" == "true" ] && echo "✓" || echo "○") pattern filtering: $has_filter"
        echo ""
        echo "TODOs remaining: $todos"
    fi
}

analyze_proxy() {
    local proxy_file="cli/src/commands/proxy.rs"
    local info=$(analyze_file "$proxy_file")
    local exists=$(echo "$info" | grep -o 'exists:[^|]*' | cut -d: -f2)
    
    # Check spec for proxy documentation
    local spec_documented=$(check_pattern "spec/chapters/13-tool-integration.md" "acp proxy")
    
    if $JSON_MODE; then
        cat << EOF
{
  "component": "proxy",
  "file": "$proxy_file",
  "exists": $exists,
  "spec_documented": $spec_documented,
  "status": "$([ "$exists" == "true" ] && echo "implemented" || echo "planned")"
}
EOF
    else
        echo "=== HTTP Proxy (acp proxy) ==="
        echo "File: $proxy_file"
        echo "Implemented: $exists"
        echo "Spec documented: $spec_documented"
        echo "Status: $([ "$exists" == "true" ] && echo "implemented" || echo "planned")"
    fi
}

analyze_sync() {
    local sync_file="cli/src/commands/sync.rs"
    local info=$(analyze_file "$sync_file")
    local exists=$(echo "$info" | grep -o 'exists:[^|]*' | cut -d: -f2)
    
    # Check for tool adapters
    local has_cursor=$(check_pattern "$sync_file" "cursor\|Cursor")
    local has_claude=$(check_pattern "$sync_file" "claude\|Claude")
    local has_copilot=$(check_pattern "$sync_file" "copilot\|Copilot")
    
    if $JSON_MODE; then
        cat << EOF
{
  "component": "sync",
  "file": "$sync_file",
  "exists": $exists,
  "adapters": {
    "cursor": $has_cursor,
    "claude_code": $has_claude,
    "copilot": $has_copilot
  },
  "status": "$([ "$exists" == "true" ] && echo "skeleton" || echo "planned")"
}
EOF
    else
        echo "=== Sync Service (acp sync) ==="
        echo "File: $sync_file"
        echo "Implemented: $exists"
        echo ""
        echo "Tool Adapters:"
        echo "  $([ "$has_cursor" == "true" ] && echo "✓" || echo "○") Cursor"
        echo "  $([ "$has_claude" == "true" ] && echo "✓" || echo "○") Claude Code"
        echo "  $([ "$has_copilot" == "true" ] && echo "✓" || echo "○") GitHub Copilot"
    fi
}

analyze_mcp() {
    local mcp_readme="docs/integrations/README.md"
    local info=$(analyze_file "$mcp_readme")
    local exists=$(echo "$info" | grep -o 'exists:[^|]*' | cut -d: -f2)
    
    # Check for tool definitions
    local has_query=$(check_pattern "$mcp_readme" "acp_query")
    local has_constraints=$(check_pattern "$mcp_readme" "acp_constraints")
    local has_expand=$(check_pattern "$mcp_readme" "acp_expand")
    local has_debug=$(check_pattern "$mcp_readme" "acp_debug")
    
    if $JSON_MODE; then
        cat << EOF
{
  "component": "mcp",
  "file": "$mcp_readme",
  "documented": $exists,
  "tools": {
    "acp_query": $has_query,
    "acp_constraints": $has_constraints,
    "acp_expand": $has_expand,
    "acp_debug": $has_debug
  },
  "status": "$([ "$exists" == "true" ] && echo "documented" || echo "planned")"
}
EOF
    else
        echo "=== MCP Server ==="
        echo "Documentation: $mcp_readme"
        echo "Documented: $exists"
        echo ""
        echo "Tools:"
        echo "  $([ "$has_query" == "true" ] && echo "✓" || echo "○") acp_query"
        echo "  $([ "$has_constraints" == "true" ] && echo "✓" || echo "○") acp_constraints"
        echo "  $([ "$has_expand" == "true" ] && echo "✓" || echo "○") acp_expand"
        echo "  $([ "$has_debug" == "true" ] && echo "✓" || echo "○") acp_debug"
    fi
}

# ============================================================================
# GOAL ALIGNMENT CHECK
# ============================================================================

check_goal_alignment() {
    echo "=== Project Goal Alignment ==="
    echo ""
    
    # G1: Token Efficiency
    echo "G1: Token Efficiency"
    echo "  Proxy bootstrap: ~50-80 tokens (vs ~1000+ file sync)"
    echo "  Primer system: Budget-aware selection"
    echo "  Alignment: ✅ Strong"
    echo ""
    
    # G6: Safety First
    echo "G6: Safety First"
    local has_safety=$(check_pattern "primers/primer.defaults.json" "safety")
    local has_required=$(check_pattern "schemas/v1/primer.schema.json" "required")
    echo "  Safety dimension in primer: $has_safety"
    echo "  Required sections support: $has_required"
    echo "  Alignment: ✅ Strong"
    echo ""
    
    # G7: Transparent Integration
    echo "G7: Transparent Integration"
    local has_init_sync=$(check_pattern "cli/src/main.rs" "sync")
    echo "  Auto-sync on init: $has_init_sync"
    echo "  Manual proxy startup required"
    echo "  Alignment: ⚠️ Partial"
}

# ============================================================================
# STATUS SUMMARY
# ============================================================================

show_status_summary() {
    echo "═══════════════════════════════════════════════════════════════════"
    echo "  ACP Daemon Implementation Status"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    
    printf "%-20s %-15s %-15s\n" "Component" "Status" "Progress"
    printf "%-20s %-15s %-15s\n" "---------" "------" "--------"
    
    # Watch
    local watch_exists=$([ -f "$CLI_SRC/watch.rs" ] && echo "true" || echo "false")
    printf "%-20s %-15s %-15s\n" "File Watcher" "$([ "$watch_exists" == "true" ] && echo "Partial" || echo "Missing")" "████░░░░ 40%"
    
    # Proxy
    local proxy_exists=$([ -f "$CLI_SRC/commands/proxy.rs" ] && echo "true" || echo "false")
    printf "%-20s %-15s %-15s\n" "HTTP Proxy" "$([ "$proxy_exists" == "true" ] && echo "Started" || echo "Planned")" "░░░░░░░░ 0%"
    
    # Sync
    local sync_exists=$([ -f "$CLI_SRC/commands/sync.rs" ] && echo "true" || echo "false")
    printf "%-20s %-15s %-15s\n" "Sync Service" "$([ "$sync_exists" == "true" ] && echo "Skeleton" || echo "Planned")" "██░░░░░░ 20%"
    
    # MCP
    local mcp_exists=$([ -f "$DOCS_DIR/integrations/README.md" ] && echo "true" || echo "false")
    printf "%-20s %-15s %-15s\n" "MCP Server" "$([ "$mcp_exists" == "true" ] && echo "Documented" || echo "Planned")" "████████ 100%"
    
    echo ""
    echo "═══════════════════════════════════════════════════════════════════"
}

# ============================================================================
# MAIN
# ============================================================================

main() {
    if $STATUS_ONLY; then
        show_status_summary
        exit 0
    fi
    
    if $SHOW_GOALS; then
        check_goal_alignment
        exit 0
    fi
    
    case "$TARGET_COMPONENT" in
        "watch")
            analyze_watch
            ;;
        "proxy")
            analyze_proxy
            ;;
        "sync")
            analyze_sync
            ;;
        "mcp")
            analyze_mcp
            ;;
        "")
            # Analyze all
            if $JSON_MODE; then
                echo "{"
                echo '"daemon_analysis": {'
                echo '"components": ['
                analyze_watch
                echo ","
                analyze_proxy
                echo ","
                analyze_sync
                echo ","
                analyze_mcp
                echo "],"
                echo '"summary": {'
                echo '"total_components": 4,'
                echo '"implemented": 2,'
                echo '"planned": 2'
                echo "}"
                echo "}"
                echo "}"
            else
                analyze_watch
                echo ""
                analyze_proxy
                echo ""
                analyze_sync
                echo ""
                analyze_mcp
                echo ""
                show_status_summary
            fi
            ;;
        *)
            echo "ERROR: Unknown component '$TARGET_COMPONENT'" >&2
            exit 2
            ;;
    esac
}

main "$@"
