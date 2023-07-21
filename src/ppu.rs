use self::{
    registers::{LoopyRegister, Registers},
    render::RenderArgs,
};
use crate::{
    bus::Bus,
    cpu::CPU,
    ppu::registers::{PPUCtrl, PPUMask},
};
use mockall::automock;
use std::{borrow::BorrowMut, cell::RefCell};

mod registers;
mod render;

pub type Frame = [[u8; 256]; 240];

#[automock]
pub trait PPU {
    fn clock(&mut self, cpu: &mut dyn CPU);
    fn read(&self, addr: u16, rd_only: bool) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn reset(&mut self);
    fn is_frame_completed(&self) -> bool;
    fn get_frame(&self) -> Frame;
}

pub struct NESPPU<'a> {
    registers: Registers,
    render_args: RenderArgs,

    ppu_bus: Box<dyn Bus + 'a>,

    scanline: i16,
    cycle: u16,

    front_buffer: Frame,
    back_buffer: Frame,

    completed_frame: RefCell<bool>,
}

impl<'a> NESPPU<'a> {
    pub fn new(ppu_bus: Box<dyn Bus + 'a>) -> Self {
        NESPPU {
            registers: Registers::new(),
            render_args: RenderArgs::new(),

            ppu_bus,

            scanline: -1,
            cycle: 0,

            front_buffer: [[0x0; 256]; 240],
            back_buffer: [[0x0; 256]; 240],

            completed_frame: RefCell::new(false),
        }
    }
}

