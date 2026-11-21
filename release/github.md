Incubation Recompiled: Battle Isle: Incubation (Blue Byte, 1997) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need Battle Isle: Incubation from GOG** (the main menu shows version 1.52e). No game data is included.

## Changes in 0.6-beta

- The Glide renderer is drawn by the graphics card, through OpenGL 3.3, instead of by the CPU. The launcher's new row *3D drawn by* chooses between *GPU (OpenGL)* and *CPU*, and the program falls back to the CPU by itself where OpenGL 3.3 is missing or is only software.
- The launcher has a second page, *Advanced settings*, for what the defaults get right for nearly everyone: *3D drawn by* and *Renderer*. The first page keeps the folder, resolution, display, language and game options.
- The launcher has a new look, in the dark leather and metal of the game's own menus.
- A *Frame rate* row in the launcher: *30* (the default, evenly paced), *60* in missions (experimental: animations run twice as fast) or *Original*. It replaces *Smooth frame pacing*; old settings read as 30.
- **F9** in missions and on the map steps the frame rate through 30, 60 and Original, with a short note saying which.
- The question before "Give up!" is drawn in the game's own fonts and load-and-save window.
- The launcher warns when the English texts are German, as the German fan patch leaves them.
- The mouse stays inside the window while playing, and the game starts in fullscreen unless the launcher's Display row says Window.
- The README lists the keys in the game: **F11** for fullscreen or a window, **F10** (or RB) for the frame counter, **F9** for the frame rate.
- The program shows its version in its Windows file properties.

The full history is in `CHANGELOG.md`.

## Downloads

- `incubation-rust-0.6-beta-windows-x64.zip`: unzip anywhere and start `incubation-rust.exe`. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `incubation-rust-0.6-beta-linux-x64.AppImage`: make it executable and start it.
- `incubation-rust-0.6-beta-SHA256SUMS.txt`: checksums of both.

Choose your game folder in the launcher, or drop the folder onto its window.

## Features

- The 3dfx Glide renderer, drawn by the graphics card (or the CPU), at higher resolutions than the original, with no 3dfx card or Glide wrapper. The original software renderer is still available.
- A launcher for the game folder, resolution, window or fullscreen, renderer, text language and options, which checks that the game files are the right version.
- The Wilderness Missions add-on, started from the launcher.
- English or German texts, chosen in the launcher. A fan patch adds German speech; see `incubation-rust-README.txt` in the zip.
- CD music and cutscene soundtracks, from the GOG version's Ogg files, with a working music volume button.
- A controller works as a mouse.
- Even frame pacing with vsync, an optional 60 FPS in missions, and an optional question before "Give up!".

## Status

This is a beta. Tested on Windows and Linux: the menus, the map, the first missions and a sample of later missions. Not every mission has been played through, so please report bugs in the issues. The picture is 4:3.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify incubation-rust-0.6-beta-linux-x64.AppImage --repo retro-gaming-tools/inc-tiro-twm-rust

You can also build the release yourself with `./build.sh`, and check the translated code against your own copy of the game with `./regenerate.sh /path/to/the/game`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by Ubisoft or Blue Byte. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
