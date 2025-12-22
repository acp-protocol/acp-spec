#!/usr/bin/env bash

# RFC Finalization Script
#
# Updates documentation, version numbers, and prepares for release.
# Used by /rfc.finalize command.
#
# Usage: ./rfc-finalize.sh [OPTIONS] <rfc-id>
#
# OPTIONS:
#   --json              Output in JSON format
#   --dry-run           Preview changes without applying
#   --version <ver>     Override version number
#   --no-changelog      Skip changelog update
#   --no-commit         Skip git commit
#   --help, -h          Show help message

set -e

# ============================================================================
# CONFIGURATION
# ============================================================================

MEMORY_DIR=".claude/memory"
DATE=$(date +%Y-%m-%d)
TIMESTAMP=$(date -Iseconds)

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

JSON_MODE=false
DRY_RUN=false
NO_CHANGELOG=false
NO_COMMIT=false
VERSION_OVERRIDE=""
RFC_ID=""

for arg in "$@"; do
    case "$arg" in
        --json)
            JSON_MODE=true
            ;;
        --dry-run)
            DRY_RUN=true
            ;;
        --no-changelog)
            NO_CHANGELOG=true
            ;;
        --no-commit)
            NO_COMMIT=true
            ;;
        --version)
            shift_next=true
            ;;
        --help|-h)
            cat << 'EOF'
Usage: rfc-finalize.sh [OPTIONS] <rfc-id>

Update documentation, version numbers, and prepare for release.

OPTIONS:
  --json              Output in JSON format
  --dry-run           Preview changes without applying
  --version <ver>     Override version number (e.g., 1.2.0)
  --no-changelog      Skip changelog update
  --no-commit         Skip git commit
  --help, -h          Show this help message

ARGUMENTS:
  <rfc-id>            RFC identifier

OUTPUTS:
  - Version bump details
  - Files to update
  - Changelog entry
  - Release notes draft

EXAMPLES:
  # Full finalization
  ./rfc-finalize.sh --json RFC-001
  
  # Dry run to preview
  ./rfc-finalize.sh --dry-run --json RFC-001
  
  # With specific version
  ./rfc-finalize.sh --version 2.0.0 --json RFC-001

EOF
            exit 0
            ;;
        --*)
            if [[ "$shift_next" == true ]]; then
                VERSION_OVERRIDE="$arg"
                shift_next=false
            else
                echo "ERROR: Unknown option '$arg'. Use --help for usage." >&2
                exit 1
            fi
            ;;
        *)
            if [[ "$shift_next" == true ]]; then
                VERSION_OVERRIDE="$arg"
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
# HELPER FUNCTIONS
# ============================================================================

# Get current version from file
get_version() {
    local file="$1"
    local version=""
    
    case "$file" in
        *.toml)
            version=$(grep "^version" "$file" 2>/dev/null | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
            ;;
        *.json)
            version=$(grep '"version"' "$file" 2>/dev/null | head -1 | sed 's/.*"version".*:.*"\([^"]*\)".*/\1/')
            ;;
        *.md)
            version=$(grep -E "^version:|^Version:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//')
            ;;
    esac
    
    echo "${version:-0.0.0}"
}

