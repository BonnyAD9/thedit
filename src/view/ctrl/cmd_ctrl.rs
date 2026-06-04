use pareg::ArgInto;
use termal::raw::events::KeyCode;

use crate::view::{
    Mode,
    ctrl::{Cmd, CmdKey, Keys, key_node::KeyNode},
};

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
        if self.cur != 0 && key.code == KeyCode::Esc {
            self.cancel();
            return Some((Some(Cmd::None), None));
        }

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

    pub fn get_all(&self) -> Vec<(Keys, Cmd)> {
        let mut res = vec![];
        // (depth, idx)
        let mut stack = vec![(0, None, 0)];
        let mut keys = vec![];

        while let Some((depth, key, node)) = stack.pop() {
            keys.resize_with(depth, || unreachable!());
            if let Some(k) = key {
                keys.push(k);
            }

            let node = &self.nodes[node];
            if let Some(cmd) = node.cmd {
                res.push((Keys(keys.clone()), cmd));
            }

            for (k, n) in &node.next {
                stack.push((keys.len(), Some(*k), *n));
            }
        }

        res
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
        res.add_cmd(p("G"), Cmd::MoveToBottom);
        res.add_cmd(p("ctrl-end"), Cmd::MoveToBottom);
        res.add_cmd(p("g g"), Cmd::MoveToTop);
        res.add_cmd(p("ctrl-home"), Cmd::MoveToTop);
        res.add_cmd(p("ctrl-b"), Cmd::MovePageUp);
        res.add_cmd(p("pg_up"), Cmd::MovePageUp);
        res.add_cmd(p("ctrl-f"), Cmd::MovePageDown);
        res.add_cmd(p("pg_down"), Cmd::MovePageDown);
        res.add_cmd(p("_"), Cmd::MoveToStart);
        res.add_cmd(p("home"), Cmd::MoveToStart);
        res.add_cmd(p("$"), Cmd::MoveToEnd);
        res.add_cmd(p("end"), Cmd::MoveToEnd);
        res.add_cmd(p("ctrl-pg_down"), Cmd::ScrollPageDown);
        res.add_cmd(p("ctrl-pg_up"), Cmd::ScrollPageUp);
        res.add_cmd(p("ctrl-down"), Cmd::ScrollDown);
        res.add_cmd(p("ctrl-up"), Cmd::ScrollUp);
        res.add_cmd(p("S U"), Cmd::ShowUnsigned);
        res.add_cmd(p("S I"), Cmd::ShowSigned);
        res.add_cmd(p("S u"), Cmd::VisualUnsigned);
        res.add_cmd(p("S i"), Cmd::VisualSigned);
        res.add_cmd(p("S E"), Cmd::SwapEndianness);
        res.add_cmd(p("esc"), Cmd::Cancel);
        res.add_cmd(p("v"), Cmd::SetMode(Mode::Visual));

        // Temporary workaround
        res.add_cmd(p("m"), Cmd::None);
        res.add_cmd(p("M"), Cmd::None);

        res
    }
}
