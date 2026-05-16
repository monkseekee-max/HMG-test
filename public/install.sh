#!/usr/bin/env sh
set -eu

REPO="monkseekee-max/HMG"
GIT_URL="https://github.com/${REPO}.git"
HMG_RELEASE_BASE_URL="${HMG_RELEASE_BASE_URL:-https://funcode.xin/HMG/releases/latest/download}"
GITHUB_RELEASE_BASE_URL="https://github.com/${REPO}/releases/latest/download"
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

supported_targets() {
  log "Supported prebuilt packages:"
  log "  hmg-x86_64-unknown-linux-gnu.tar.gz"
  log "  hmg-aarch64-unknown-linux-gnu.tar.gz"
  log "  hmg-x86_64-apple-darwin.tar.gz"
  log "  hmg-aarch64-apple-darwin.tar.gz"
  log "Windows PowerShell uses: hmg-x86_64-pc-windows-gnu.zip"
}

install_from_release_url() {
  asset="$1"
  base_url="$2"
  url="${base_url%/}/${asset}"
  package_dir="$TMP_DIR/package"

  rm -rf "$package_dir"
  mkdir -p "$package_dir"

  log "Trying HMG release: $url"
  if ! curl -fL --retry 3 --retry-delay 1 --connect-timeout 20 "$url" -o "$TMP_DIR/$asset"; then
    log "Release unavailable or download failed: $url"
    return 1
  fi

  if ! tar -xzf "$TMP_DIR/$asset" -C "$package_dir"; then
    log "Downloaded release is not a valid tar.gz package: $url"
    return 1
  fi

  for bin in hmg hmg-server hmg-hook-worker; do
    if [ ! -f "$package_dir/$bin" ]; then
      log "Release package is missing required binary: $bin"
      return 1
    fi
  done

  mkdir -p "$BIN_DIR"
  for bin in hmg hmg-server hmg-hook-worker; do
    install -m 0755 "$package_dir/$bin" "$BIN_DIR/$bin"
  done
  return 0
}

install_from_release() {
  os="$(uname -s 2>/dev/null || echo unknown)"
  arch="$(uname -m 2>/dev/null || echo unknown)"
  target="$(target_triple)"
  if [ -z "$target" ]; then
    log "Unsupported platform for prebuilt install: $os/$arch"
    supported_targets
    return 1
  fi

  asset="hmg-${target}.tar.gz"
  log "Detected platform: $os/$arch -> $target"

  for base_url in "$HMG_RELEASE_BASE_URL" "$GITHUB_RELEASE_BASE_URL"; do
    [ -n "$base_url" ] || continue
    if install_from_release_url "$asset" "$base_url"; then
      return 0
    fi
  done

  log "No prebuilt HMG release package found for $target."
  supported_targets
  return 1
}

install_from_cargo() {
  if ! need_cmd cargo; then
    log "Cargo/Rust toolchain not found."
    log "Install Rust first: https://rustup.rs/"
    log "Then rerun: curl -fsSL https://funcode.xin/HMG/install.sh | sh"
    return 1
  fi

  log "No prebuilt HMG binary was found for this platform. Falling back to source install."
  log "Source install requires Rust and access to ${GIT_URL}."
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
