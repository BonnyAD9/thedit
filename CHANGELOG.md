# CHANGELOG

## future
### Changes
- Show the key codes properly.
- Optimize reading selectoin for parsing.
- Optimize drawing by allowing drawing the status bar independently.

## v0.1.0
### Features
- Hex view
- Ascii/utf view

#### CLI
- help
- hexdump
- interactive editor
- limit number of lines
- print only what fits
- utf view
- read from stdin

#### Interactive editor
- Lazy Loading (Only small portion of file is loaded in memory)
- BigEndian/LittleEndian mode
- NORMAL/VISUAL mode
- See currently selected values as decimal integer
- Scrollbar
- Exit button
- File position
- Number of selected bytes
- Exit
- Scrolling
- Moving
- Interpreting values as decimal
- Signed/unsigned value view
- Help
- **Mouse controls**:
    - `scroll up`/`scroll down`/`scrollbar drag`: Scrolling
    - `left click`/`right click`: Move cursor to the given position
    - `left drag`: Enter visual mode. Preview value as decimal unsigned.
    - `right drag`: Enter visual mode. Preview value as decimal signed.
    - `forward`/`back`: Change to little/big endian.
- **Key binds**:
    - `esc`: Cancel / go to NORMAL mode.
    - `h`/`left`: Move one character left, wrapping to previous line if needed.
    - `j`/`down`: Move one line down.
    - `k`/`up`: Move one line up.
    - `l`/`right`: Move one character right, wrapping to previous line if
      needed.
    - `ctrl-f`/`pg_down`: Move one page down.
    - `ctrl-b`/`pg_up`: Move one page up.
    - `$`/`end`: Move to the end of the line.
    - `_`/`home`: Move to the start of the line.
    - `G`/`ctrl-end`: Move to the given line or to the end of the file.
    - `g g`/`ctrl-home`: Move to the given line or to the start of the file.
    - `ctrl-e`/`ctrl-down`: Scroll one line down.
    - `ctrl-y`/`ctrl-up`: Scroll one line up.
    - `ctrl-pg_down`: Scroll one page down.
    - `ctrl-pg_up`: Scroll one page up.
    - `ctrl-d`: Scroll half page down.
    - `ctrl-u`: Scroll half page up.
    - `v`: Enter visual mode.
    - `S I`: Show signed integer.
    - `S U`: Show unsigned integer.
    - `S E`: Swap endianness.
    - `S i`: Change visual mode to signed.
    - `S u`: Change visual mode to unsigned.
    - `:`: Start command.
- **Commands**:
    - `:ascii`: Change right column mode to ascii.
    - `:utf`: Change right column mode to utf.
    - `:q`/`:x`/`:quit`/`:exit`: Exit the editor.
    - `:be`: Change the mode to Big Endian.
    - `:le`: Change the mode to Little Endian.
    - `:h`/`:help`: Show help.
    - `:sbyte`: Show one byte as signed decimal.
    - `:short`: Show two bytes as signed decimal.
    - `:int`: Show four bytes as signed decimal.
    - `:long`: Show eight bytes as signed decimal.
    - `:byte`: Show one byte as unsigned decimal.
    - `:ushort`: Show two bytes as unsigned decimal.
    - `:uint`: Show four bytes as unsigned decimal.
    - `:ulong`: Show eight bytes as unsigned decimal.