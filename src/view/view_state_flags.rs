use bitflags::bitflags;

bitflags! {
    pub struct ViewStateFlags: u64 {
        const REDRAW_BUFFER = 0x1;
        const REDRAW_STATUS = 0x2;
        const REDRAW_ALL = 0x3;
        const EXIT = 0x4;
        const BIG_ENDIAN = 0x8;
        const SIGNED_DRAG = 0x10;
        const UTF = 0x20;
    }
}

impl ViewStateFlags {
    pub fn when(self, v: bool) -> Self {
        if v { self } else { Self::empty() }
    }
}
