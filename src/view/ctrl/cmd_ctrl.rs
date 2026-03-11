use pareg::ArgInto;
use termal::raw::events::KeyCode;

use crate::view::ctrl::{Cmd, CmdKey, Keys, key_node::KeyNode};

#[derive(Debug, Clone, Default)]
pub struct CmdCtrl {
    nodes: Vec<KeyNode>,
    cur: usize,
    num: Option<usize>,
}

impl CmdCtrl {
    pub fn add_cmd(
        &mut self,
        keys: impl IntoIterator<Item = CmdKey>,
        cmd: Cmd,
    ) {
        if self.nodes.is_empty() {
            self.nodes.push(KeyNode::default());
        }
        let mut cur = 0;
        for k in keys {
            let new = self.nodes.len();
            cur = *self.nodes[cur].next.entry(k).or_insert(new);
            if cur == new {
                self.nodes.push(KeyNode::default());
            }
        }
        self.nodes[cur].cmd = Some(cmd);
    }

    pub fn type_key(
        &mut self,
        key: CmdKey,
    ) -> Option<(Option<Cmd>, Option<usize>)> {
        if self.cur == 0
            && let KeyCode::Char(c) = key.code
            && let Some(d) = c.to_digit(10)
        {
            // Numbers before command.
            self.num = Some(self.num.unwrap_or_default() * 10 + d as usize);
            return None;
        }

        let Some(n) = self.nodes[self.cur].get(key) else {
            // Unknown command.
            let num = self.num;
            self.cancel();
            return Some((None, num));
        };

        let Some(cmd) = self.nodes[n].cmd else {
            // Command not full.
            self.cur = n;
            return None;
        };

        // Proper command.
        let num = self.num;
        self.cancel();
        Some((Some(cmd), num))
    }

    pub fn cancel(&mut self) {
        self.num = None;
        self.cur = 0;
    }

    pub fn default_controls() -> Self {
        let mut res = Self::default();

        fn p(s: &str) -> Keys {
            s.arg_into().unwrap()
        }

        res.add_cmd(p("j"), Cmd::MoveDown);
        res.add_cmd(p("down"), Cmd::MoveDown);
        res.add_cmd(p("k"), Cmd::MoveUp);
        res.add_cmd(p("up"), Cmd::MoveUp);
        res.add_cmd(p("h"), Cmd::MoveLeftWrap);
        res.add_cmd(p("left"), Cmd::MoveLeftWrap);
        res.add_cmd(p("l"), Cmd::MoveRightWrap);
        res.add_cmd(p("right"), Cmd::MoveRightWrap);
        res.add_cmd(p("ctrl-y"), Cmd::ScrollUp);
        res.add_cmd(p("ctrl-e"), Cmd::ScrollDown);
        res.add_cmd(p("ctrl-u"), Cmd::ScrollUpHalf);
        res.add_cmd(p("ctrl-d"), Cmd::ScrollDownHalf);
        res.add_cmd(p(":"), Cmd::StartCommand);
        res.add_cmd(p("shift-g"), Cmd::MoveToBottom);
        res.add_cmd(p("g g"), Cmd::MoveToTop);

        res
    }
}
