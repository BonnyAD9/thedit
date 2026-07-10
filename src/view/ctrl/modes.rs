use pareg::{ArgInto, FromArg};

use crate::view::ctrl::{Mode, items::Items};

pub struct Modes(pub Vec<Mode>);

impl<'a> FromArg<'a> for Modes {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        let mut res = vec![];
        for i in Items::new(arg) {
            res.push(i.arg_into()?);
        }
        Ok(Self(res))
    }
}

impl IntoIterator for Modes {
    type Item = Mode;
    type IntoIter = std::vec::IntoIter<Mode>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
