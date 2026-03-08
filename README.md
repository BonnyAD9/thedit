# thedit

Terminal Hex EDITor. Right now only works as colorful hexdump. I would like to
make this into full interactive terminal hex editor in the future.

<img width="634" height="400" alt="image" src="https://github.com/user-attachments/assets/eadcffcb-0f14-4ea9-92f1-819b06b89cb6" />

**Colors** and the yellow header are automatically enabled when printing to terminal. The meaning is:
- **WHITE**: Graphic ascii characters.
- **GREEN**: Whitespace.
- **GRAY**: NULL byte.
- **BLUE**: Other ascii characters (until `0x7F`)
- **CYAN**: Non ascii bytes (above `0x7F`)

Apart from colors, you can also enable utf graphic representation of ascii control characters with the flat `--utf`:

<img width="633" height="401" alt="image" src="https://github.com/user-attachments/assets/ea3508d4-7311-4330-b290-936cfb72bddd" />
