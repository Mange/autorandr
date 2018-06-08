# autorandr

> Automatically assigns a layout to all connected screens using xrandr.

This small command will automatically make all your connected displays use their
highest resolution, and then the highest refresh rate for that resolution.

Then screens will be placed in a horizontal line with the biggest screen to the
left and the smallest screen to the right.

## Future plans

I generally only work with machines with either 1 or 2 screens connected, and
always in a horizontal fashion. It happens that I want to plug into a projector,
but then I manually set resolution and screen config rather than going through
this tool.

However, if the need ever arises (either because my setup changes, or because
other people find this utility useful) then some other features could be worked
on:

* Allow smarter layouts, like "Biggest in the center, smaller around it"
* Allow layout presets to be saved; "If these displays are connected, use layout
  X, otherwise pick something automatically".
  
Let me know if you are interested!

## Copyright and license

The code is released under the MIT license.

Copyright (c) 2018 Magnus Bergmark.
