The first public release of Incubation Recompiled: Battle Isle: Incubation (Blue Byte, 1997) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need Battle Isle: Incubation from GOG** (the main menu shows version 1.52e). No game data is included.

## Downloads

- `incubation-rust-0.5-beta-windows-x64.zip`: unzip anywhere and start `incubation-rust.exe`. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `incubation-rust-0.5-beta-linux-x64.AppImage`: make it executable and start it.
- `incubation-rust-0.5-beta-SHA256SUMS.txt`: checksums of both.

Choose your game folder in the launcher, or drop the folder onto its window.

## Features

- The 3dfx Glide renderer, drawn on the CPU, at higher resolutions than the original, with no 3dfx card or Glide wrapper. The original software renderer is still available.
- A launcher for the game folder, resolution, window or fullscreen, renderer, text language and options, which checks that the game files are the right version.
- The Wilderness Missions add-on, started from the launcher.
- English or German texts, chosen in the launcher. A fan patch adds German speech; see `incubation-rust-README.txt` in the zip.
- CD music and cutscene soundtracks, from the GOG version's Ogg files, with a working music volume button.
- A controller works as a mouse.
- Even frame pacing with vsync, and an optional question before "Give up!".

## Status

This is a beta. Tested on Windows and Linux: the menus, the map, the first missions and a sample of later missions. Not every mission has been played through, so please report bugs in the issues. The picture is 4:3.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify incubation-rust-0.5-beta-linux-x64.AppImage --repo retro-gaming-tools/inc-tiro-twm-rust

You can also build the release yourself with `./build.sh`, and check the translated code against your own copy of the game with `./regenerate.sh /path/to/the/game`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by Ubisoft or Blue Byte. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
