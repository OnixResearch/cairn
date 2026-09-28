#!/usr/bin/env bash
set -euo pipefail

# Import only the pinned, independently committed mechanism source; no remote is assumed.
source_git=${1:?usage: tools/import-radicle-cob-git-store.sh /path/to/source-git-repository}
revision=af38dae7e0454b1206512794420a3076ea5a8d1f
expected=ff1392cab83cb76123c56b0a70ab50fa0893788f5a876e4daf614003905a38e0
root=$(git -C "$(dirname -- "${BASH_SOURCE[0]}")/.." rev-parse --show-toplevel)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"; if [[ -n "${staged:-}" ]]; then rm -rf "$staged"; fi' EXIT

git -C "$source_git" cat-file -e "$revision^{commit}"
git -C "$source_git" archive --format=tar "$revision" > "$tmp/source.tar"
observed=$(b3sum "$tmp/source.tar")
observed=${observed%% *}
if [[ "$observed" != "$expected" ]]; then
  printf 'COB source archive digest mismatch: expected %s, observed %s\n' "$expected" "$observed" >&2
  exit 1
fi
tar -xf "$tmp/source.tar" -C "$tmp"
staged=$(mktemp -d "$root/vendor/.cob-import-XXXXXXXX")
destination="$root/vendor/radicle-cob-git-store"
mkdir -p "$staged/src"
cp "$tmp/Cargo.toml" "$staged/Cargo.toml"
cp "$tmp/src/lib.rs" "$staged/src/lib.rs"
cp "$tmp/LICENSE-MIT" "$staged/LICENSE-MIT"
cp "$tmp/LICENSE-APACHE" "$staged/LICENSE-APACHE"
chmod 755 "$staged" "$staged/src"
if [[ -e "$destination" ]]; then
  if diff -qr "$staged" "$destination" >/dev/null; then
    printf 'Already imported %s (archive BLAKE3 %s)\n' "$revision" "$expected"
    exit 0
  fi
  printf 'Existing vendor snapshot differs; refusing to overwrite %s\n' "$destination" >&2
  exit 1
fi
mv "$staged" "$destination"
printf 'Imported %s (archive BLAKE3 %s)\n' "$revision" "$expected"
