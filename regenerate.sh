#!/usr/bin/env bash
# Checks the published code against your own copy of the game: builds the
# recompiler in recompiler/, runs it on the game folder you name with the
# settings in recomp.json, and compares what it writes with this repository.
# Usage: ./regenerate.sh /path/to/the/game. Needs nothing but Docker.
set -euo pipefail
cd "$(dirname "$0")"
[[ $# -eq 1 && -d $1 ]] || { echo "Usage: $0 /path/to/the/game" >&2; exit 2; }
game=$(cd "$1" && pwd)
image=$(sed -n 's/^name = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)-build
docker build -t "$image" build
# The recompiler writes to fixed paths (/recomp, /port, /generated, /out)
# inside the container, so this runs as root there, with the repository and
# the game mounted read-only. Nothing outside the container is written.
docker run --rm \
    --volume "$PWD:/src:ro" \
    --volume "$game:/game:ro" \
    "$image" bash /src/build/regenerate.sh
