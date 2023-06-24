use std::cell::RefCell;

pub(super) struct Registers {
    pub write_latch: RefCell<bool>,
    pub loopy_t: u16,
    pub loopy_v: RefCell<u16>,
    pub fine_x: u8,

    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: RefCell<u8>,

    pub odd_frame: bool,
    pub ppu_data_buffer: RefCell<u8>,
}

impl Registers {
    pub(super) fn new() -> Self {
        Registers {
            write_latch: RefCell::new(false),
            loopy_t: 0x0,
            loopy_v: RefCell::new(0x0),
            fine_x: 0x0,

            ppu_ctrl: 0x0,
            ppu_mask: 0x0,
            ppu_status: RefCell::new(0x0),

            odd_frame: false,
            ppu_data_buffer: RefCell::new(0x0),
        }
    }
}