impl PPU for NESPPU<'_> {
    fn read(&self, addr: u16, rd_only: bool) -> u8 {
        assert!((0x2000..=0x3fff).contains(&addr) || addr == 0x4014);

        if addr == 0x4014 {
            return 0x0; //OAMDMA is write-only
        }

        let offset = (addr - 0x2000) & 0x7;
        match offset {
            0x0 => 0x0, //PPUCTRL is write-only
            0x1 => 0x0, //PPUMASK is write-only
            0x2 => {
                let status_to_return = self.registers.ppu_status.borrow().into_bytes()[0];

                let mut ppu_status = self.registers.ppu_status.borrow_mut();
                let mut write_latch = self.registers.write_latch.borrow_mut();

                (*ppu_status).set_vblank(false);
                *write_latch = false;

                status_to_return
            }
            0x3 => 0x0,
            0x4 => 0x0,
            0x5 => 0x0, //PPUSCROLL is write-only
            0x6 => 0x0, //PPUADDR is write-only
            0x7 => {
                if rd_only {
                    return 0x0;
                }

                let mut data = *self.registers.ppu_data_buffer.borrow();
                let mut loopy_v = self.registers.loopy_v.borrow_mut();

                let loopy_v_raw = (*loopy_v).get_raw();
                *self.registers.ppu_data_buffer.borrow_mut() =
                    self.ppu_bus.read(loopy_v_raw & 0x3fff);

                if loopy_v_raw >= 0x3f00 {
                    data = *self.registers.ppu_data_buffer.borrow();
                }

                let offset = if self.registers.ppu_ctrl.increment() {
                    32
                } else {
                    1
                };

                let to_write = loopy_v_raw + offset;

                *loopy_v = LoopyRegister::from_bytes([
                    (to_write & 0xff) as u8,
                    ((to_write & 0xff00) >> 8) as u8,
                ]);

                data
            }
            _ => panic!("PPU Register {offset} is invalid, must be from 0x0 to 0x7"),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        assert!((0x2000..=0x3fff).contains(&addr) || addr == 0x4014);

        if addr == 0x4014 {
            return;
        }

        let offset = (addr - 0x2000) & 0x7;
        match offset {
            0x0 => {
                self.registers.ppu_ctrl = PPUCtrl::from_bytes([data]);
                let nametable_x = self.registers.ppu_ctrl.nametable_x();
                let nametable_y = self.registers.ppu_ctrl.nametable_y();

                self.registers.loopy_t.set_nametable_x(nametable_x);
                self.registers.loopy_t.set_nametable_y(nametable_y);
            }
            0x1 => {
                self.registers.ppu_mask = PPUMask::from_bytes([data]);
            }
            0x2 => {} //PPUSTATUS is read-only
            0x3 => {}
            0x4 => {}
            0x5 => {
                let latch = *self.registers.write_latch.borrow();
                if latch {
                    *self.registers.write_latch.borrow_mut() = false;
                    self.registers.loopy_t.set_coarse_y(data >> 3);
                    self.registers.loopy_t.set_fine_y(data & 0x7);
                } else {
                    *self.registers.write_latch.borrow_mut() = true;
                    self.registers.fine_x = data & 0x7;
                    self.registers.loopy_t.set_coarse_x(data >> 3);
                }
            }
            0x6 => {
                let latch = *self.registers.write_latch.borrow();
                if latch {
                    *self.registers.write_latch.borrow_mut() = false;

                    let mut bytes = self.registers.loopy_t.into_bytes();
                    bytes[0] = data;
                    *self.registers.loopy_t.borrow_mut() = LoopyRegister::from_bytes(bytes);

                    *self.registers.loopy_v.borrow_mut() = self.registers.loopy_t;
                } else {
                    *self.registers.write_latch.borrow_mut() = true;
                    let mut bytes = self.registers.loopy_t.into_bytes();
                    bytes[1] = data & 0x3f;
                    *self.registers.loopy_t.borrow_mut() = LoopyRegister::from_bytes(bytes);
                }
            }
            0x7 => {
                let mut loopy_v = self.registers.loopy_v.borrow_mut();
                self.ppu_bus.write(loopy_v.get_raw() & 0x3fff, data);

                let offset = if self.registers.ppu_ctrl.increment() {
                    32
                } else {
                    1
                };

                let raw = loopy_v.get_raw() + offset;

                *loopy_v =
                    LoopyRegister::from_bytes([(raw & 0xff) as u8, ((raw & 0xff00) >> 8) as u8]);
            }
            _ => panic!("Register {offset} is invalid, must be from 0x0 to 0x7"),
        }
    }

    fn reset(&mut self) {
        self.registers.ppu_ctrl = PPUCtrl::from_bytes([0x0]);
        self.registers.ppu_mask = PPUMask::from_bytes([0x0]);

        let mut ppu_status = self.registers.ppu_status.borrow_mut();
        let mut write_latch = self.registers.write_latch.borrow_mut();

        (*ppu_status).set_spr_0_hit(false);
        (*ppu_status).set_spr_overflow(false);

        *write_latch = false;
        self.registers.loopy_t = LoopyRegister::from_bytes([0x0; 2]);
        self.registers.odd_frame = false;
    }

    fn clock(&mut self, cpu: &mut dyn CPU) {
        //Update registers
        self.update_registers(cpu);

        //Draw pixel
        self.draw_pixel();
        //Shift registers for next pixel
        self.shift_registers_left();

        //Increment cycle/scanline
        self.increment_cycle();
    }

    fn is_frame_completed(&self) -> bool {
        let mut completed = self.completed_frame.borrow_mut();
        if *completed {
            *completed = false;
            return true;
        }

        false
    }

    fn get_frame(&self) -> Frame {
        self.front_buffer
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
    fn draw_pixel(&mut self) {
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
    fn shift_registers_left(&mut self) {
        if self.cycle >= 336 {
            return;
        }

        self.render_args.shift_lsb <<= 1;
        self.render_args.shift_msb <<= 1;

        self.render_args.palette_shift_lsb <<= 1;
        self.render_args.palette_shift_msb <<= 1;
    }

    #[inline]
    fn update_registers(&mut self, cpu: &mut dyn CPU) {
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
    fn increment_cycle(&mut self) {
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

#[cfg(test)]
mod ppu_tests {
    use std::borrow::Borrow;

    use mockall::predicate::eq;

    use crate::{bus::MockBus, ppu::registers::PPUStatus};

    use super::*;

    #[test]
    fn test_ppu_ctrl_write_sets_nt_bits_in_t_reg() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));

        ppu.write(0x2000, 0x7);
        assert_eq!(
            0b0000_1100_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(0x7, ppu.registers.ppu_ctrl.into_bytes()[0]);

        assert_eq!(true, ppu.registers.ppu_ctrl.nametable_x());
        assert_eq!(true, ppu.registers.ppu_ctrl.nametable_y());
        assert_eq!(true, ppu.registers.ppu_ctrl.increment());
        assert_eq!(false, ppu.registers.ppu_ctrl.spr_addr());
        assert_eq!(false, ppu.registers.ppu_ctrl.bg_addr());
        assert_eq!(false, ppu.registers.ppu_ctrl.spr_size());
        assert_eq!(false, ppu.registers.ppu_ctrl.master_slave_select());
        assert_eq!(false, ppu.registers.ppu_ctrl.nmi_enable());

        ppu.write(0x2000, 0x0);
        assert_eq!(
            0b0000_0000_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(0x0, ppu.registers.ppu_ctrl.into_bytes()[0]);

        assert_eq!(false, ppu.registers.ppu_ctrl.nametable_x());
        assert_eq!(false, ppu.registers.ppu_ctrl.nametable_y());
        assert_eq!(false, ppu.registers.ppu_ctrl.increment());
        assert_eq!(false, ppu.registers.ppu_ctrl.spr_addr());
        assert_eq!(false, ppu.registers.ppu_ctrl.bg_addr());
        assert_eq!(false, ppu.registers.ppu_ctrl.spr_size());
        assert_eq!(false, ppu.registers.ppu_ctrl.master_slave_select());
        assert_eq!(false, ppu.registers.ppu_ctrl.nmi_enable());

        ppu.write(0x2000, 0x2);
        assert_eq!(
            0b0000_1000_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(0x2, ppu.registers.ppu_ctrl.into_bytes()[0]);

        assert_eq!(false, ppu.registers.ppu_ctrl.nametable_x());
        assert_eq!(true, ppu.registers.ppu_ctrl.nametable_y());
        assert_eq!(false, ppu.registers.ppu_ctrl.increment());
        assert_eq!(false, ppu.registers.ppu_ctrl.spr_addr());
        assert_eq!(false, ppu.registers.ppu_ctrl.bg_addr());
        assert_eq!(false, ppu.registers.ppu_ctrl.spr_size());
        assert_eq!(false, ppu.registers.ppu_ctrl.master_slave_select());
        assert_eq!(false, ppu.registers.ppu_ctrl.nmi_enable());
    }

    #[test]
    fn test_ppu_status_read_resets_latch() {
        let ppu = NESPPU::new(Box::new(MockBus::new()));

        *ppu.registers.write_latch.borrow_mut() = true;
        *ppu.registers.ppu_status.borrow_mut() = PPUStatus::from_bytes([0xff]);

        assert_eq!(0xff, ppu.read(0x2002, false));
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(0x7f, ppu.registers.ppu_status.borrow().into_bytes()[0]);

        assert_eq!(0x7f, ppu.read(0x2002, false));
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(0x7f, ppu.registers.ppu_status.borrow().into_bytes()[0]);
    }

    #[test]
    fn test_ppu_write_scroll() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2005, 0b1000_1101);

        assert_eq!(true, *ppu.registers.write_latch.borrow());
        assert_eq!(0b101, ppu.registers.fine_x);
        assert_eq!(0b10001, ppu.registers.loopy_t.borrow().get_raw());

        ppu.write(0x2005, 0b10001101);
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(
            0b0101_0010_0011_0001,
            ppu.registers.loopy_t.borrow().get_raw()
        );
    }

    #[test]
    fn test_ppu_write_addr() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2006, 0xff);

        assert_eq!(true, *ppu.registers.write_latch.borrow());
        assert_eq!(
            0b0011_1111_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );

        ppu.write(0x2006, 0xfe);
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(
            0b0011_1111_1111_1110,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(
            0b0011_1111_1111_1110,
            ppu.registers.loopy_v.borrow().get_raw()
        );
    }

    #[test]
    fn test_ppu_reset() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        *ppu.registers.ppu_status.borrow_mut() = PPUStatus::from_bytes([0xff]);
        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0xff, 0x7f]));
        ppu.registers.odd_frame = true;
        ppu.reset();

        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(false, ppu.registers.odd_frame);

        assert_eq!(0x0, ppu.registers.ppu_ctrl.into_bytes()[0]);
        assert_eq!(
            0x80,
            ppu.registers.ppu_status.borrow().into_bytes()[0] & 0xe0
        );
        assert_eq!(0x0, ppu.registers.loopy_t.borrow().get_raw());
        assert_eq!(0x7fff, ppu.registers.loopy_v.borrow().get_raw());
    }

    #[test]
    fn test_ppu_mask() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2001, 0xff);
        assert_eq!(0xff, ppu.registers.ppu_mask.into_bytes()[0]);
    }

    #[test]
    fn test_ppu_data_read() {
        let mut bus = MockBus::new();
        bus.expect_read().with(eq(0x2000)).once().return_const(0xff);
        bus.expect_read().with(eq(0x2001)).once().return_const(0xee);
        bus.expect_read().with(eq(0x3f00)).once().return_const(0xdd);
        bus.expect_read().with(eq(0x2400)).once().return_const(0xcc);

        let mut ppu = NESPPU::new(Box::new(bus));
        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0x00, 0x20]));

        assert_eq!(0x0, ppu.read(0x2007, false));
        assert_eq!(0x2001, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xff, *ppu.registers.ppu_data_buffer.borrow());

        assert_eq!(0xff, ppu.read(0x2007, false));
        assert_eq!(0x2002, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xee, *ppu.registers.ppu_data_buffer.borrow());

        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0x00, 0x3f]));
        assert_eq!(0xdd, ppu.read(0x2007, false));
        assert_eq!(0x3f01, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xdd, *ppu.registers.ppu_data_buffer.borrow());

        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0x00, 0x24]));
        ppu.registers.ppu_ctrl.set_increment(true);
        assert_eq!(0xdd, ppu.read(0x2007, false));
        assert_eq!(0x2420, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xcc, *ppu.registers.ppu_data_buffer.borrow());
    }

    #[test]
    fn test_ppu_data_write() {
        let mut bus = MockBus::new();
        bus.expect_write()
            .with(eq(0x2000), eq(0xff))
            .once()
            .return_const(());
        bus.expect_write()
            .with(eq(0x2001), eq(0xee))
            .once()
            .return_const(());

        let mut ppu = NESPPU::new(Box::new(bus));

        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0x00, 0x20]));
        ppu.write(0x2007, 0xff);
        assert_eq!(0x2001, ppu.registers.loopy_v.borrow().get_raw());

        ppu.registers.ppu_ctrl.set_increment(true);
        ppu.write(0x2007, 0xee);
        assert_eq!(0x2021, ppu.registers.loopy_v.borrow().get_raw());
    }
}
