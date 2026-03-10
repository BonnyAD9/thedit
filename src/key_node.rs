use std::collections::HashMap;

use crate::{cmd::Cmd, cmd_key::CmdKey};

#[derive(Debug)]
pub struct KeyNode {
    cmd: Option<Vec<Cmd>>,
    next: HashMap<CmdKey, KeyNode>,
}