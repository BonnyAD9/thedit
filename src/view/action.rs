use crate::view::ctrl::{Cmd, Keys, Modes};

#[derive(Debug)]
pub enum Action {
    Cmd(Cmd, Option<usize>),
    SetKey(Modes, Keys, Cmd),
}
