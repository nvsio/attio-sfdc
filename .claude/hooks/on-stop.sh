#!/bin/bash
# Stop hook: Runs when Claude finishes responding
# Ensures code quality before Claude stops working

set -e

cd "$CLAUDE_PROJECT_DIR"

# Skip if no Cargo.toml (project not initialized yet)
if [[ ! -f "Cargo.toml" ]]; then
    echo '{"decision": "allow"}'
    exit 0
fi

# Check if there are any Rust files to validate
if ! find src -name "*.rs" 2>/dev/null | head -1 | grep -q .; then
    echo '{"decision": "allow"}'
    exit 0
fi

ISSUES=""

# Check formatting
if command -v cargo &> /dev/null; then
    FMT_OUTPUT=$(cargo fmt --check 2>&1) || {
        ISSUES="$ISSUES\n- Code is not formatted. Run 'cargo fmt' to fix."
    }
fi

# Run clippy (quick mode)
if command -v cargo &> /dev/null; then
    CLIPPY_OUTPUT=$(timeout 60 cargo clippy --message-format=short 2>&1) || true
    if echo "$CLIPPY_OUTPUT" | grep -q "error\["; then
        CLIPPY_ERRORS=$(echo "$CLIPPY_OUTPUT" | grep "error\[" | head -5)
        ISSUES="$ISSUES\n- Clippy errors found:\n$CLIPPY_ERRORS"
    fi
fi

# Run tests (quick mode - only unit tests)
if command -v cargo &> /dev/null; then
    TEST_OUTPUT=$(timeout 120 cargo test --lib 2>&1) || {
        TEST_FAILURES=$(echo "$TEST_OUTPUT" | grep -A 2 "FAILED" | head -10)
        ISSUES="$ISSUES\n- Tests failing:\n$TEST_FAILURES"
    }
fi

# If there are issues, suggest Claude continue
if [[ -n "$ISSUES" ]]; then
    cat << EOF
{
    "decision": "allow",
    "continue": true,
    "systemMessage": "Quality checks found issues that should be addressed:\n$ISSUES\n\nPlease fix these issues before completing the task."
}
EOF
    exit 0
fi

# All good
echo '{"decision": "allow", "suppressOutput": true}'
