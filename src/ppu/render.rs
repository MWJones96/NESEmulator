pub(super) struct RenderArgs {
    pub(super) nt_data: u8,
    pub(super) at_data: u8,
    pub(super) bg_low: u8,
    pub(super) bg_high: u8,

    pub(super) shift_lsb: u16,
    pub(super) shift_msb: u16,

    pub(super) palette_shift_lsb: u16,
    pub(super) palette_shift_msb: u16,
}

impl RenderArgs {
    pub(super) fn new() -> Self {
        RenderArgs {
            nt_data: 0,
            at_data: 0,
            bg_low: 0,
            bg_high: 0,
            shift_lsb: 0,
            shift_msb: 0,
            palette_shift_lsb: 0,
            palette_shift_msb: 0,
        }
    }
}
