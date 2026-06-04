use std::time::Duration;

use termal::{
    raw::{raw_guard, request, term_size},
    reset_terminal,
};

use crate::{err::Result, file_view::FileView, view::view_state::ViewState};

mod ctrl;
mod help;
mod mode;
mod pager;
mod pos;
mod view_state;

pub use self::{help::*, mode::*, pos::*};

pub fn view(file: FileView, utf: bool) -> Result<()> {
    let size = term_size()?;
    let chr_h = request::char_size(Duration::from_millis(100))?;
    let mut state =
        ViewState::new(file, size.char_width, size.char_height, chr_h.y, utf);

    termal::register_reset_on_panic();
    let res = raw_guard(true, || state.run());
    reset_terminal();
    res
}
