use std::cell::RefCell;

pub(super) struct Registers {
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: RefCell<u8>,

    pub oam_addr: u8,
    pub oam_data: u8,

    pub ppu_scroll_x: u8,
    pub ppu_scroll_y: u8,

    pub ppu_addr: u16,
    pub ppu_data: u8,

    pub oam_dma: u8,

    pub scroll_latch: RefCell<bool>,
    pub addr_latch: RefCell<bool>,
}

impl Registers {
    pub(super) fn new() -> Self {
        Registers {
            ppu_ctrl: 0x0,
            ppu_mask: 0x0,
            ppu_status: RefCell::new(0x0),

            oam_addr: 0x0,
            oam_data: 0x0,

            ppu_scroll_x: 0x0,
            ppu_scroll_y: 0x0,

            ppu_addr: 0x0,
            ppu_data: 0x0,

            oam_dma: 0x0,

            scroll_latch: RefCell::new(false),
            addr_latch: RefCell::new(false),
        }
    }
}
