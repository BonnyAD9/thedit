use std::path::PathBuf;

use pareg::Pareg;

use crate::{
    cli::{ArgFlags, help, line_count::LineCount},
    err::Result,
};

#[derive(Debug, Default)]
pub struct Args {
    pub file: Option<PathBuf>,
    pub count: LineCount,
    pub flags: ArgFlags,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Self::default();

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => {
                    help();
                    res.flags |= ArgFlags::HELPED;
                }
                "-i" | "--input" => res.file = Some(args.next_arg()?),
                "-d" | "--dump" => res.flags |= ArgFlags::DUMP,
                "-c" | "--count" => {
                    res.count = args.next_arg()?;
                    res.flags |= ArgFlags::DUMP;
                },
                "--head" => {
                    res.count = LineCount::Auto;
                    res.flags |= ArgFlags::DUMP;
                },
                "--utf" => res.flags |= ArgFlags::UTF,
                "--stdin" => {
                    res.file = None;
                    res.flags |= ArgFlags::STDIN | ArgFlags::DUMP
                }
                v if !v.starts_with('-') => res.file = Some(args.cur_arg()?),
                _ => return Err(args.err_unknown_argument().into()),
            }
        }

        Ok(res)
    }

    pub fn helped(&self) -> bool {
        self.flags.contains(ArgFlags::HELPED)
    }

    pub fn dump(&self) -> bool {
        self.flags.contains(ArgFlags::DUMP)
    }

    pub fn utf(&self) -> bool {
        self.flags.contains(ArgFlags::UTF)
    }

    pub fn stdin(&self) -> bool {
        self.flags.contains(ArgFlags::STDIN)
    }
}
