use crate::{
    err::{Error, Result},
    view::ctrl::Cmd,
};

#[derive(Debug, Clone, Default)]
pub struct CommandCtrl;

impl CommandCtrl {
    pub fn execute(&self, cmd: &str) -> Result<(Cmd, Option<usize>)> {
        if matches!(cmd, ":q" | ":x" | ":exit" | ":quit") {
            Ok((Cmd::Exit, None))
        } else {
            Error::unknown_command(cmd.to_string()).err()
        }
    }
}
