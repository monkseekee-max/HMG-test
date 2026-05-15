#!/usr/bin/env sh
set -eu

REPO="monkseekee-max/HMG"
GIT_URL="https://github.com/${REPO}.git"
BIN_DIR="${HMG_INSTALL_DIR:-$HOME/.local/bin}"
TMP_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t hmg-install)"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT INT TERM

log() {
  printf '%s\n' "$*"
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1
}

target_triple() {
  os="$(uname -s 2>/dev/null || echo unknown)"
  arch="$(uname -m 2>/dev/null || echo unknown)"
  case "$os:$arch" in
    Linux:x86_64|Linux:amd64) echo "x86_64-unknown-linux-gnu" ;;
    Linux:aarch64|Linux:arm64) echo "aarch64-unknown-linux-gnu" ;;
    Darwin:x86_64) echo "x86_64-apple-darwin" ;;
    Darwin:arm64|Darwin:aarch64) echo "aarch64-apple-darwin" ;;
    *) echo "" ;;
  esac
}

install_from_release() {
  target="$(target_triple)"
  [ -n "$target" ] || return 1
  asset="hmg-${target}.tar.gz"
  url="https://github.com/${REPO}/releases/latest/download/${asset}"
  if ! curl -fsI "$url" >/dev/null 2>&1; then
    return 1
  fi

  log "Downloading HMG release: $asset"
  curl -fsSL "$url" -o "$TMP_DIR/$asset"
  tar -xzf "$TMP_DIR/$asset" -C "$TMP_DIR"
  mkdir -p "$BIN_DIR"
  for bin in hmg hmg-server hmg-hook-worker; do
    if [ -f "$TMP_DIR/$bin" ]; then
      install -m 0755 "$TMP_DIR/$bin" "$BIN_DIR/$bin"
    fi
  done
  return 0
}

install_from_cargo() {
  if ! need_cmd cargo; then
    log "Cargo/Rust toolchain not found."
    log "Install Rust first: https://rustup.rs/"
    log "Then rerun: curl -fsSL https://funcode.xin/HMG/install.sh | sh"
    return 1
  fi

  log "Installing HMG from GitHub with cargo..."
  cargo_root="$TMP_DIR/cargo-root"
  CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install --git "$GIT_URL" --root "$cargo_root" hmg-server --bins --force
  mkdir -p "$BIN_DIR"
  for bin in hmg hmg-server hmg-hook-worker; do
    if [ -f "$cargo_root/bin/$bin" ]; then
      install -m 0755 "$cargo_root/bin/$bin" "$BIN_DIR/$bin"
    fi
  done
}

main() {
  if install_from_release; then
    :
  else
    install_from_cargo
  fi

  log ""
  log "HMG installed."
  log "If hmg is not found, add this to your shell PATH:"
  log "  export PATH=\"$BIN_DIR:\$PATH\""
  log ""
  log "Next steps:"
  log "  hmg init -g"
  log "  hmg doctor"
  log "  codex"
  log ""
  log "Update later with:"
  log "  hmg update"
}

main "$@"
