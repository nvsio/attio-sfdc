#!/bin/bash
# Pre-tool hook: Validates bash commands before execution
# Ensures dangerous commands are blocked and provides guidance

set -e

# Read hook input from stdin
INPUT=$(cat)

# Extract the command from tool input
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# List of dangerous patterns to block
DANGEROUS_PATTERNS=(
    "rm -rf /"
    "rm -rf /*"
    ":(){ :|:& };:"     # Fork bomb
    "mkfs"
    "> /dev/sd"
    "dd if=/dev/zero"
    "chmod -R 777 /"
    "curl.*|.*bash"     # Piping curl to bash
    "wget.*|.*sh"       # Piping wget to shell
)

# Check for dangerous patterns
for pattern in "${DANGEROUS_PATTERNS[@]}"; do
    if echo "$COMMAND" | grep -qE "$pattern"; then
        echo '{"decision": "block", "reason": "Dangerous command pattern detected: '"$pattern"'"}'
        exit 2
    fi
done

# Warn about certain operations but allow them
WARN_PATTERNS=(
    "cargo publish"
    "wrangler deploy"
    "git push"
    "git push --force"
)

for pattern in "${WARN_PATTERNS[@]}"; do
    if echo "$COMMAND" | grep -qE "$pattern"; then
        # Allow but add a system message
        echo '{"decision": "allow", "systemMessage": "Running deployment/publish command: '"$pattern"'. Ensure you have reviewed the changes."}'
        exit 0
    fi
done

# Allow everything else
echo '{"decision": "allow"}'
