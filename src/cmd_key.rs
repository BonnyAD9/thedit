use termal::raw::events::{KeyCode, Modifiers};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct CmdKey {
    code: KeyCode,
    modifiers: Modifiers,
}