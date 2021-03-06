# autorandr

| | |
|---|---|
| ⚠️ | **Warning:** This project is no longer maintained. I found another project (with the same name - [autorandr](https://github.com/phillipberndt/autorandr)) that does everything I want which saves me a lot of time. It even supports the extra stuff under "Future plans" below. |

> Automatically assigns a layout to all connected screens using xrandr.

This small command will automatically make all your connected displays use their
highest resolution, and then the highest refresh rate for that resolution.

Then screens will be placed in a horizontal line with the biggest screen to the
left and the smallest screen to the right.

## Usage

Execute `autorandr` inside your Xorg session. You must have `xrandr` installed.

```
autorandr 1.0.0
Magnus Bergmark <magnus.bergmark@gmail.com>
Maximize resolution and refresh rate of all connected monitors and order them in an automatic layout. Currently the
largest screen is always placed leftmost with smaller screens going to the right.

USAGE:
    autorandr [FLAGS]

FLAGS:
    -n, --dry-run    Don't set resolution, instead print the xrandr command that would have been executed.
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## Installation

If you have a Rust toolchain installed you can install this by running

```bash
cargo install --git https://github.com/Mange/autorandr.git
```

You can also clone the repo and install using `cargo install` at the repo root.

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
