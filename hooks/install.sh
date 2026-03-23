#!/usr/bin/env bash
# Install shadow-rs git hooks.
#
# Usage: ./hooks/install.sh
#
# This symlinks the hooks into .git/hooks/ so they run automatically
# on commit and push. No GitHub Actions, no cloud — everything local.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOKS_DIR="$REPO_ROOT/hooks"
GIT_HOOKS_DIR="$REPO_ROOT/.git/hooks"

for hook in pre-commit pre-push; do
    src="$HOOKS_DIR/$hook"
    dst="$GIT_HOOKS_DIR/$hook"

    if [ -f "$src" ]; then
        chmod +x "$src"
        ln -sf "$src" "$dst"
        echo "installed: $hook -> $dst"
    fi
done

echo ""
echo "Git hooks installed. They run in Docker — make sure images are built:"
echo "  docker compose build"
