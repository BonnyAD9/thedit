use std::io::{BufRead, IsTerminal, stdout};

use crate::{cli::Args, err::Result, print, utils::read_to_eof};

pub fn dump(mut src: impl BufRead, args: Args) -> Result<()> {
    let color = stdout().is_terminal();
    let mut line = String::new();
    if color {
        print::header(&mut line, color);
        println!("{line}");
    }

    let utf = args.utf();

    let target = args.count.get();
    let mut cur = 0;

    let mut pos = 0;
    let mut buf = [0; 16];

    let mut red = 1;
    while red != 0 && cur < target {
        red = read_to_eof(&mut src, &mut buf)?;

        line.clear();
        print::line_num(&mut line, color, pos, 8);
        if red != 0 {
            line += "  ";
            print::hex_line(&mut line, color, &buf[..red], 8, 16, None);
            line += "  ";
            print::ascii_line(&mut line, color, &buf[..red], 8, 16, utf, None);
        }

        println!("{line}");
        pos += red;
        cur += 1;
    }

    Ok(())
}
