#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="${HOME}/.local/bin"

cd "$ROOT_DIR"

echo "[axiom] building release binaries..."
cargo build --release -p axiom-cli -p axiom-interpreter

mkdir -p "$BIN_DIR"
cp "target/release/axiom-cli" "$BIN_DIR/axiom"
cp "target/release/axiom-interpreter" "$BIN_DIR/axiom-interpreter"
chmod +x "$BIN_DIR/axiom" "$BIN_DIR/axiom-interpreter"

echo "[axiom] installed:"
echo "  - $BIN_DIR/axiom"
echo "  - $BIN_DIR/axiom-interpreter"

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
  SHELL_RC="${HOME}/.profile"
  if [[ -n "${SHELL:-}" ]]; then
    case "$(basename "$SHELL")" in
      bash) SHELL_RC="${HOME}/.bashrc" ;;
      zsh) SHELL_RC="${HOME}/.zshrc" ;;
      fish) SHELL_RC="${HOME}/.config/fish/config.fish" ;;
    esac
  fi

  mkdir -p "$(dirname "$SHELL_RC")"
  if [[ "$SHELL_RC" == *"config.fish" ]]; then
    grep -q "set -gx PATH $BIN_DIR" "$SHELL_RC" 2>/dev/null || echo "set -gx PATH $BIN_DIR \$PATH" >> "$SHELL_RC"
  else
    grep -q "export PATH=\"$BIN_DIR:\$PATH\"" "$SHELL_RC" 2>/dev/null || echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_RC"
  fi
  echo "[axiom] added $BIN_DIR to PATH in $SHELL_RC"
fi

echo "[axiom] done. Open a new shell and run: axiom version"
