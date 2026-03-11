use pareg::{ArgInto, FromArg};

use crate::view::ctrl::cmd_key::CmdKey;

pub struct Keys(pub Vec<CmdKey>);

impl IntoIterator for Keys {
    type Item = CmdKey;

    type IntoIter = <Vec<CmdKey> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> FromArg<'a> for Keys {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        let mut res = vec![];
        for a in arg.trim().split_ascii_whitespace() {
            res.push(a.arg_into()?);
        }
        Ok(Self(res))
    }
}
