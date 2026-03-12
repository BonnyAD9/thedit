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

    pub fn add_cmd_cnt(
        &mut self,
        cmd: impl Into<String>,
        act: Cmd,
        cnt: usize,
    ) {
        self.cmds.insert(cmd.into(), (act, Some(cnt)));
    }

    pub fn default_controls() -> Self {
        let mut res = Self::default();

        res.add_cmd(":x", Cmd::Exit);
        res.add_cmd(":q", Cmd::Exit);
        res.add_cmd(":exit", Cmd::Exit);
        res.add_cmd(":quit", Cmd::Exit);
        res.add_cmd_cnt(":byte", Cmd::ShowUnsigned, 1);
        res.add_cmd_cnt(":sbyte", Cmd::ShowSigned, 1);
        res.add_cmd_cnt(":short", Cmd::ShowSigned, 2);
        res.add_cmd_cnt(":ushort", Cmd::ShowUnsigned, 2);
        res.add_cmd_cnt(":int", Cmd::ShowSigned, 4);
        res.add_cmd_cnt(":uint", Cmd::ShowUnsigned, 4);
        res.add_cmd_cnt(":long", Cmd::ShowSigned, 8);
        res.add_cmd_cnt(":ulong", Cmd::ShowUnsigned, 8);
        res.add_cmd_cnt(":be", Cmd::SetBigEndian, 8);
        res.add_cmd_cnt(":le", Cmd::SetLittleEndian, 8);

        res
    }
}
