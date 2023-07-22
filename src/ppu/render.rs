use std::cell::RefCell;

use crate::cpu::CPU;

use super::NESPPU;

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

impl NESPPU<'_> {
    #[inline]
    fn fetch_bg_msb(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let fine_y = self.registers.loopy_v.borrow().fine_y() as u16;
        let offset: u16 = if self.registers.ppu_ctrl.bg_addr() {
            0x1000
        } else {
            0x0
        };
        self.render_args.bg_high = self
            .ppu_bus
            .read(offset + 16 * (self.render_args.nt_data as u16) + fine_y + 8);
    }

    #[inline]
    fn fetch_bg_lsb(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let fine_y = self.registers.loopy_v.borrow().fine_y() as u16;
        let offset: u16 = if self.registers.ppu_ctrl.bg_addr() {
            0x1000
        } else {
            0x0
        };

        self.render_args.bg_low = self
            .ppu_bus
            .read(offset + 16 * (self.render_args.nt_data as u16) + fine_y);
    }

    #[inline]
    fn fetch_at_data(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let loopy_v = *self.registers.loopy_v.borrow();

        self.render_args.at_data = self.ppu_bus.read(
            0x23C0
                | ((loopy_v.nametable_y() as u16) << 11)
                | ((loopy_v.nametable_x() as u16) << 10)
                | (((loopy_v.coarse_y() as u16) >> 2) << 3)
                | ((loopy_v.coarse_x() as u16) >> 2),
        );

        if (loopy_v.coarse_y() & 0x2) != 0 {
            self.render_args.at_data >>= 4;
        }

        if (loopy_v.coarse_x() & 0x2) != 0 {
            self.render_args.at_data >>= 2;
        }

        self.render_args.at_data &= 0x3;
    }

    #[inline]
    fn fetch_nt_data(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let nt_addr = 0x2000 | ((*self.registers.loopy_v.borrow()).get_raw() & 0xfff);
        self.render_args.nt_data = self.ppu_bus.read(nt_addr);
    }

    #[inline]
    fn push_to_shift_registers(&mut self) {
        self.render_args.shift_lsb =
            (self.render_args.shift_lsb & 0xff00) | (self.render_args.bg_low as u16);
        self.render_args.shift_msb =
            (self.render_args.shift_msb & 0xff00) | (self.render_args.bg_high as u16);

        self.render_args.palette_shift_lsb = (self.render_args.palette_shift_lsb & 0xff00)
            | (if (self.render_args.at_data & 0b1) != 0 {
                0xff
            } else {
                0x00
            });
        self.render_args.palette_shift_msb = (self.render_args.palette_shift_msb & 0xff00)
            | (if (self.render_args.at_data & 0b10) != 0 {
                0xff
            } else {
                0x00
            });
    }

    #[inline]
    fn increment_x(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let mut loopy_v = self.registers.loopy_v.borrow_mut();
        if loopy_v.coarse_x() == 31 {
            loopy_v.set_coarse_x(0);
            let other_nt = !loopy_v.nametable_x();
            loopy_v.set_nametable_x(!other_nt);
        } else {
            let new_coarse_x = loopy_v.coarse_x() + 1;
            loopy_v.set_coarse_x(new_coarse_x);
        }
    }

    #[inline]
    fn increment_y(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let mut loopy_v = self.registers.loopy_v.borrow_mut();
        let new_fine_y = (*loopy_v).fine_y() + 1;
        if new_fine_y >= 8 {
            (*loopy_v).set_fine_y(0);
            let new_coarse_y = (*loopy_v).coarse_y() + 1;
            if new_coarse_y >= 30 {
                let other_nt = !loopy_v.nametable_y();
                (*loopy_v).set_coarse_y(0);
                loopy_v.set_nametable_y(other_nt);
            } else {
                (*loopy_v).set_coarse_y(new_coarse_y);
            }
        } else {
            (*loopy_v).set_fine_y(new_fine_y);
        }
    }

    #[inline]
    fn reset_y(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let mut loopy_v = self.registers.loopy_v.borrow_mut();
        let loopy_t = self.registers.loopy_t;

        (*loopy_v).set_coarse_y(loopy_t.coarse_y());
        (*loopy_v).set_fine_y(loopy_t.fine_y());
        (*loopy_v).set_nametable_y(loopy_t.nametable_y());
    }

    #[inline]
    fn reset_x(&mut self) {
        if !(self.registers.ppu_mask.show_bg() || self.registers.ppu_mask.show_spr()) {
            return;
        }

        let mut loopy_v = self.registers.loopy_v.borrow_mut();
        let loopy_t = self.registers.loopy_t;

        (*loopy_v).set_coarse_x(loopy_t.coarse_x());
        (*loopy_v).set_nametable_x(loopy_t.nametable_x());
    }

    #[inline]
    pub(super) fn draw_pixel(&mut self) {
        if !(self.scanline >= 0 && self.scanline < 240 && self.cycle < 256) {
            return;
        }

        if !self.registers.ppu_mask.show_bg() {
            self.back_buffer[self.scanline as usize][self.cycle as usize] = 0x0;
            return;
        }

        let fine_x_bitmux: u16 = 0x8000 >> self.registers.fine_x;

        let pixel_lsb = ((self.render_args.shift_lsb & fine_x_bitmux) != 0) as u16;
        let pixel_msb = ((self.render_args.shift_msb & fine_x_bitmux) != 0) as u16;

        let palette_lsb = ((self.render_args.palette_shift_lsb & fine_x_bitmux) != 0) as u16;
        let palette_msb = ((self.render_args.palette_shift_msb & fine_x_bitmux) != 0) as u16;

        let pixel = (pixel_msb << 1) | pixel_lsb;
        let palette = (palette_msb << 1) | palette_lsb;

        self.back_buffer[self.scanline as usize][self.cycle as usize] =
            self.ppu_bus.read(0x3f00 + palette * 4 + pixel);
    }

    #[inline]
    pub(super) fn shift_registers_left(&mut self) {
        if self.cycle >= 336 {
            return;
        }

        self.render_args.shift_lsb <<= 1;
        self.render_args.shift_msb <<= 1;

        self.render_args.palette_shift_lsb <<= 1;
        self.render_args.palette_shift_msb <<= 1;
    }

    #[inline]
    pub(super) fn update_registers(&mut self, cpu: &mut dyn CPU) {
        if self.scanline == -1 {
            match self.cycle {
                1 => {
                    //Clean VBlank
                    let mut ppu_status = self.registers.ppu_status.borrow_mut();
                    (*ppu_status).set_vblank(false);
                }
                (280..=304) => self.reset_y(),
                _ => {}
            }
        }

        if let -1..=239 = self.scanline {
            //Render
            match self.cycle {
                328 | 336 | (8..=248) if (self.cycle - 8) % 8 == 0 => {
                    self.fetch_nt_data();
                    self.fetch_at_data();
                    self.fetch_bg_lsb();
                    self.fetch_bg_msb();

                    self.increment_x();
                    self.push_to_shift_registers();
                }
                256 => {
                    self.increment_x();
                    self.increment_y();
                }
                257 => self.reset_x(),
                _ => {}
            }
        }

        if self.scanline == 241 && self.cycle == 1 {
            //VBlank, send an NMI to the CPU
            let mut ppu_status = self.registers.ppu_status.borrow_mut();
            (*ppu_status).set_vblank(true);

            if self.registers.ppu_ctrl.nmi_enable() {
                cpu.cpu_nmi();
            }
        }
    }

    #[inline]
    pub(super) fn increment_cycle(&mut self) {
        let skip_cycle: bool = self.scanline == -1 && self.cycle == 339 && self.registers.odd_frame;
        self.cycle += 1 + (skip_cycle as u16);
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
                self.scanline = -1;
                self.completed_frame = RefCell::new(true);
                self.registers.odd_frame = !self.registers.odd_frame;
            }
        }
    }
}
