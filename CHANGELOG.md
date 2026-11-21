# Changelog

Newest first. Each version has a tag `v<version>`, and `./changes.sh <old> <new>` shows the full diff between two of them, such as `./changes.sh 0.4-beta 0.5-beta`.

## 0.6-beta (2026-11-21)

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
