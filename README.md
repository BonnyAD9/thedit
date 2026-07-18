# thedit
[![version][aur-badge]][aur]

**Terminal Hex EDITor** and hexdump with familiar controls for both vim and non-vim users.

<p align="center">
    <img src="https://github.com/user-attachments/assets/9b925fcc-fb0f-470d-8cb0-e110e4e47d85" />
</p>

## Controls

Thedit supports subset familiar vim controls (`hjkl` movement, visual mode, ...) and
also has great mouse support (scrolling, selection, dragging the scrollbar, ...).

When in visual mode, thedit will show decimal value of the selection in the bottom left
corner. The default endianness is Big Endian, but you can easily switch it to **Little
Endian** by typing `:le` or `S E` (**S**witch **E**ndianness) or by pressing the **mouse
back** button (you can see `LE` to the right of `VISUAL` in the bottom right, this means that
it has already been switched to Little Endian). To switch to **Big Endian** type `:be` (or
`S E`) or use the **mouse forward** button. By default the value is interpreted as unsigned
integer, you can change that by typing `S i` to go to **signed** mode or `S u` to go to
**unsigned** mode. When selecting with mouse, the signedness is determined by the button
you youse. **Left button** uses **unsigned** value and **right button** uses **signed**
value. To show the value without entering visual mode, you can type `S U` to show unsigned
value or `S I` to show signed value. This will by default interpret the next 4 bytes.
If you want different amount, prefix the command with that amount. You can also use the
commands `:uint`, `:int`, `:long`,... to do the same thing.

The numbers on the bottom right show the cursor position (`1,8` - line 1 character 8) and
the number of selected characters (`4`).

To **exit** type `:q` or press the **red cross in the top right**.

To see the full controls type `:help`.

## Formatting

**Colors** and the yellow header are automatically enabled when printing to
terminal. The meaning is:
- **WHITE**: Graphic ascii characters.
- **GREEN**: Whitespace.
- **GRAY**: NULL byte.
- **BLUE**: Other ascii characters (until `0x7F`)
- **CYAN**: Non ascii bytes (above `0x7F`)

Apart from colors, you can also enable utf graphic representation of ascii
control characters with the flag `--utf`:

<p align="center">
    <img src="https://github.com/user-attachments/assets/bd566411-c642-4000-b028-04267bfca022" />
</p>

The flag `--head` makes sure that only the first lines of the file that fit on the screen
will be printed. The flag `--head` also implies function of the flag `-d` which tells
thedit to do hexdump instead of opening the interactive editor. Thedit will also
automatically do hexdump if it detects that its stdout is not terminal.

For more features see `--help`.

## Features

Thedit now only works as interactive hex viewer or hexdump. I would like to make
this into full interactive terminal hex editor in the future.

## Configuration

Thedit uses configuration with lua. The root config file is located at
`thedit/lua/init.lua` in your config directory (on linux that usually is
`~/.config/thedit/lua/init.lua`).

You can use the table `thedit` for configuration. There the following members:

### `funcion thedit.cmd(cmd, cnt = nil)`
Execute the given command with the given number parameter.

`cnt` is nil or positive integer.

`cmd` is string which may be any of the following commands:
- `none`: Does nothing.
- `exit`: Quits thedit.
- `scroll-down`: Scroll down by one or the given amount.
- `scroll-up`: Scroll up by one or the given amount.
- `scroll-down-half`: Scroll down half the screen.
- `scroll-up-half`: Scroll up half the screen.
- `move-right`: Move the cursor right by one or cnt.
- `move-left`: Move the cursor left by one or cnt.
- `move-up`: Move the cursor up by one or cnt.
- `move-down`: Move the cursor down by one or cnt.
- `move-right-wrap`: Move the cursor right by one or cnt wrapping to the next
  line if needed.
- `move-left-wrap`: Move the cursor left by one or cnt wrapping to the previous
  line if needed.
