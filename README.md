# thedit

**Terminal Hex EDITor** and hexdump with familiar controls for both vim and non-vim users.

<p align="center">
    <img src="https://github.com/user-attachments/assets/9b925fcc-fb0f-470d-8cb0-e110e4e47d85" />
</p>

Thedit supports subset familiar vim controls (`hjkl` movement, visual mode, ...) and
also has great mouse support (scrolling, selection, dragging the scrollbar, ...).
The keybinds will be fully customizable in the future.

When in visual mode, thedit will show decimal value of the selection in the bottom left
corner. The default endianness is Big Endian, but you can easily switch it to Little
Endian by typing `:le` or `S E` (**S**witch **E**ndianness) or by pressing the mouse
back button (you can see `LE` to the right of `VISUAL` in the bottom right, this means that
it has already been switched to Little Endian). By default the value is interpreted as
unsigned integer, you can change that by typing `S i` to go to signed mode or `S u` to go
to unsigned mode. When selecting with mouse, the signedness is determined by the button
you youse. Left button uses unsigned value and right uses signed value. To show the value
without entering visual mode, you can type `S U` to show unsigned value or `S I` to show
signed value. This will by default interpret the next 4 bytes. If you want different
amount, prefix the command with that amount. You can also use the commands `:uint`, `:int`,
`:long`,... to do the same thing.

The numbers on the bottom right show the cursor position (`1,8` - line 1 character 8) and
the number of selected characters (`4`).

To exit type `:q` or press the red cross in the top right.

To see the full controls type `:help`.

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
    <img src="https://github.com/user-attachments/assets/ff617f2b-dca3-4c7a-a3f5-cfa648519983" />
</p>

The flag `--head` makes sure that only the first lines of the file that fit on the screen
will be printed. The flag `--head` also implies function of the flag `-d` which tells
thedit to do hexdump instead of opening the interactive editor. Thedit will also
automatically do hexdump if it detects that its stdout is not terminal.

For more features see `--help`.

Thedit now only works as interactive hex viewer or hexdump. I would like to make
this into full interactive terminal hex editor in the future.