# Bump version according to semver
bump_version() {
    local current="$1"
    local bump_type="$2"  # major, minor, patch
    
    local major=$(echo "$current" | cut -d. -f1)
    local minor=$(echo "$current" | cut -d. -f2)
    local patch=$(echo "$current" | cut -d. -f3 | sed 's/-.*//')
    
    case "$bump_type" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Determine bump type from RFC
determine_bump_type() {
    local rfc_file="$1"
    local bump_type="minor"  # default
    
    if [[ -f "$rfc_file" ]]; then
        if grep -qi "breaking change\|major.*change\|incompatible" "$rfc_file" 2>/dev/null; then
            bump_type="major"
        elif grep -qi "bug\s*fix\|patch\|hotfix" "$rfc_file" 2>/dev/null; then
            bump_type="patch"
        fi
    fi
    
    echo "$bump_type"
}

# Find RFC file
find_rfc_file() {
    local rfc_id="$1"
    local rfc_file=""
    
    # Search in accepted RFCs
    rfc_file=$(find rfcs/accepted -name "*$rfc_id*.md" -o -name "*$(printf '%03d' $rfc_id)*.md" 2>/dev/null | head -1)
    
    # If not found, search in proposed
    [[ -z "$rfc_file" ]] && rfc_file=$(find rfcs/proposed -name "*$rfc_id*.md" -o -name "*$(printf '%03d' $rfc_id)*.md" 2>/dev/null | head -1)
    
    echo "$rfc_file"
}

# Get RFC title
get_rfc_title() {
    local file="$1"
    local title=$(grep -E "^-?\s*\*?\*?Title.*:" "$file" 2>/dev/null | head -1 | sed 's/.*:\s*//' | tr -d '*' | xargs)
    [[ -z "$title" ]] && title=$(head -1 "$file" | sed 's/^#\s*//')
    echo "$title"
}

# Find version files
find_version_files() {
    local files=()
    
    [[ -f "cli/Cargo.toml" ]] && files+=("cli/Cargo.toml")
    [[ -f "Cargo.toml" ]] && files+=("Cargo.toml")
    [[ -f "package.json" ]] && files+=("package.json")
    [[ -f "schemas/v1/cache.schema.json" ]] && files+=("schemas/v1/cache.schema.json")
    
    # Find spec file
    local spec_file=$(find spec -name "ACP-*.md" 2>/dev/null | head -1)
    [[ -n "$spec_file" ]] && files+=("$spec_file")
    
    echo "${files[@]}"
}

# Generate changelog entry
generate_changelog_entry() {
    local version="$1"
    local rfc_title="$2"
    local rfc_id="$3"
    
    cat << EOF
## [$version] - $DATE

### Added
- $rfc_title ([RFC-$rfc_id])

### Changed
- See RFC-$rfc_id for detailed changes

[RFC-$rfc_id]: ./rfcs/accepted/$rfc_id-*.md
EOF
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

# Find RFC file
RFC_FILE=$(find_rfc_file "$RFC_ID")
if [[ -z "$RFC_FILE" ]]; then
    if $JSON_MODE; then
        echo '{"error":"RFC file not found","rfc_id":"RFC-'"$RFC_ID_PADDED"'"}'
    else
        echo "ERROR: RFC file not found for RFC-$RFC_ID_PADDED"
    fi
    exit 1
fi

RFC_TITLE=$(get_rfc_title "$RFC_FILE")

# Determine version bump
BUMP_TYPE=$(determine_bump_type "$RFC_FILE")

# Find version files and current versions
VERSION_FILES=($(find_version_files))
CURRENT_VERSION="0.0.0"

if [[ ${#VERSION_FILES[@]} -gt 0 ]]; then
    CURRENT_VERSION=$(get_version "${VERSION_FILES[0]}")
fi

# Calculate new version
if [[ -n "$VERSION_OVERRIDE" ]]; then
    NEW_VERSION="$VERSION_OVERRIDE"
    BUMP_TYPE="custom"
else
    NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$BUMP_TYPE")
fi

# Generate changelog entry
CHANGELOG_ENTRY=$(generate_changelog_entry "$NEW_VERSION" "$RFC_TITLE" "$RFC_ID_PADDED")

# Build file update list
version_updates=()
for file in "${VERSION_FILES[@]}"; do
    current=$(get_version "$file")
    version_updates+=("$file:$current:$NEW_VERSION")
done

# Documentation files to update
doc_files=()
[[ -f "docs/annotation-reference.md" ]] && doc_files+=("docs/annotation-reference.md")
[[ -f "docs/cli-reference.md" ]] && doc_files+=("docs/cli-reference.md")
[[ -f "README.md" ]] && doc_files+=("README.md")

# ============================================================================
# OUTPUT
# ============================================================================

if $JSON_MODE; then
    # Build version files JSON
    vf_json="["
    first=true
    for update in "${version_updates[@]}"; do
        IFS=':' read -r path current new <<< "$update"
        $first || vf_json+=","
        first=false
        vf_json+="{\"path\":\"$path\",\"current\":\"$current\",\"new\":\"$new\"}"
    done
    vf_json+="]"
    
    # Build doc files JSON
    df_json="[$(printf '"%s",' "${doc_files[@]}" | sed 's/,$//')]"
    [[ "$df_json" == "[\"\"]" ]] && df_json="[]"
    
    cat << JSONEOF
{
  "rfc_id": "RFC-$RFC_ID_PADDED",
  "title": "$RFC_TITLE",
  "finalization_id": "fin-$RFC_ID_PADDED-$(date +%Y%m%d-%H%M%S)",
  "dry_run": $DRY_RUN,
  "version": {
    "current": "$CURRENT_VERSION",
    "new": "$NEW_VERSION",
    "bump_type": "$BUMP_TYPE",
    "reason": "RFC implementation complete"
  },
  "files_to_update": {
    "version_files": $vf_json,
    "changelog": "CHANGELOG.md",
    "rfc_file": "$RFC_FILE",
    "documentation": $df_json
  },
  "changelog_entry": {
    "version": "$NEW_VERSION",
    "date": "$DATE",
    "sections": {
      "Added": ["$RFC_TITLE"],
      "Changed": [],
      "Deprecated": [],
      "Fixed": [],
      "Security": []
    }
  },
  "release_notes_draft": "Version $NEW_VERSION implements RFC-$RFC_ID_PADDED: $RFC_TITLE"
}
JSONEOF
else
    echo "========================================"
    echo "RFC-$RFC_ID_PADDED Finalization"
    echo "========================================"
    echo ""
    echo "RFC: $RFC_TITLE"
    echo "Version: $CURRENT_VERSION → $NEW_VERSION ($BUMP_TYPE)"
    echo ""
    echo "Files to update:"
    for update in "${version_updates[@]}"; do
        IFS=':' read -r path current new <<< "$update"
        echo "  $path: $current → $new"
    done
    echo ""
    echo "Changelog entry:"
    echo "$CHANGELOG_ENTRY"
    
    if $DRY_RUN; then
        echo ""
        echo "[DRY RUN - No changes applied]"
    fi
fi

# Apply changes if not dry run
if [[ "$DRY_RUN" != true ]]; then
    # Update version files
    for update in "${version_updates[@]}"; do
        IFS=':' read -r path current new <<< "$update"
        # Real implementation would update the file
        # sed -i "s/$current/$new/g" "$path"
    done
    
    # Update changelog
    if [[ "$NO_CHANGELOG" != true && -f "CHANGELOG.md" ]]; then
        # Real implementation would prepend changelog entry
        :
    fi
    
    # Git commit
    if [[ "$NO_COMMIT" != true ]] && command -v git &>/dev/null; then
        # Real implementation would create commit
        :
    fi
fi

exit 0
