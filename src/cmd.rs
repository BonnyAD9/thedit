use pareg::FromArg;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromArg)]
#[arg(exact)]
pub enum Cmd {
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
}
