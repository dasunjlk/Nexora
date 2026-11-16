#!/usr/bin/env bash
# Build nexora.exe (cross, mingw), a portable zip and the NSIS installer into dist/.
#
#   tools/package.sh                 # makensis must be on PATH
#   NSIS_HOST=user@host tools/package.sh   # build the installer on a Linux box with nsis
#
# Optional: SELFTEST_HOST=user@host also runs `nexora.exe --selftest` under Wine + Xvfb
# there and copies the screenshots back to dist/selftest/.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="$(grep -m1 '^version' "$ROOT/Cargo.toml" | sed -E 's/.*"(.*)".*/\1/')"
export PATH="$HOME/.cargo/bin:$PATH"

cd "$ROOT"
cargo test -p nexora-core --quiet
cargo build -p nexora --release --target x86_64-pc-windows-gnu
EXE="$ROOT/target/x86_64-pc-windows-gnu/release/nexora.exe"

mkdir -p "$ROOT/dist"
cp "$EXE" "$ROOT/dist/nexora.exe"
# Portable build inside its own folder, so nexora.exe never sits loose next to
# other downloads (Windows looks for some DLLs in the exe's folder first).
( cd "$ROOT/dist" && rm -rf Nexora "Nexora-$VERSION-portable.zip" && mkdir Nexora && cp nexora.exe "$ROOT/THIRD-PARTY-NOTICES.txt" Nexora/ && zip -qr "Nexora-$VERSION-portable.zip" Nexora && rm -rf Nexora )

if [[ -n "${NSIS_HOST:-}" ]]; then
  REMOTE_DIR="nexora-pkg"
  ssh -o BatchMode=yes "$NSIS_HOST" "rm -rf ~/$REMOTE_DIR && mkdir -p ~/$REMOTE_DIR/installer/art ~/$REMOTE_DIR/crates/nexora/res ~/$REMOTE_DIR/dist"
  scp -q -o BatchMode=yes "$ROOT/installer/nexora.nsi" "$NSIS_HOST:$REMOTE_DIR/installer/"
  scp -q -o BatchMode=yes "$ROOT/installer/art/nsis-welcome.bmp" "$ROOT/installer/art/nsis-header.bmp" "$NSIS_HOST:$REMOTE_DIR/installer/art/"
  scp -q -o BatchMode=yes "$ROOT/crates/nexora/res/nexora.ico" "$NSIS_HOST:$REMOTE_DIR/crates/nexora/res/"
  scp -q -o BatchMode=yes "$ROOT/THIRD-PARTY-NOTICES.txt" "$NSIS_HOST:$REMOTE_DIR/"
  scp -q -o BatchMode=yes "$EXE" "$NSIS_HOST:$REMOTE_DIR/nexora.exe"
  ssh -o BatchMode=yes "$NSIS_HOST" "cd ~/$REMOTE_DIR && makensis -V2 -DEXE=\$HOME/$REMOTE_DIR/nexora.exe -DOUTDIR=\$HOME/$REMOTE_DIR/dist installer/nexora.nsi"
  scp -q -o BatchMode=yes "$NSIS_HOST:$REMOTE_DIR/dist/Nexora-Setup-$VERSION.exe" "$ROOT/dist/"
elif command -v makensis >/dev/null; then
  makensis -V2 -DEXE="$EXE" -DOUTDIR="$ROOT/dist" "$ROOT/installer/nexora.nsi"
else
  echo "makensis not found and NSIS_HOST not set: skipping the installer" >&2
fi

if [[ -n "${SELFTEST_HOST:-}" ]]; then
  ssh -o BatchMode=yes "$SELFTEST_HOST" "mkdir -p ~/nexora-test"
  scp -q -o BatchMode=yes "$EXE" "$SELFTEST_HOST:nexora-test/nexora.exe"
  ssh -o BatchMode=yes "$SELFTEST_HOST" 'cd ~/nexora-test && rm -rf st-out && export WINEPREFIX=$HOME/nexora-test/wp WINEDEBUG=-all && timeout 180 xvfb-run -a -s "-screen 0 1920x1080x24" wine ~/nexora-test/nexora.exe --selftest st-out; echo "selftest exit=$?"'
  rm -rf "$ROOT/dist/selftest" && mkdir -p "$ROOT/dist/selftest"
  scp -q -r -o BatchMode=yes "$SELFTEST_HOST:nexora-test/st-out/*" "$ROOT/dist/selftest/" || true
fi

ls -la "$ROOT/dist"
