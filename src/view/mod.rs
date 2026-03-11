use termal::{
    raw::{raw_guard, term_size},
    reset_terminal,
};

use crate::{err::Result, file_view::FileView, view::view_state::ViewState};

mod ctrl;
mod view_state;

pub fn view(file: FileView) -> Result<()> {
    let height = term_size()?.char_height;
    let mut state = ViewState::new(file, height);

    termal::register_reset_on_panic();
    let res = raw_guard(true, || state.run());
    reset_terminal();
    res
}
