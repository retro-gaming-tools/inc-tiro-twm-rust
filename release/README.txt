Incubation Recompiled 0.5-beta
==============================

A native Windows and Linux version of Battle Isle: Incubation (Blue Byte, 1997).

Why this exists
---------------
Incubation is very hard to get working on modern machines. This project makes
it start and play on current Windows and Linux without compatibility modes,
Glide wrappers or other tinkering.

How it was made
---------------
The game's original Windows program was translated statically into native
64-bit code, which runs on a new runtime built on SDL3. The coding was done by
AI (Claude, by Anthropic). The testing was done by a human.

Source code
-----------
The complete source is published with each release: the translated code, the
runtime, the recompiler and the Docker build that makes these files. You can
build the release yourself, check that the code matches your own copy of the
game, and check that a downloaded file was built by GitHub from that source:
  gh attestation verify <file> --repo retro-gaming-tools/inc-tiro-twm-rust
Source and releases: https://github.com/retro-gaming-tools/inc-tiro-twm-rust

What you need
-------------
- Battle Isle: Incubation from GOG (the main menu shows version 1.52e).
  Other versions are not supported yet. No game data is included in this download.
- A 64-bit Windows or Linux PC. The 3D view is drawn by the graphics card if
  it has OpenGL 3.3, and otherwise on the CPU, where a reasonably fast CPU
  matters.

Installation
------------
1. Install the game from GOG.
2. Unzip this download anywhere, or into the game folder.
3. Start the program (the .exe on Windows, the .AppImage on Linux) and choose
   your game folder in the launcher, or drop the folder onto its window.

Your settings are saved in settings.ini beside the program.

Windows may warn that the program is from an unknown publisher, because it is
not code-signed. Choose "More info", then "Run anyway".

Playing in German
-----------------
The GOG version already has German texts: set Text language to German in
the launcher. Speech and cutscene voices stay English.

The German texts are older than the Wilderness Missions add-on. The add-on's
mission names and briefings, and some multiplayer texts, have no German
translation, so the program shows the English ones there.

For German speech, install the fan-made German patch. It adds German mission
speech, briefing voices and cutscene lip-sync. Download it from
  https://www.schote.biz/archiv-sprachdateien-deutschpatches/incubation
and see this GOG forum thread about it:
  https://www.gog.com/forum/battle_isle/iccubation_deutschgerman

1. Copy the patch's files into your game folder, replacing the existing ones,
   but leave out its DATA/TXT/English folder. That folder holds German texts
   again, and keeping the real English one lets the program fill the gaps.
2. In the launcher, set Text language to German. The patch replaces the
   speech files themselves, so its German speech plays with either setting.

The cutscene voices stay English even with the patch: they are the CD music
tracks, which the patch doesn't include.

On Linux, files whose names differ only in case can end up side by side
after copying, such as speech files. The program warns about them in its
output; keep only the patch's copy.

Features
--------
- The 3dfx Glide renderer, drawn by the graphics card with OpenGL 3.3 or on
  the CPU, at higher resolutions than the original. No 3dfx card or Glide
  wrapper needed. The original software renderer is still available.
- A launcher for the game folder, resolution, window or fullscreen, renderer
  and options. It checks that your game files are the right version.
- The Wilderness Missions add-on, which you start from the launcher.
- English or German texts, chosen in the launcher. A fan patch adds German
  speech.
- CD music and cutscene soundtracks, from the Ogg files of the GOG version.
  The music volume button in the options works.
- Even frame pacing with vsync.
- An optional question before "Give up!", which the original runs at once on
  Escape.
- A frame-time display on F10 or RB.

Controls
--------
Mouse and keyboard as in the original. A controller works as a mouse:
- Left stick: move the pointer. Right stick: move it slowly.
- A or right trigger: left click. B or left trigger: right click.
- Start: Escape. D-pad: arrow keys. RB: frame-time display.

Status
------
Tested on Windows and Linux: the menus, the map, the first missions and a
sample of later missions. Not every mission has been played through, so
please report bugs. The picture is 4:3; there is no widescreen.

Legal
-----
This is a fan project. It is not affiliated with or endorsed by Ubisoft or
Blue Byte. No game data is included; you need a legally bought copy of the
game. Battle Isle and Incubation are trademarks of their respective owners.
