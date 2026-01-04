#!/bin/bash
# Post-edit hook: Runs after Write/Edit/MultiEdit on Rust files
# Automatically formats code and runs clippy

set -e

# Read hook input from stdin
INPUT=$(cat)

# Extract the file path from the tool input
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // .tool_input.filePath // empty')

# Only process Rust files
if [[ "$FILE_PATH" == *.rs ]]; then
    cd "$CLAUDE_PROJECT_DIR"

    # Check if Cargo.toml exists (project is set up)
    if [[ -f "Cargo.toml" ]]; then
        # Format the specific file if rustfmt is available
        if command -v rustfmt &> /dev/null; then
            rustfmt "$FILE_PATH" 2>/dev/null || true
        fi

        # Run clippy on the file (quick check)
        if command -v cargo &> /dev/null; then
            # Only run clippy if it's not going to take too long
            timeout 30 cargo clippy --message-format=short 2>&1 | head -20 || true
        fi
    fi
fi

# Output result
echo '{"decision": "allow", "suppressOutput": true}'
