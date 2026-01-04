#!/bin/bash
# Session start hook: Provides context when Claude begins working
# Outputs relevant project state information

set -e

cd "$CLAUDE_PROJECT_DIR"

# Read hook input
INPUT=$(cat)
SOURCE=$(echo "$INPUT" | jq -r '.source // "startup"')

# Build context message
CONTEXT=""

# Check project initialization status
if [[ ! -f "Cargo.toml" ]]; then
    CONTEXT="$CONTEXT\nProject Status: NOT INITIALIZED - Cargo.toml does not exist. Run 'cargo init' or create the project structure first."
else
    CONTEXT="$CONTEXT\nProject Status: Initialized"

    # Get build status
    if command -v cargo &> /dev/null; then
        if cargo check 2>/dev/null; then
            CONTEXT="$CONTEXT\nBuild Status: Compiles successfully"
        else
            CONTEXT="$CONTEXT\nBuild Status: HAS ERRORS - run 'cargo check' to see issues"
        fi
    fi

    # Check test status (quick)
    if [[ -d "tests" ]] || find src -name "*_test.rs" 2>/dev/null | head -1 | grep -q .; then
        TEST_COUNT=$(cargo test --lib 2>&1 | grep -E "^test result:" | head -1 || echo "unknown")
        CONTEXT="$CONTEXT\nTest Status: $TEST_COUNT"
    fi
fi

# Check git status
if [[ -d ".git" ]]; then
    BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
    UNCOMMITTED=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
    CONTEXT="$CONTEXT\nGit Branch: $BRANCH"
    CONTEXT="$CONTEXT\nUncommitted Changes: $UNCOMMITTED files"
fi

# Development phase detection
if [[ -f "src/lib.rs" ]]; then
    if grep -q "todo!" src/**/*.rs 2>/dev/null; then
        TODO_COUNT=$(grep -r "todo!" src/ 2>/dev/null | wc -l | tr -d ' ')
        CONTEXT="$CONTEXT\nTODO Items: $TODO_COUNT remaining"
    fi
fi

# Output the context as stdout (will be added to conversation)
if [[ -n "$CONTEXT" ]]; then
    echo "=== Project Context ===$CONTEXT"
    echo ""
fi

exit 0
