#!/usr/bin/env bash

# RFC Implementation Script
#
# Manages implementation sessions for RFCs.
# Used by /rfc.implement command.
#
# Usage: ./rfc-implement.sh [OPTIONS] <rfc-id>
#
# OPTIONS:
#   --init              Initialize new implementation session
#   --continue          Continue from last checkpoint
#   --resume <id>       Resume from specific checkpoint
#   --rollback <id>     Rollback to checkpoint
#   --status            Show current status
#   --json              Output in JSON format
#   --help, -h          Show help message

set -e

# ============================================================================
# CONFIGURATION
# ============================================================================

MEMORY_DIR=".claude/memory"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
MODE="status"
RFC_ID=""
CHECKPOINT_ID=""

for arg in "$@"; do
    case "$arg" in
        --json)
            JSON_MODE=true
            ;;
        --init)
            MODE="init"
            ;;
        --continue)
            MODE="continue"
            ;;
        --resume)
            MODE="resume"
            ;;
        --rollback)
            MODE="rollback"
            ;;
        --status)
            MODE="status"
            ;;
        --help|-h)
            cat << 'EOF'
Usage: rfc-implement.sh [OPTIONS] <rfc-id>

Manage RFC implementation sessions.

OPTIONS:
  --init              Initialize new implementation session
  --continue          Continue from last checkpoint
  --resume <id>       Resume from specific checkpoint
  --rollback <id>     Rollback to checkpoint
  --status            Show current status
  --json              Output in JSON format
  --help, -h          Show this help message

ARGUMENTS:
  <rfc-id>            RFC identifier (e.g., RFC-001, 001)

EXAMPLES:
  # Initialize new session
  ./rfc-implement.sh --init --json RFC-001
  
  # Continue from last checkpoint
  ./rfc-implement.sh --continue --json RFC-001
  
  # Check status
  ./rfc-implement.sh --status --json RFC-001

EOF
            exit 0
            ;;
        --*)
            echo "ERROR: Unknown option '$arg'. Use --help for usage." >&2
            exit 1
            ;;
        *)
            if [[ -z "$RFC_ID" ]]; then
                RFC_ID="$arg"
            else
                CHECKPOINT_ID="$arg"
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
# HELPER FUNCTIONS
# ============================================================================

# Ensure memory directory exists
ensure_memory_dir() {
    mkdir -p "$MEMORY_DIR"
}

# Get status file path
get_status_file() {
    echo "$MEMORY_DIR/rfc-status-$RFC_ID_PADDED.md"
}

# Get tasks file path
get_tasks_file() {
    echo "$MEMORY_DIR/rfc-tasks-$RFC_ID_PADDED.md"
}

# Get checkpoints file path
get_checkpoints_file() {
    echo "$MEMORY_DIR/rfc-checkpoints-$RFC_ID_PADDED.json"
}

# Initialize new session
init_session() {
    local session_id="impl-$RFC_ID_PADDED-$TIMESTAMP"
    local status_file=$(get_status_file)
    local tasks_file=$(get_tasks_file)
    
    ensure_memory_dir
    
    # Check if session already exists
    if [[ -f "$status_file" ]]; then
        if $JSON_MODE; then
            echo '{"error":"Session already exists","status_file":"'"$status_file"'","hint":"Use --continue to resume"}'
        else
            echo "ERROR: Implementation session already exists: $status_file"
            echo "Use --continue to resume the existing session"
        fi
        exit 1
    fi
    
    # Create initial status file
    cat > "$status_file" << EOF
# RFC-$RFC_ID_PADDED Implementation Status

**Session**: $session_id
**Started**: $(date -Iseconds)
**Last Updated**: $(date -Iseconds)
**Progress**: 0% (0/0 tasks)

## Current Phase: 1 - Foundation

## Completed Tasks
| Task | Completed | Duration | Notes |
|------|-----------|----------|-------|

## In Progress
| Task | Started | Status |
|------|---------|--------|

## Files Modified
| File | Task | Change Type |
|------|------|-------------|

## Checkpoints
| ID | Timestamp | Phase | Task | Notes |
|----|-----------|-------|------|-------|
EOF

    # Initialize checkpoints file
    echo '{"checkpoints":[],"last_checkpoint":null}' > "$(get_checkpoints_file)"
    
    if $JSON_MODE; then
        cat << JSONEOF
{
  "rfc_id": "RFC-$RFC_ID_PADDED",
  "session_id": "$session_id",
  "status_file": "$status_file",
  "current_phase": 1,
  "current_task": null,
  "completed_tasks": [],
  "pending_tasks": [],
  "blocked_tasks": [],
  "total_tasks": 0,
  "progress_percent": 0,
  "resume_point": null,
  "last_checkpoint": null
}
JSONEOF
    else
        echo "Implementation session initialized: $session_id"
        echo "Status file: $status_file"
    fi
}

