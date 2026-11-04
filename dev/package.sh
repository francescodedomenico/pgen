#!/usr/bin/env bash
# Builds the release binary for one target and packages it into target/dist/:
#   pgen-<version>-<label>.tar.gz   (macOS, Linux)
#   pgen-<version>-<label>.zip      (Windows)
# plus a .sha256 beside it. The archive holds the binary, LICENSE and README.md.
# Runs on the three GitHub runners (bash everywhere; Git Bash on Windows) and locally.
# Usage: bash dev/package.sh <version> <rust-target> <label>
#   e.g. bash dev/package.sh 0.2.0 aarch64-apple-darwin macos-aarch64
set -euo pipefail

version="${1:?version}"
target="${2:?rust target triple}"
label="${3:?artifact label, e.g. linux-x86_64}"
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

# The binary must carry the version being released.
manifest_version="$(bash dev/release-version.sh)"
[[ "$manifest_version" == "$version" ]] || {
  echo "::error::Cargo.toml says $manifest_version, asked to package $version"; exit 1; }

cargo build --release --locked --target "$target"

exe=""
[[ "$target" == *windows* ]] && exe=".exe"
bin="target/$target/release/pgen$exe"
[[ -x "$bin" ]] || { echo "::error::no binary at $bin"; exit 1; }

stage="target/dist/stage/pgen-$version-$label"
rm -rf "$stage"
mkdir -p "$stage" target/dist
cp "$bin" LICENSE README.md "$stage/"

cd target/dist
if [[ "$target" == *windows* ]]; then
  archive="pgen-$version-$label.zip"
  rm -f "$archive"
  7z a -bso0 -bsp0 "$archive" "./stage/pgen-$version-$label/"*
else
  archive="pgen-$version-$label.tar.gz"
  tar -C stage -czf "$archive" "pgen-$version-$label"
fi
rm -rf stage

# Checksum in `sha256sum -c` format; macOS has shasum, the others sha256sum.
if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$archive" > "$archive.sha256"
else
  shasum -a 256 "$archive" > "$archive.sha256"
fi
ls -l "$archive" "$archive.sha256"
