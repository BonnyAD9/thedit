use std::collections::HashMap;

use crate::{
    err::{Error, Result},
    view::ctrl::Cmd,
};

#[derive(Debug, Clone, Default)]
pub struct CommandCtrl {
    cmds: HashMap<String, (Cmd, Option<usize>)>,
}

impl CommandCtrl {
    pub fn execute(&self, cmd: &str) -> Result<(Cmd, Option<usize>)> {
        if let Some(res) = self.cmds.get(cmd) {
            Ok(*res)
        } else {
            Error::unknown_command(cmd.to_string()).err()
        }
    }

    pub fn add_cmd(&mut self, cmd: impl Into<String>, act: Cmd) {
        self.cmds.insert(cmd.into(), (act, None));
    }

    pub fn default_controls() -> Self {
        let mut res = Self::default();

        res.add_cmd(":x", Cmd::Exit);
        res.add_cmd(":q", Cmd::Exit);
        res.add_cmd(":exit", Cmd::Exit);
        res.add_cmd(":quit", Cmd::Exit);

        res
    }
}