# Get current status
get_status() {
    local status_file=$(get_status_file)
    local checkpoints_file=$(get_checkpoints_file)
    
    if [[ ! -f "$status_file" ]]; then
        if $JSON_MODE; then
            echo '{"error":"No implementation session found","hint":"Use --init to start"}'
        else
            echo "ERROR: No implementation session found for RFC-$RFC_ID_PADDED"
            echo "Use --init to start a new session"
        fi
        exit 1
    fi
    
    # Parse status file
    local session_id=$(grep "^\*\*Session\*\*:" "$status_file" | sed 's/.*:\s*//')
    local started=$(grep "^\*\*Started\*\*:" "$status_file" | sed 's/.*:\s*//')
    local last_updated=$(grep "^\*\*Last Updated\*\*:" "$status_file" | sed 's/.*:\s*//')
    local progress=$(grep "^\*\*Progress\*\*:" "$status_file" | sed 's/.*:\s*//')
    local current_phase=$(grep "^## Current Phase:" "$status_file" | sed 's/.*:\s*\([0-9]*\).*/\1/')
    
    # Get last checkpoint
    local last_checkpoint="null"
    if [[ -f "$checkpoints_file" ]]; then
        last_checkpoint=$(cat "$checkpoints_file" | grep -o '"last_checkpoint":[^,}]*' | sed 's/.*://')
    fi
    
    if $JSON_MODE; then
        cat << JSONEOF
{
  "rfc_id": "RFC-$RFC_ID_PADDED",
  "session_id": "$session_id",
  "status_file": "$status_file",
  "started": "$started",
  "last_updated": "$last_updated",
  "progress": "$progress",
  "current_phase": $current_phase,
  "last_checkpoint": $last_checkpoint
}
JSONEOF
    else
        echo "RFC-$RFC_ID_PADDED Implementation Status"
        echo "========================================"
        echo "Session: $session_id"
        echo "Started: $started"
        echo "Last Updated: $last_updated"
        echo "Progress: $progress"
        echo "Current Phase: $current_phase"
        echo "Status File: $status_file"
    fi
}

# Continue from last checkpoint
continue_session() {
    local status_file=$(get_status_file)
    
    if [[ ! -f "$status_file" ]]; then
        if $JSON_MODE; then
            echo '{"error":"No session to continue","hint":"Use --init to start"}'
        else
            echo "ERROR: No implementation session to continue"
        fi
        exit 1
    fi
    
    # Update last updated timestamp
    sed -i "s/\*\*Last Updated\*\*:.*/\*\*Last Updated\*\*: $(date -Iseconds)/" "$status_file"
    
    get_status
}

# Create checkpoint
create_checkpoint() {
    local phase="$1"
    local task="$2"
    local notes="$3"
    local checkpoint_id="cp-$RFC_ID_PADDED-$TIMESTAMP"
    local checkpoints_file=$(get_checkpoints_file)
    
    ensure_memory_dir
    
    # Add checkpoint to status file
    local status_file=$(get_status_file)
    echo "| $checkpoint_id | $(date -Iseconds) | $phase | $task | $notes |" >> "$status_file"
    
    if $JSON_MODE; then
        echo '{"checkpoint_id":"'"$checkpoint_id"'","phase":"'"$phase"'","task":"'"$task"'"}'
    else
        echo "Checkpoint created: $checkpoint_id"
    fi
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

case "$MODE" in
    init)
        init_session
        ;;
    continue)
        continue_session
        ;;
    status)
        get_status
        ;;
    resume)
        # TODO: Implement resume from specific checkpoint
        echo "Resume functionality not yet implemented"
        ;;
    rollback)
        # TODO: Implement rollback
        echo "Rollback functionality not yet implemented"
        ;;
    *)
        echo "ERROR: Unknown mode '$MODE'" >&2
        exit 1
        ;;
esac

exit 0
