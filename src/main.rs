use std::{
    fs::File,
    io::{BufReader, IsTerminal, stdin},
    process::ExitCode,
};

use pareg::Pareg;
use termal::eprintacln;

use crate::{
    cli::{Args, help},
    dump::dump,
    err::Result,
    file_view::FileView,
    view::view,
};

mod cli;
mod dump;
mod err;
mod file_view;
mod print;
mod utils;
mod view;

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
        view(FileView::new(File::open(f)?))
    }
}
