use std::fmt::Display;

use pareg::FromArg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromArg, Hash)]
#[arg(exact)]
pub enum Mode {
    #[arg("n", "normal")]
    Normal,
    #[arg("v", "visual")]
    Visual,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::Normal => "normal",
            Mode::Visual => "visual",
        };
        f.write_str(s)
    }
}
