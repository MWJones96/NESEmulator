#![allow(dead_code)]
use std::cell::RefCell;

use modular_bitfield::{
    bitfield,
    specifiers::{B2, B3, B5},
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
    #[skip]
    unused: B5,
    pub(super) spr_overflow: bool,
    pub(super) spr_0_hit: bool,
    pub(super) vblank: bool,
}

#[bitfield]
#[derive(Copy, Clone)]
pub(super) struct LoopyRegister {
    pub(super) coarse_x: B5,
    pub(super) coarse_y: B5,
    pub(super) nametable: B2,
    pub(super) fine_y: B3,
    #[skip]
    unused: bool,
}

impl LoopyRegister {
    pub(super) fn get_raw(&self) -> u16 {
        (self.fine_y() as u16) << 12
            | (self.nametable() as u16) << 10
            | (self.coarse_y() as u16) << 5
            | (self.coarse_x() as u16)
    }

    pub(super) fn increment(&mut self, inc: u16) {
        let loopy_raw = self.get_raw();
        let new_loopy_raw = loopy_raw.wrapping_add(inc);

        self.set_coarse_x((new_loopy_raw & 0b11111) as u8);
        self.set_coarse_y(((new_loopy_raw & 0x3f0) >> 5) as u8);
        self.set_nametable(((new_loopy_raw & 0xC00) >> 10) as u8);
        self.set_fine_y(((new_loopy_raw & 0x7000) >> 12) as u8);
    }

    pub(super) fn set_low_byte(&mut self, byte: u8) {
        let new_loopy_raw = (self.get_raw() & 0xff00) | (byte as u16);

        self.set_coarse_x((new_loopy_raw & 0b11111) as u8);
        self.set_coarse_y(((new_loopy_raw & 0x3f0) >> 5) as u8);
        self.set_nametable(((new_loopy_raw & 0xC00) >> 10) as u8);
        self.set_fine_y(((new_loopy_raw & 0x7000) >> 12) as u8);
    }

    pub(super) fn set_high_byte(&mut self, byte: u8) {
        let new_loopy_raw = (self.get_raw() & 0xff) | (byte as u16) << 8;

        self.set_coarse_x((new_loopy_raw & 0b11111) as u8);
        self.set_coarse_y(((new_loopy_raw & 0x3f0) >> 5) as u8);
        self.set_nametable(((new_loopy_raw & 0xC00) >> 10) as u8);
        self.set_fine_y(((new_loopy_raw & 0x7000) >> 12) as u8);
    }

    pub(super) fn set_bit_15_to_0(&mut self) {
        let new_loopy_raw = self.get_raw() & 0x3fff;

        self.set_coarse_x((new_loopy_raw & 0b11111) as u8);
        self.set_coarse_y(((new_loopy_raw & 0x3f0) >> 5) as u8);
        self.set_nametable(((new_loopy_raw & 0xC00) >> 10) as u8);
        self.set_fine_y(((new_loopy_raw & 0x7000) >> 12) as u8);
    }
}

pub(super) struct Registers {
    pub write_latch: RefCell<bool>,
    pub loopy_t: LoopyRegister,
    pub loopy_v: RefCell<LoopyRegister>,
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
            loopy_t: LoopyRegister::from_bytes([0x0; 2]),
            loopy_v: RefCell::new(LoopyRegister::from_bytes([0x0; 2])),
            fine_x: 0x0,

            ppu_ctrl: PPUCtrl::from_bytes([0x0]),
            ppu_mask: PPUMask::from_bytes([0x0]),
            ppu_status: RefCell::new(PPUStatus::from_bytes([0x0])),

            odd_frame: false,
            ppu_data_buffer: RefCell::new(0x0),
        }
    }
}
