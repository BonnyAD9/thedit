#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cmd {
    Exit,
    ScrollDown,
    ScrollUp,
    MoveRight,
    MoveDown,
    MoveLeft,
    MoveUp,
    MoveRightWrap,
    MoveLeftWrap,
    ScrollToView,
}