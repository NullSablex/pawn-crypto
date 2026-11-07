#!/usr/bin/env bash
# Builds the plugin for Windows and Linux from Windows (Git Bash).
#
# Outputs:
#   dist/<plugin>.dll — Windows (i686-pc-windows-msvc)
#   dist/<plugin>.so  — Linux   (i686-unknown-linux-gnu via WSL or Docker/cross)
#
# Always builds with SA-MP + native Open Multiplayer support.
#
# Linux (.so) build:
#   WSL    — autodetected. Requires Rust inside WSL (https://rustup.rs).
#   Docker — fallback when WSL is unavailable. Requires Docker Desktop + cross.
#   Force a mode with --wsl or --docker.
#
# Usage:
#   ./scripts/build-windows.sh              # autodetect
#   ./scripts/build-windows.sh --wsl        # force WSL for Linux
#   ./scripts/build-windows.sh --docker     # force Docker for Linux
#   PROFILE=dev ./scripts/build-windows.sh  # dev build (default: release)
#
# PLUGIN_NAME is read from the project's Cargo.toml.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
PROFILE="${PROFILE:-release}"
METADATA="$(cargo metadata --no-deps --format-version 1 --manifest-path "$ROOT_DIR/Cargo.toml")"
PLUGIN_NAME="$(jq -r '.packages[0].targets[] | select(.kind | index("cdylib")) | .name' <<< "$METADATA")"
TARGET_DIR="$(jq -r '.target_directory' <<< "$METADATA")"
ARTIFACT_PROFILE="$PROFILE"
[[ "$PROFILE" != "dev" ]] || ARTIFACT_PROFILE="debug"
cd "$ROOT_DIR"
LINUX_BUILD=""  # "wsl" | "docker" | "" (autodetect)

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[build] $*${NC}"; }
log_step() { echo -e "${YELLOW}[build] $*${NC}"; }
log_err()  { echo -e "${RED}[build] $*${NC}" >&2; }

for arg in "$@"; do
  case "$arg" in
    --wsl)    LINUX_BUILD="wsl" ;;
    --docker) LINUX_BUILD="docker" ;;
    *) log_err "Unknown argument: $arg"; exit 1 ;;
  esac
done

ensure_target() {
  if ! rustup target list --installed | rg -qx "$1"; then
    log_step "Installing target: $1"
    rustup target add "$1"
  fi
}

ensure_cross() {
  if ! command -v cross >/dev/null 2>&1; then
    log_step "Installing cross..."
    cargo install cross
  fi
}

build_windows() {
  local target="i686-pc-windows-msvc"
  ensure_target "$target"
  log_step "Building: $target"
  cargo build --locked --profile "$PROFILE" --target "$target"

  local src="$TARGET_DIR/$target/$ARTIFACT_PROFILE/${PLUGIN_NAME}.dll"
  local dst="$DIST_DIR/${PLUGIN_NAME}.dll"
  [[ -f "$src" ]] || { log_err "Artifact not found: $src"; exit 1; }
  cp "$src" "$dst"
  log_info "Windows: $dst"
}

build_linux_wsl() {
  local target="i686-unknown-linux-gnu"
  local wsl_root="/mnt${ROOT_DIR}"

  # Check the linker before starting the Linux build.
  if ! wsl bash -c "echo 'int main(){return 0;}' | gcc -m32 -x c - -o /dev/null" >/dev/null 2>&1; then
    log_err "WSL is missing 32-bit C support ('gcc -m32' cannot build)."
    log_err "Install it inside WSL, e.g.:  sudo apt install gcc-multilib g++-multilib"
    log_err "Or build the .so through Docker instead:  ./scripts/build-windows.sh --docker"
    exit 1
  fi

  log_step "Building: $target (via WSL)"
  wsl bash -c "rustup target add '$target' && cd '$wsl_root' && cargo build --locked --profile '$PROFILE' --target '$target'"

  local src="$TARGET_DIR/$target/$ARTIFACT_PROFILE/lib${PLUGIN_NAME}.so"
  local dst="$DIST_DIR/${PLUGIN_NAME}.so"
  [[ -f "$src" ]] || { log_err "Artifact not found: $src"; exit 1; }
  cp "$src" "$dst"
  log_info "Linux:   $dst"
}

build_linux_docker() {
  local target="i686-unknown-linux-gnu"
  ensure_target "$target"
  ensure_cross
  log_step "Building: $target (via cross/Docker)"
  cross build --locked --profile "$PROFILE" --target "$target"

  local src="$TARGET_DIR/$target/$ARTIFACT_PROFILE/lib${PLUGIN_NAME}.so"
  local dst="$DIST_DIR/${PLUGIN_NAME}.so"
  [[ -f "$src" ]] || { log_err "Artifact not found: $src"; exit 1; }
  cp "$src" "$dst"
  log_info "Linux:   $dst"
}

build_linux() {
  case "$LINUX_BUILD" in
    wsl)    build_linux_wsl ;;
    docker) build_linux_docker ;;
    *)
      if command -v wsl >/dev/null 2>&1; then
        log_step "WSL detected."
        build_linux_wsl
      elif command -v docker >/dev/null 2>&1; then
        log_step "Docker detected."
        build_linux_docker
      else
        log_err "Neither WSL nor Docker found. Install one or force a mode with --wsl/--docker."
        exit 1
      fi
      ;;
  esac
}

main() {
  mkdir -p "$DIST_DIR"
  log_info "Mode: SA-MP + native Open Multiplayer"
  build_windows
  build_linux
  "$ROOT_DIR/scripts/licenses.sh"
  log_info "Done: $DIST_DIR/"
}

main
