use std::collections::HashMap;

use crate::view::ctrl::{cmd::Cmd, cmd_key::CmdKey};

#[derive(Debug, Clone, Default)]
pub struct KeyNode {
    pub cmd: Option<Cmd>,
    pub next: HashMap<CmdKey, usize>,
}

impl KeyNode {
    pub fn get(&self, key: CmdKey) -> Option<usize> {
        self.next.get(&key).copied()
    }
}
