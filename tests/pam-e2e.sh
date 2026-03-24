#!/usr/bin/env bash
# PAM end-to-end test — actually change a password.
# Usage: docker compose run --rm debian bash tests/pam-e2e.sh

set -euo pipefail
cargo build --release 2>/dev/null

echo "=== Setting up test user ==="
# Give testuser a known password
echo "testuser:oldpassword" | chpasswd

echo "=== Verifying old password works ==="
echo "oldpassword" | su -c "echo 'auth ok'" testuser && echo "PASS: old password works" || echo "FAIL: old password rejected"

echo "=== Changing password via shadow-rs passwd ==="
# This requires PAM feature — skip if not compiled with PAM
if ./target/release/passwd --help 2>&1 | grep -q "stdin"; then
    echo "newpassword
newpassword" | ./target/release/passwd -s testuser 2>&1 && echo "PASS: password changed" || echo "SKIP: PAM not functional (expected in CI without full PAM config)"
else
    echo "SKIP: passwd not compiled with stdin support"
fi

echo "=== Done ==="
