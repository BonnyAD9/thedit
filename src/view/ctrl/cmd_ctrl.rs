use std::collections::HashMap;

use pareg::ArgInto;
use termal::raw::events::KeyCode;

use crate::view::ctrl::{
    Cmd, CmdKey, Keys, Mode, key_node::KeyNode, modes::Modes,
};

#[derive(Debug, Clone, Default)]
pub struct CmdCtrl {
    nodes: Vec<KeyNode>,
    roots: HashMap<Mode, usize>,
    cur: Option<usize>,
    num: Option<usize>,
}

impl CmdCtrl {
    pub fn add_cmd(
        &mut self,
        mode: Mode,
        keys: impl IntoIterator<Item = CmdKey>,
        cmd: Cmd,
    ) {
        let mut cur = *self.roots.entry(mode).or_insert_with(|| {
            let root = self.nodes.len();
            self.nodes.push(KeyNode::default());
            root
        });

        for k in keys {
            let new = self.nodes.len();
            cur = *self.nodes[cur].next.entry(k).or_insert(new);
            if cur == new {
                self.nodes.push(KeyNode::default());
            }
        }
        self.nodes[cur].cmd = Some(cmd);
    }

    pub fn add_mm_cmd(
        &mut self,
        modes: impl IntoIterator<Item = Mode>,
        keys: impl IntoIterator<Item = CmdKey> + Clone,
        cmd: Cmd,
    ) {
        for m in modes.into_iter() {
            self.add_cmd(m, keys.clone(), cmd.clone());
        }
    }

    pub fn type_key(
        &mut self,
        mode: Mode,
        key: CmdKey,
    ) -> Option<(Option<Cmd>, Option<usize>)> {
        if self.cur.is_some() && key.code == KeyCode::Esc {
            self.cancel();
            return Some((Some(Cmd::None), None));
        }

        if self.cur.is_none()
            && let KeyCode::Char(c) = key.code
            && let Some(d) = c.to_digit(10)
        {
            // Numbers before command.
            self.num = Some(self.num.unwrap_or_default() * 10 + d as usize);
            return None;
        }

        self.cur = self.cur.or_else(|| self.roots.get(&mode).copied());

        let Some(cur) = self.cur else {
            let num = self.num;
            self.cancel();
            return Some((None, num));
        };

        let Some(n) = self.nodes[cur].get(key) else {
            // Unknown command.
            let num = self.num;
            self.cancel();
            return Some((None, num));
        };

        let Some(cmd) = self.nodes[n].cmd.clone() else {
            // Command not full.
            self.cur = Some(n);
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
            if let Some(cmd) = node.cmd.clone() {
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
        self.cur = None;
    }

    pub fn default_controls() -> Self {
        let mut res = Self::default();

        fn m(s: &str) -> Modes {
            s.arg_into().unwrap()
        }

        fn k(s: &str) -> Keys {
            s.arg_into().unwrap()
        }

        let mut add_cmd =
            |modes, keys, cmd| res.add_mm_cmd(m(modes), k(keys), cmd);

        add_cmd("nv", "j", Cmd::MoveDown);
        add_cmd("nv", "<down>", Cmd::MoveDown);
        add_cmd("nv", "k", Cmd::MoveUp);
        add_cmd("nv", "<up>", Cmd::MoveUp);
        add_cmd("nv", "h", Cmd::MoveLeft);
        add_cmd("nv", "<left>", Cmd::MoveLeft);
        add_cmd("nv", "l", Cmd::MoveRight);
        add_cmd("nv", "<right>", Cmd::MoveRight);
        add_cmd("nv", "<C-y>", Cmd::ScrollUp);
        add_cmd("nv", "<C-e>", Cmd::ScrollUp);
        add_cmd("nv", "<C-u>", Cmd::ScrollUpHalf);
        add_cmd("nv", "<C-d>", Cmd::ScrollDownHalf);
        add_cmd("nv", ":", Cmd::StartCommand);
        add_cmd("nv", "G", Cmd::MoveToBottom);
        add_cmd("nv", "<C-end>", Cmd::MoveToBottom);
        add_cmd("nv", "gg", Cmd::MoveToTop);
        add_cmd("nv", "<C-home>", Cmd::MoveToTop);
        add_cmd("nv", "<C-b>", Cmd::MovePageUp);
        add_cmd("nv", "<pg_up>", Cmd::MovePageUp);
        add_cmd("nv", "<C-f>", Cmd::MovePageDown);
        add_cmd("nv", "<pg_down>", Cmd::MovePageDown);
        add_cmd("nv", "_", Cmd::MoveToStart);
        add_cmd("nv", "<home>", Cmd::MoveToStart);
        add_cmd("nv", "$", Cmd::MoveToEnd);
        add_cmd("nv", "<end>", Cmd::MoveToEnd);
        add_cmd("nv", "<C-pg_down>", Cmd::ScrollPageDown);
        add_cmd("nv", "<C-pg_up>", Cmd::ScrollPageUp);
        add_cmd("nv", "<C-down>", Cmd::ScrollDown);
        add_cmd("nv", "<C-up>", Cmd::ScrollUp);
        add_cmd("nv", "SU", Cmd::ShowUnsigned);
        add_cmd("nv", "SI", Cmd::ShowSigned);
        add_cmd("nv", "Su", Cmd::VisualUnsigned);
        add_cmd("nv", "Si", Cmd::VisualSigned);
        add_cmd("nv", "SE", Cmd::SwapEndianness);
        add_cmd("nv", "<esc>", Cmd::Cancel);
        add_cmd("n", "v", Cmd::SetMode(Mode::Visual));
        add_cmd("v", "v", Cmd::SetMode(Mode::Normal));

        // Temporary workaround
        add_cmd("nv", "m", Cmd::None);
        add_cmd("nv", "M", Cmd::None);

        res
    }
}
