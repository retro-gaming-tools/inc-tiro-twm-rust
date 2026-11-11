## The launcher

![The launcher](release/launcher.png)

The launcher opens before the game. Your choices are saved in `settings.ini` beside the program (beside the `.AppImage` file on Linux), and `--play` on the command line skips the launcher.

- **Game folder**: where your copy of the game is. Click *Choose...* or press Enter to pick it, or drop the folder onto the window. The line below it says whether the game files were found and are the right version (GOG, 1.52e).
- **3D resolution**: the size the 3D view is drawn at with the Glide renderer, from the original 640 x 480 to several higher resolutions. Menus and the interface stay at the game's 640 x 480 and are scaled to fit. With the Software renderer this row is greyed out, because that renderer always draws at 640 x 480.
- **Display**: *Window* or *Fullscreen*.
- **Text language**: *English* or *German*. It picks only the texts (menus, briefings, info screens); speech and cutscene voices don't change. The GOG version has both. The fan-made German patch adds German speech, which then plays with either setting.
- **Wilderness Missions**: *Off* starts the main campaign, with its intro. *On* starts the Wilderness Missions add-on.
- **Ask before giving up**: *On* asks Yes or No before *Give up!*, which ends the mission as lost. The original gives up at once on Escape, a key players tend to press expecting a menu. *Off* is the original behaviour.
- **Advanced settings...** opens a second page, described below.
- **Start game** and **Quit**.

### Advanced settings

The defaults on this page suit most machines. Change them if the game runs badly or looks wrong. *Back* or Escape returns to the first page.

- **3D drawn by**: *GPU (OpenGL)* (the default) draws the Glide renderer's 3D on the graphics card, with OpenGL 3.3. *CPU* draws it on the processor, where higher resolutions need a faster CPU. If the machine has no OpenGL 3.3, or only a software one, the CPU draws whatever this row says. With the Software renderer this row is greyed out.
- **Renderer**: *Glide* (the default) is the game's 3dfx renderer, drawn by this program without a 3dfx card or a Glide wrapper, at the 3D resolution on the first page. *Software* is the game's original software renderer, at 640 x 480.
- **Smooth frame pacing**: *On* (the default) shows frames at even intervals, in step with the display. *Off* runs the game's original frame limiter, which lets frames alternate between 16 and 33 ms.

The two lines at the bottom give hints, such as dropping the game folder on the window. In yellow, they warn about files whose names differ only in case, which on Linux usually means a patch was copied beside the files it should have replaced instead of over them. The game still starts.

The launcher works with the mouse, the keyboard (arrow keys to choose and change, Enter to confirm, Escape to quit) and a controller (d-pad to choose and change, A to confirm, Start to start the game, B to quit).
