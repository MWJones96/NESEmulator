use std::cell::RefCell;

pub(super) struct Registers {
    pub _ppu_ctrl: u8,
    pub _ppu_mask: u8,
    pub _ppu_status: RefCell<u8>,

    pub _ppu_scroll_x: u8,
    pub _ppu_scroll_y: u8,

    pub _ppu_addr: RefCell<u16>,
    pub _ppu_data: u8,

    pub _scroll_latch: RefCell<bool>,
    pub _addr_latch: RefCell<bool>,
}

impl Registers {
    pub(super) fn new() -> Self {
        Registers {
            _ppu_ctrl: 0x0,
            _ppu_mask: 0x0,
            _ppu_status: RefCell::new(0x0),

            _ppu_scroll_x: 0x0,
            _ppu_scroll_y: 0x0,

            _ppu_addr: RefCell::new(0x0),
            _ppu_data: 0x0,

            _scroll_latch: RefCell::new(false),
            _addr_latch: RefCell::new(false),
        }
    }
}
