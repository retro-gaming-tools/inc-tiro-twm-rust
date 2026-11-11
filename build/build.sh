#!/usr/bin/env bash
# Builds the release inside the image from build/Dockerfile: the Windows zip
# and the Linux AppImage, into dist/. The checkout is mounted at /src.
set -euo pipefail

cd /src
program=$(sed -n 's/^name = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)
version=$(tr -d '[:space:]' <VERSION)
title=$(sed -n 's/^# \(.*\), recompiled$/\1/p' README.md | head -n 1)
dist=/src/dist
mkdir -p "$dist"

# Paths and dates that would otherwise end up in the programs and archives
# are fixed, so two builds of one commit can come out the same.
export SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-$(git -c safe.directory=/src log -1 --format=%ct 2>/dev/null || echo 0)}
export RUSTFLAGS="--remap-path-prefix=/src=. --remap-path-prefix=$CARGO_HOME=cargo"
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

echo "== fetching crates"
cargo fetch --locked
echo "== building for Linux"
cargo build --release --offline --locked --target x86_64-unknown-linux-gnu
echo "== building for Windows"
cargo build --release --offline --locked --target x86_64-pc-windows-gnu

# Released programs carry no symbols.
stage=$(mktemp -d)
mkdir "$stage/$program"
x86_64-w64-mingw32-strip -o "$stage/$program/$program.exe" "target/x86_64-pc-windows-gnu/release/$program.exe"
cp /opt/sdl3-windows/bin/SDL3.dll "$stage/$program/"
# Named after the program, so they don't replace the game's own readme.txt
# when the zip is unpacked into the game folder.
cp release/README.txt "$stage/$program/$program-README.txt" 2>/dev/null || true
cp CHANGELOG.md "$stage/$program/$program-CHANGELOG.txt" 2>/dev/null || true
cp /opt/sdl3-windows/share/licenses/SDL3/LICENSE.txt "$stage/$program/SDL3-LICENSE.txt"
find "$stage" -exec touch -h -d "@$SOURCE_DATE_EPOCH" {} +
windows=$dist/$program-$version-windows-x64.zip
rm -f "$windows"
(cd "$stage" && find "$program" | LC_ALL=C sort | zip -q -X -@ "$windows")

appdir=$stage/AppDir
mkdir -p "$appdir/usr/bin" "$appdir/usr/lib"
strip -o "$appdir/usr/bin/$program" "target/x86_64-unknown-linux-gnu/release/$program"
cp -L /opt/sdl3/lib/libSDL3.so.0 "$appdir/usr/lib/"
mkdir -p "$appdir/usr/share/licenses/SDL3"
cp /opt/sdl3/share/licenses/SDL3/LICENSE.txt "$appdir/usr/share/licenses/SDL3/"
cp icon.png "$appdir/$program.png"
cp icon.png "$appdir/.DirIcon"
cat >"$appdir/$program.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=$title
Exec=$program
Icon=$program
Categories=Game;
EOF
# The game folder is looked for where the player started the AppImage.
cat >"$appdir/AppRun" <<EOF
#!/bin/sh
here=\$(dirname "\$(readlink -f "\$0")")
export LD_LIBRARY_PATH="\$here/usr/lib:\$LD_LIBRARY_PATH"
cd "\${OWD:-.}"
exec "\$here/usr/bin/$program" "\$@"
EOF
chmod +x "$appdir/AppRun"
find "$appdir" -exec touch -h -d "@$SOURCE_DATE_EPOCH" {} +
linux=$dist/$program-$version-linux-x64.AppImage
rm -f "$linux"
appimagetool --no-appstream --runtime-file /opt/appimage-runtime "$appdir" "$linux" >/dev/null
rm -rf "$stage"

(cd "$dist" && sha256sum "$(basename "$windows")" "$(basename "$linux")" >"$program-$version-SHA256SUMS.txt")
echo "== built"
cat "$dist/$program-$version-SHA256SUMS.txt"
