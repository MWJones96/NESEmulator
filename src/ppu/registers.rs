use std::cell::RefCell;

use modular_bitfield::{
    bitfield,
    specifiers::{B2, B5},
};

#[bitfield]
#[derive(Copy, Clone)]
pub(super) struct PPUCtrl {
    pub(super) nametable: B2,
    pub(super) increment: bool,
    pub(super) spr_addr: bool,
    pub(super) bg_addr: bool,
    pub(super) spr_size: bool,
    pub(super) master_slave_select: bool,
    pub(super) nmi_enable: bool,
}

#[bitfield]
#[derive(Copy, Clone)]
pub(super) struct PPUMask {
    pub(super) greyscale: bool,
    pub(super) bg_in_left_8: bool,
    pub(super) spr_in_left_8: bool,
    pub(super) show_bg: bool,
    pub(super) show_spr: bool,
    pub(super) emph_red: bool,
    pub(super) emph_green: bool,
    pub(super) emph_blue: bool,
}

#[bitfield]
#[derive(Copy, Clone)]
pub(super) struct PPUStatus {
    pub(super) _unused: B5,
    pub(super) spr_overflow: bool,
    pub(super) spr_0_hit: bool,
    pub(super) vblank: bool,
}

pub(super) struct Registers {
    pub write_latch: RefCell<bool>,
    pub loopy_t: u16,
    pub loopy_v: RefCell<u16>,
    pub fine_x: u8,

    pub ppu_ctrl: PPUCtrl,
    pub ppu_mask: PPUMask,
    pub ppu_status: RefCell<PPUStatus>,

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

            ppu_ctrl: PPUCtrl::from_bytes([0x0]),
            ppu_mask: PPUMask::from_bytes([0x0]),
            ppu_status: RefCell::new(PPUStatus::from_bytes([0x0])),

            odd_frame: false,
            ppu_data_buffer: RefCell::new(0x0),
        }
    }
}
