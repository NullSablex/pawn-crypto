#!/usr/bin/env bash
# Inventory the locked dependencies without assigning the project's license.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
mkdir -p dist

if ! command -v cargo-about >/dev/null 2>&1; then
  echo "[licenses] Install cargo-about: cargo install cargo-about --locked" >&2
  exit 1
fi

# Local package sources keep this reproducible and avoid fetching GitHub files.
# Missing or unaccepted licenses fail the command rather than being omitted.
cargo about generate --offline --locked --fail --format json \
  --output-file dist/licenses.json
cargo about generate --offline --locked --fail about.hbs \
  --output-file dist/THIRD-PARTY-LICENSES.html

echo "[licenses] Reports: dist/licenses.json and dist/THIRD-PARTY-LICENSES.html"
