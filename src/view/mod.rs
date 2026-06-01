use termal::{
    raw::{raw_guard, term_size},
    reset_terminal,
};

use crate::{err::Result, file_view::FileView, view::view_state::ViewState};

mod ctrl;
mod mode;
mod pos;
mod view_state;

pub use self::{mode::*, pos::*};

pub fn view(file: FileView) -> Result<()> {
    let size = term_size()?;
    let mut state = ViewState::new(file, size.char_width, size.char_height);

    termal::register_reset_on_panic();
    let res = raw_guard(true, || state.run());
    reset_terminal();
    res
}
