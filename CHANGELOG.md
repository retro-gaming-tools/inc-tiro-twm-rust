# Changelog

Newest first. Each version has a tag `v<version>`, and `./changes.sh <old> <new>` shows the full diff between two of them, such as `./changes.sh 0.4-beta 0.5-beta`.

## 0.5-beta (2026-11-09)

The first public release.

- Battle Isle: Incubation (GOG, version 1.52e) as a native 64-bit Windows and Linux program, translated statically from the original into Rust.
- The 3dfx Glide renderer, drawn on the CPU, at higher resolutions than the original. The original software renderer is still available.
- A launcher for the game folder, resolution, window or fullscreen, renderer, text language and options, which checks that the game files are the right version, and shows the version.
- The Wilderness Missions add-on, started from the launcher.
- English or German texts, both in the GOG version. The program fills gaps in the German texts, such as the add-on's briefings, from the English ones. A fan patch adds German speech.
- CD music and cutscene soundtracks from the GOG version's Ogg files, with a working music volume button.
- A controller works as a mouse.
- Even frame pacing with vsync, also in the software renderer's credits.
- An optional question before "Give up!", which the original runs at once on Escape.
- A frame-time display on F10 or RB.
- The source, the recompiler and the Docker build are published with it: `./build.sh` builds the release, and `./regenerate.sh` checks the code against your copy of the game.
- The Glide renderer is drawn by the graphics card, through OpenGL 3.3, instead of by the CPU. The launcher's new row *3D drawn by* chooses between *GPU (OpenGL)* and *CPU*, and the program falls back to the CPU by itself where OpenGL 3.3 is missing or is only software.
- The launcher has a second page, *Advanced settings*, for what the defaults get right for nearly everyone: *3D drawn by*, *Renderer* and *Smooth frame pacing*. The first page keeps the folder, resolution, display, language and game options.
