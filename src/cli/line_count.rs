use pareg::{ArgInto, FromArg};
use termal::raw::term_size;

#[derive(Debug, Copy, Clone, Default)]
pub enum LineCount {
    #[default]
    All,
    Auto,
    Exact(usize),
}

impl LineCount {
    pub fn get(&self) -> usize {
        match self {
            Self::All => usize::MAX,
            Self::Auto => term_size().map(|s| s.char_height - 2).unwrap_or(10),
            Self::Exact(n) => *n,
        }
    }
}

impl<'a> FromArg<'a> for LineCount {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        match arg {
            "auto" => Ok(LineCount::Auto),
            "all" => Ok(LineCount::All),
            _ => Ok(LineCount::Exact(arg.arg_into()?)),
        }
    }
}
