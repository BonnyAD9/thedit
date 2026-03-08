use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Default)]
    pub struct ArgFlags: u64 {
        const NONE = 0x0;
        const HELPED = 0x1;
        const DUMP = 0x2;
        const UTF = 0x4;
        const STDIN = 0x8;
    }
}
