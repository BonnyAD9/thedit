use std::{
    fs::File,
    io::{BufRead, BufReader, IsTerminal, stdin, stdout},
    process::ExitCode,
};

use pareg::Pareg;
use termal::{eprintacln, printmcln};

use crate::{
    cli::{Args, help},
    err::Result,
    utils::read_to_eof,
};

mod cli;
mod err;
//mod file_view;
mod print;
mod utils;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintacln!("{'r}error: {'_}{e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let mut args = Args::parse(Pareg::args())?;

    let Some(f) = args.file.take() else {
        if !args.helped() {
            let cin = stdin();
            if args.stdin() || !cin.is_terminal() {
                dump(cin.lock(), args)?;
            } else {
                eprintacln!(
                    "{'m}warning: {'_}Refusing to read from terminal. Use \
                    `{'y}--stdin{'_}` to force reading from terminal."
                );
                help();
            }
        }
        return Ok(());
    };

    if args.dump() {
        dump(BufReader::new(File::open(f)?), args)
    } else {
        eprintacln!(
            "{'m}warning: {'_}The default action may change in the future. \
            Use `{'y}-d{'_}` for hexdump."
        );
        dump(BufReader::new(File::open(f)?), args)
    }
}

fn dump(mut src: impl BufRead, args: Args) -> Result<()> {
    let color = stdout().is_terminal();
    if color {
        printmcln!(
            color,
            "          {'y}00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F   \
            {'y}01234567 89ABCDEF{'_}"
        );
    }

    let utf = args.utf();

    let target = args.count.get();
    let mut cur = 0;

    let mut pos = 0;
    let mut buf = [0; 16];

    let mut line = String::new();
    let mut red = 1;
    while red != 0 && cur < target {
        red = read_to_eof(&mut src, &mut buf)?;

        line.clear();
        print::line_num(&mut line, color, pos, 8);
        if red != 0 {
            line += "  ";
            print::hex_line(&mut line, color, &buf[..red], 8, 16);
            line += "  ";
            print::ascii_line(&mut line, color, &buf[..red], 8, 16, utf);
        }

        println!("{line}");
        pos += red;
        cur += 1;
    }

    Ok(())
}
