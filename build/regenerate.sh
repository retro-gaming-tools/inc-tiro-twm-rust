#!/usr/bin/env bash
# Runs inside the image from build/Dockerfile; see ../regenerate.sh.
set -euo pipefail

config=/src/recomp.json
setting() { jq -r "$1" "$config"; }

# The recompiler builds against the runtime beside it, as in the tool.
mkdir -p /recomp /generated /out
cp -r /src/recompiler/. /recomp/
cp -r /src/runtime /recomp/runtime
ln -s /src /port
cp -r /src/mods /generated/mods
[[ -d /src/release ]] && cp -r /src/release /generated/release

export CARGO_HOME=/opt/cargo
echo "== building the recompiler"
cargo fetch --quiet --locked --manifest-path /recomp/Cargo.toml
cargo build --quiet --release --offline --locked --manifest-path /recomp/Cargo.toml --target-dir /out/target

mapfile -t files < <(setting '.recompFiles // {} | to_entries[] | "/game/\(.key)=\(.value)"')
export RECOMP_VERSION RECOMP_REPO RECOMP_PATCHES RECOMP_SWITCHES RECOMP_RENDERERS RECOMP_LANGUAGES RECOMP_MUSIC RECOMP_REGISTRY RECOMP_CD
RECOMP_REGISTRY=$(setting '.recompRegistry // [] | join(";")')
RECOMP_CD=$(setting '.recompCd // ""')
RECOMP_VERSION=$(setting '.recompVersion // ""')
RECOMP_REPO=$(setting '.recompRepo // ""')
RECOMP_PATCHES=$(setting '.recompPatches // [] | join(";")')
RECOMP_SWITCHES=$(setting '.recompSwitches // {} | to_entries | map("\(.key)=\(.value)") | join(";")')
RECOMP_RENDERERS=$(setting '.recompRenderers // [] | join(";")')
RECOMP_LANGUAGES=$(setting '.recompLanguages // [] | join(";")')
RECOMP_MUSIC=$(setting '.recompMusic // [] | join(";")')

echo "== recompiling your copy of the game"
/out/target/release/recomp "$(setting .name)" "$(setting '.title // .name')" \
    "$(setting '.needs // "your own copy of the game"')" "$(setting '.recompCommand // "game.exe"')" \
    "${files[@]}"

echo "== comparing with this repository"
status=0
for path in src runtime Cargo.toml build.rs VERSION README.md icon.rc .gitignore; do
    if ! diff -r -q "/generated/$path" "/src/$path"; then
        status=1
    fi
done
if [[ $status -eq 0 ]]; then
    echo "No differences: the code here is exactly what the recompiler makes from your game."
else
    echo "The files above differ from what the recompiler makes from your game."
fi
exit $status
