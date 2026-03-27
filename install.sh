#!/usr/bin/env bash
set -euo pipefail

REPO="${AXIOM_REPO:-axiom-lang/axiom}"
VERSION="${AXIOM_VERSION:-latest}"
INSTALL_ROOT="${AXIOM_INSTALL_ROOT:-$HOME/.local/axiom}"
BIN_DIR="$INSTALL_ROOT/bin"
FROM_SOURCE="${AXIOM_FROM_SOURCE:-0}"

log() { echo "[axiom] $*"; }

add_to_path() {
  local shell_rc
  if [[ ":$PATH:" == *":$BIN_DIR:"* ]]; then
    return 0
  fi

  shell_rc="${HOME}/.profile"
  if [[ -n "${SHELL:-}" ]]; then
    case "$(basename "$SHELL")" in
      bash) shell_rc="${HOME}/.bashrc" ;;
      zsh) shell_rc="${HOME}/.zshrc" ;;
      fish) shell_rc="${HOME}/.config/fish/config.fish" ;;
    esac
  fi

  mkdir -p "$(dirname "$shell_rc")"
  if [[ "$shell_rc" == *"config.fish" ]]; then
    grep -q "set -gx PATH $BIN_DIR" "$shell_rc" 2>/dev/null || echo "set -gx PATH $BIN_DIR \$PATH" >> "$shell_rc"
  else
    grep -q "export PATH=\"$BIN_DIR:\$PATH\"" "$shell_rc" 2>/dev/null || echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$shell_rc"
  fi

  log "added $BIN_DIR to PATH in $shell_rc"
}

install_from_source() {
  local root_dir
  root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  cd "$root_dir"

  log "building release binaries from source..."
  cargo build --release -p axiom-cli -p axiom-interpreter

  mkdir -p "$BIN_DIR"
  cp "target/release/axiom-cli" "$BIN_DIR/axiom"
  cp "target/release/axiom-interpreter" "$BIN_DIR/axiom-interpreter"
  chmod +x "$BIN_DIR/axiom" "$BIN_DIR/axiom-interpreter"

  add_to_path
  log "installed binaries in $BIN_DIR"
  log "done. Open a new shell and run: axiom version"
}

fetch_release_asset() {
  local api_url tmp_json
  if [[ "$VERSION" == "latest" ]]; then
    api_url="https://api.github.com/repos/$REPO/releases/latest"
  else
    api_url="https://api.github.com/repos/$REPO/releases/tags/$VERSION"
  fi

  tmp_json="$(mktemp)"
  curl -fsSL -H "User-Agent: axiom-installer" "$api_url" -o "$tmp_json"

  python3 - "$tmp_json" <<'PY'
import json, platform, sys
p = sys.argv[1]
with open(p, 'r', encoding='utf-8') as f:
    data = json.load(f)
assets = data.get('assets', [])
os_name = platform.system().lower()
arch = platform.machine().lower()

if os_name == 'darwin':
    os_keys = ['darwin', 'macos', 'apple']
elif os_name == 'linux':
    os_keys = ['linux']
else:
    os_keys = [os_name]

arch_keys = {
    'x86_64': ['x86_64', 'amd64'],
    'amd64': ['x86_64', 'amd64'],
    'aarch64': ['aarch64', 'arm64'],
    'arm64': ['aarch64', 'arm64'],
}.get(arch, [arch])

cands = []
for a in assets:
    name = a.get('name', '').lower()
    if not any(k in name for k in os_keys):
        continue
    if not any(k in name for k in arch_keys):
        continue
    if name.endswith('.tar.gz') or name.endswith('.tgz') or name.endswith('.zip'):
        cands.append(a)

if not cands:
    for a in assets:
        name = a.get('name', '').lower()
        if name.endswith('.tar.gz') or name.endswith('.tgz') or name.endswith('.zip'):
            cands.append(a)

if not cands:
    print('')
    sys.exit(0)

cands.sort(key=lambda a: (0 if a['name'].lower().endswith(('.tar.gz', '.tgz')) else 1, a['name']))
selected = cands[0]
print(selected.get('browser_download_url', ''))
print(selected.get('name', ''))
print(data.get('tag_name', 'unknown'))
PY

  rm -f "$tmp_json"
}

install_from_release() {
  local info url name tag tmpdir archive extract
  info="$(fetch_release_asset)"
  if [[ -z "$info" ]]; then
    return 1
  fi

  mapfile -t lines <<< "$info"
  url="${lines[0]:-}"
  name="${lines[1]:-}"
  tag="${lines[2]:-unknown}"

  if [[ -z "$url" ]]; then
    return 1
  fi

  tmpdir="$(mktemp -d)"
  archive="$tmpdir/$name"
  extract="$tmpdir/extract"

  mkdir -p "$extract" "$BIN_DIR"

  log "installing from release $tag: $name"
  curl -fsSL -H "User-Agent: axiom-installer" "$url" -o "$archive"

  if [[ "$name" == *.zip ]]; then
    if command -v unzip >/dev/null 2>&1; then
      unzip -q "$archive" -d "$extract"
    else
      python3 - "$archive" "$extract" <<'PY'
import sys, zipfile
archive, extract = sys.argv[1], sys.argv[2]
with zipfile.ZipFile(archive) as zf:
    zf.extractall(extract)
PY
    fi
  else
    tar -xzf "$archive" -C "$extract"
  fi

  local cli interp
  cli="$(find "$extract" -type f \( -name 'axiom-cli' -o -name 'axiom-cli.exe' \) | head -n1 || true)"
  interp="$(find "$extract" -type f \( -name 'axiom-interpreter' -o -name 'axiom-interpreter.exe' \) | head -n1 || true)"

  if [[ -z "$cli" || -z "$interp" ]]; then
    rm -rf "$tmpdir"
    return 1
  fi

  cp "$cli" "$BIN_DIR/axiom"
  cp "$interp" "$BIN_DIR/axiom-interpreter"
  chmod +x "$BIN_DIR/axiom" "$BIN_DIR/axiom-interpreter"

  rm -rf "$tmpdir"
  add_to_path
  log "installed binaries in $BIN_DIR"
  log "done. Open a new shell and run: axiom version"
}

if [[ "$FROM_SOURCE" == "1" ]]; then
  install_from_source
  exit 0
fi

if ! install_from_release; then
  log "release install failed or assets unavailable, falling back to source build"
  install_from_source
fi
