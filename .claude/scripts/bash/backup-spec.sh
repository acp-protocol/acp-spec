#!/usr/bin/env bash

# Backup ACP spec files before remediation
#
# Usage: ./backup-spec.sh [OPTIONS]
#
# OPTIONS:
#   --output <dir>      Backup directory (default: .acp/backups/spec-TIMESTAMP)
#   --json              Output in JSON format
#   --help, -h          Show help message

set -e

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
OUTPUT_DIR=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --json)
            JSON_MODE=true
            shift
            ;;
        --help|-h)
            cat << 'EOF'
Usage: backup-spec.sh [OPTIONS]

Create backup of ACP spec files before remediation.

OPTIONS:
  --output <dir>      Backup directory (default: .acp/backups/spec-TIMESTAMP)
  --json              Output in JSON format
  --help, -h          Show this help message

OUTPUT:
  Creates timestamped backup of all spec files.
  Returns path to backup directory.

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

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
[[ -z "$OUTPUT_DIR" ]] && OUTPUT_DIR="$PROJECT_ROOT/.acp/backups/spec-$TIMESTAMP"

SPEC_DIR="$PROJECT_ROOT/spec"

# ============================================================================
# BACKUP
# ============================================================================

perform_backup() {
    # Create backup directory
    mkdir -p "$OUTPUT_DIR"

    # Track backed up files
    local backed_up=()
    local total_size=0

    # Backup main spec
    if [[ -f "$SPEC_DIR/ACP-1.0.md" ]]; then
        cp "$SPEC_DIR/ACP-1.0.md" "$OUTPUT_DIR/"
        backed_up+=("spec/ACP-1.0.md")
        total_size=$((total_size + $(stat -f%z "$SPEC_DIR/ACP-1.0.md" 2>/dev/null || stat -c%s "$SPEC_DIR/ACP-1.0.md" 2>/dev/null || echo 0)))
    fi

    # Backup chapters
    if [[ -d "$SPEC_DIR/chapters" ]]; then
        mkdir -p "$OUTPUT_DIR/chapters"
        for chapter in "$SPEC_DIR/chapters"/*.md; do
            if [[ -f "$chapter" ]]; then
                cp "$chapter" "$OUTPUT_DIR/chapters/"
                backed_up+=("spec/chapters/$(basename "$chapter")")
                total_size=$((total_size + $(stat -f%z "$chapter" 2>/dev/null || stat -c%s "$chapter" 2>/dev/null || echo 0)))
            fi
        done
    fi

    # Backup grammar files if present
    if [[ -d "$SPEC_DIR/grammar" ]]; then
        mkdir -p "$OUTPUT_DIR/grammar"
        for grammar in "$SPEC_DIR/grammar"/*; do
            if [[ -f "$grammar" ]]; then
                cp "$grammar" "$OUTPUT_DIR/grammar/"
                backed_up+=("spec/grammar/$(basename "$grammar")")
            fi
        done
    fi

    # Create manifest
    cat > "$OUTPUT_DIR/manifest.json" << EOF
{
  "timestamp": "$TIMESTAMP",
  "created": "$(date -Iseconds)",
  "source": "$SPEC_DIR",
  "files_count": ${#backed_up[@]},
  "total_size_bytes": $total_size,
  "files": $(printf '%s\n' "${backed_up[@]}" | jq -R . | jq -s .)
}
EOF

    # Output
    if $JSON_MODE; then
        cat << EOF
{
  "backup_dir": "$OUTPUT_DIR",
  "timestamp": "$TIMESTAMP",
  "files_backed_up": ${#backed_up[@]},
  "total_size_bytes": $total_size,
  "manifest": "$OUTPUT_DIR/manifest.json"
}
EOF
    else
        echo "Backup created: $OUTPUT_DIR"
        echo "Files backed up: ${#backed_up[@]}"
        echo "Total size: $total_size bytes"
    fi
}

# ============================================================================
# MAIN
# ============================================================================

if [[ ! -d "$SPEC_DIR" ]]; then
    if $JSON_MODE; then
        echo '{"error":"Spec directory not found","path":"'"$SPEC_DIR"'"}'
    else
        echo "ERROR: Spec directory not found: $SPEC_DIR" >&2
    fi
    exit 1
fi

perform_backup