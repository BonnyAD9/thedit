use std::fmt::{Display, Write};

use pareg::{ArgInto, FromArg};

use crate::view::ctrl::{cmd_key::CmdKey, items::Items};

#[derive(Debug, Clone, Default)]
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
        for a in Items::new(arg) {
            res.push(a.arg_into()?);
        }
        Ok(Self(res))
    }
}

impl Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, key) in self.0.iter().enumerate() {
            if i != 0 {
                f.write_char(' ')?;
            }
            write!(f, "{key}")?;
        }

        Ok(())
    }
}
