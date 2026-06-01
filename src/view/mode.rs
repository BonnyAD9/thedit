use pareg::FromArg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromArg)]
#[arg(exact)]
pub enum Mode {
    #[arg("normal")]
    Normal,
    #[arg("visual")]
    Visual,
}