- `scroll-to-view`: Scroll so that the cursor is in the view.
- `start-command`: Start long command.
- `move-to-top`: Move to the given line or to the start of the file.
- `move-to-bottom`: Move to the given line or to the end of the file.
- `view-signed`: View the next cnt or 4 bytes as signed value.
- `view-unsigned`: View the next cnt or 4 bytes as unsigned value.
- `swap-endianness`: Swap the endianness.
- `set-big-endian`: Set endianness to big endian.
- `set-little-endian`: Set endianness to little endian.
- `cancel`: Cancel the current command.
- `mode=<mode>`: Set the mode of terminal. `<mode>` may be one of:
    - `n`, `normal`: Normal mode.
    - `v`, `visual`: Visual mode.
- `visual-signed`: Set the value preview in visual mode to signed mode.
- `visual-unsigned`: Set the value preview in visual mode to unsigned mode.
- `move-pg-up`: Move one page up.
- `move-pg-down`: Move one page down.
- `scroll-pg-up`: Scroll one page up.
- `scroll-pg-down`: Scroll one page down.
- `move-to-start`: Move to the start of the current line.
- `move-to-end`: Move to the end of the current line.
- `show-help`: Show help.
- `enable-utf=<true|false>`: Enable/disable utf mode.

Example usage:
```lua
-- set mode to visual
thedit.cmd("mode=v")
-- move to the last line
thedit.cmd("move-to-bottom")
-- move to line 5
thedit.cmd("move-to-bottom", 5)
```

### `function thedit.map_key(modes, keys, action)`
Sets keybind.

`modes`: list of modes in which the keybind is active. Modes are the same as
in the command `mode=<mode>`.
If the one letter variant is used, the multiple values may be just the letters
concatenated. If the multiletter variants are used, you need to enclose each
mode in angle brackets. Spaces outside of angle brackets are ignored. (e.g.
`nv` is the same as `<normal><visual>` which is same as `<normal> <visual>`)

`keys`: list of keys that have to be pressed in order to trigger the command.
List is encoded the same way as modes. Each key may be prefixed with modifiers
(separated by dash `-`). If when using upper case ascii alphabetic key, shift
modifier is implicit. The following modifiers are available:
- `shift`, `S`: Shift key.
- `alt`, `A`: Alt key.
- `ctrl`, `control`, `C`: Control key.
- `meta`, `M`: Meta key (windows key).

The following keys are available:
- any letter except space (` `) or angle brackets (`<` and `>`): key
  representing that letter.
- `up`: Up arrow.
- `down`: Down arrow.
- `right`: Right arrow.
- `left`: Left arrow.
- `space`: Spacebar.
- `tab`: Tabulator key.
- `enter`: Enter key.
- `dash`: Key representing the character `-`.
- `f0` - `f20`: The function keys.
- `delete`: Delete key.
- `insert`: Insert key.
- `end`: End key.
- `home`: Home key.
- `pgup`, `pg_up`: Page up key.
- `pgdown`, `pg_down`: Page down key.
- `backspace`: Backspace key.
- `esc`: Escape key.

`action` is either command (same as `cmd` in `thedit.cmd`) or lua function with
one or zero arguments. If `action` is lua function. The argument passed to the
function is the number before the command.

Example usage:
```lua
-- The following do the same: on `gg` move to the given line or bottom
thedit.map_key("nv", "gg", "move-to-bottom")
thedit.map_key("nv", "g g", function(cnt)
    thedit.cmd("move-to-bottom", cnt)
end)

-- The following do the same: on `G` move to the given line or top
thedit.map_key("nv", "G", "move-to-start")
thedit.map_key("nv", "<S-g>", "move-to-start")
```

## Links
- **Author:** [BonnyAD9][author]
- **GitHub repository:** [BonnyAD9/uamp][github]
- **My website:** [bonnyad9.github.io][my-web]
- **Aur package:** [aur.archlinux.org][aur]


[author]: https://github.com/BonnyAD9
[github]: https://github.com/BonnyAD9/thedit
[my-web]: https://bonnyad9.github.io/
[aur]: https://aur.archlinux.org/packages/thedit
[aur-badge]: https://img.shields.io/aur/version/thedit