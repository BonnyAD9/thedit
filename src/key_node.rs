use std::collections::HashMap;

use pareg::ArgInto;

use crate::{cmd::Cmd, cmd_key::CmdKey, keys::Keys};

#[derive(Debug, Clone, Default)]
pub struct KeyNode {
    cmd: Vec<Cmd>,
    next: HashMap<CmdKey, KeyNode>,
}

impl KeyNode {
    pub fn add_cmd(
        &mut self,
        keys: impl IntoIterator<Item = CmdKey>,
        cmd: Vec<Cmd>,
    ) {
        let mut keys = keys.into_iter();
        if let Some(key) = keys.next() {
            self.next.entry(key).or_default().add_cmd(keys, cmd);
        } else {
            self.cmd = cmd;
        }
    }

    pub fn get(&self, key: CmdKey) -> Option<&KeyNode> {
        self.next.get(&key)
    }

    pub fn cmd(&self) -> &[Cmd] {
        &self.cmd
    }

    pub fn default_controls() -> Self {
        let mut res = Self::default();

        fn p(s: &str) -> Keys {
            s.arg_into().unwrap()
        }

        res.add_cmd(p("j"), vec![Cmd::MoveDown]);
        res.add_cmd(p("down"), vec![Cmd::MoveDown]);
        res.add_cmd(p("k"), vec![Cmd::MoveUp]);
        res.add_cmd(p("up"), vec![Cmd::MoveUp]);
        res.add_cmd(p("h"), vec![Cmd::MoveLeft]);
        res.add_cmd(p("left"), vec![Cmd::MoveLeft]);
        res.add_cmd(p("l"), vec![Cmd::MoveRight]);
        res.add_cmd(p("right"), vec![Cmd::MoveRight]);
        res.add_cmd(p("ctrl-y"), vec![Cmd::ScrollUp]);
        res.add_cmd(p("ctrl-e"), vec![Cmd::ScrollDown]);
        res.add_cmd(p("ctrl-u"), vec![Cmd::ScrollUpHalf]);
        res.add_cmd(p("ctrl-d"), vec![Cmd::ScrollDownHalf]);
        res.add_cmd(p(":"), vec![Cmd::StartCommand]);

        res
    }
}
