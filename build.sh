#!/usr/bin/env bash
# Builds the release with Docker: the Windows zip and the Linux AppImage go to
# dist/. Needs nothing but Docker.
set -euo pipefail
cd "$(dirname "$0")"
image=$(sed -n 's/^name = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)-build
docker build -t "$image" build
docker run --rm \
    --user "$(id -u):$(id -g)" \
    --env HOME=/tmp \
    ${SOURCE_DATE_EPOCH:+--env SOURCE_DATE_EPOCH="$SOURCE_DATE_EPOCH"} \
    --volume "$PWD:/src" \
    "$image" bash /src/build/build.sh
