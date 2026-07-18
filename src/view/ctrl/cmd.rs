use std::fmt::Display;

use pareg::FromArg;

use crate::view::ctrl::Mode;

#[derive(Debug, Clone, PartialEq, FromArg)]
#[arg(exact, split = '=')]
pub enum Cmd {
    #[arg("none")]
    None,
    #[arg("exit")]
    Exit,
    #[arg("scroll-down")]
    ScrollDown,
    #[arg("scroll-up")]
    ScrollUp,
    #[arg("scroll-down-half")]
    ScrollDownHalf,
    #[arg("scroll-up-half")]
    ScrollUpHalf,
    #[arg("move-right")]
    MoveRight,
    #[arg("move-down")]
    MoveDown,
    #[arg("move-left")]
    MoveLeft,
    #[arg("move-up")]
    MoveUp,
    #[arg("move-right-wrap")]
    MoveRightWrap,
    #[arg("move-left-wrap")]
    MoveLeftWrap,
    #[arg("scroll-to-view")]
    ScrollToView,
    #[arg("start-command")]
    StartCommand,
    #[arg("move-to-top")]
    MoveToTop,
    #[arg("move-to-bottom")]
    MoveToBottom,
    #[arg("view-signed")]
    ShowSigned,
    #[arg("view-unsigned")]
    ShowUnsigned,
    #[arg("swap-endianness")]
    SwapEndianness,
    #[arg("set-big-endian")]
    SetBigEndian,
    #[arg("set-little-endian")]
    SetLittleEndian,
    #[arg("cancel")]
    Cancel,
    #[arg("mode")]
    SetMode(Mode),
    #[arg("visual-signed")]
    VisualSigned,
    #[arg("visual-unsigned")]
    VisualUnsigned,
    #[arg("move-pg-up")]
    MovePageUp,
    #[arg("move-pg-down")]
    MovePageDown,
    #[arg("scroll-pg-up")]
    ScrollPageUp,
    #[arg("scroll-pg-down")]
    ScrollPageDown,
    #[arg("move-to-start")]
    MoveToStart,
    #[arg("move-to-end")]
    MoveToEnd,
    #[arg("show-help")]
    ShowHelp,
    #[arg("enable-utf")]
    EnableUtf(bool),
    Lua(mlua::Function),
}

impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Cmd::None => "none",
            Cmd::Exit => "exit",
            Cmd::ScrollDown => "scroll-down",
            Cmd::ScrollUp => "scroll-up",
            Cmd::ScrollDownHalf => "scroll-down-half",
            Cmd::ScrollUpHalf => "scroll-up-half",
            Cmd::MoveRight => "move-right",
            Cmd::MoveDown => "move-down",
            Cmd::MoveLeft => "move-left",
            Cmd::MoveUp => "move-up",
            Cmd::MoveRightWrap => "move-right-wrap",
            Cmd::MoveLeftWrap => "move-left-wrap",
            Cmd::ScrollToView => "scroll-to-view",
            Cmd::StartCommand => "start-command",
            Cmd::MoveToTop => "move-to-top",
            Cmd::MoveToBottom => "move-to-bottom",
            Cmd::ShowSigned => "show-signed",
            Cmd::ShowUnsigned => "show-unsigned",
            Cmd::SwapEndianness => "swap-endianness",
            Cmd::SetBigEndian => "set-big-endian",
            Cmd::SetLittleEndian => "set-little-endian",
            Cmd::Cancel => "cancel",
            Cmd::SetMode(mode) => return write!(f, "set-mode={mode}"),
            Cmd::VisualSigned => "visual-signed",
            Cmd::VisualUnsigned => "visual-unsigned",
            Cmd::MovePageUp => "move-page-up",
            Cmd::MovePageDown => "move-page-down",
            Cmd::ScrollPageUp => "scroll-page-up",
            Cmd::ScrollPageDown => "scroll-page-down",
            Cmd::MoveToStart => "move-to-start",
            Cmd::MoveToEnd => "move-to-end",
            Cmd::ShowHelp => "show-help",
            Cmd::EnableUtf(e) => return write!(f, "enable-utf={e}"),
            Cmd::Lua(_) => "lua-function",
        };
        f.write_str(s)
    }
}
