use self::registers::{LoopyRegister, Registers};
use crate::{
    bus::Bus,
    cpu::CPU,
    ppu::registers::{PPUCtrl, PPUMask},
};
use mockall::automock;
use std::{borrow::BorrowMut, cell::RefCell};

mod registers;

pub type Frame = [[u8; 256]; 240];

#[automock]
pub trait PPU {
    fn clock(&mut self, cpu: &mut dyn CPU);
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn reset(&mut self);
    fn is_frame_completed(&self) -> bool;
    fn get_frame(&self) -> Frame;
}

pub struct NESPPU<'a> {
    registers: Registers,
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
    fn clock(&mut self, cpu: &mut dyn CPU) {
        //Update registers
        match self.scanline {
            -1 => {
                //Clear VBlank
                if self.cycle == 1 {
                    let mut ppu_status = self.registers.ppu_status.borrow_mut();
                    (*ppu_status).set_vblank(false);
                }
            } //Pre-render
            0..=239 => {
                //Render
                match self.cycle {
                    (2..=250) if (self.cycle - 2) % 8 == 0 => {}
                    (4..=252) if (self.cycle - 4) % 8 == 0 => {}
                    (6..=254) if (self.cycle - 6) % 8 == 0 => {}
                    (8..=256) if (self.cycle - 8) % 8 == 0 => {}
                    (9..=257) if (self.cycle - 9) % 8 == 0 => {}
                    257 => {}
                    322 | 330 | 338 | 340 => {}
                    324 | 332 => {}
                    326 | 334 => {}
                    328 | 336 => {}
                    329 | 337 => {}
                    _ => {}
                }
            }
            240 | 242..=260 => {} //Post-render
            241 => {
                //VBlank, send an NMI to the CPU
                if self.cycle == 1 {
                    let mut ppu_status = self.registers.ppu_status.borrow_mut();
                    (*ppu_status).set_vblank(true);

                    if self.registers.ppu_ctrl.nmi_enable() {
                        cpu.cpu_nmi();
                    }
                }
            } //Post-render
            sl => panic!("Invalid scanline {sl}. Must be between -1 and 260 inclusive"),
        }

        //Draw pixel
        if self.scanline >= 0 && self.scanline < 240 && self.cycle < 256 {
            self.back_buffer[self.scanline as usize][self.cycle as usize] = 0x1A;
        }

        //Increment cycle/scanline
        self.cycle += 1 + ((self.cycle == 339 && self.registers.odd_frame) as u16);
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

    fn read(&self, addr: u16) -> u8 {
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
                let mut data = *self.registers.ppu_data_buffer.borrow();
                let mut loopy_v = self.registers.loopy_v.borrow_mut();

                let loopy_v_raw = (*loopy_v).get_raw();
                *self.registers.ppu_data_buffer.borrow_mut() =
                    self.ppu_bus.read(loopy_v_raw & 0x3fff);

                if loopy_v_raw >= 0x3f00 {
                    data = *self.registers.ppu_data_buffer.borrow();
                }

                if self.registers.ppu_ctrl.increment() {
                    (*loopy_v).increment(32);
                } else {
                    (*loopy_v).increment(1);
                };

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
                let nametable = self.registers.ppu_ctrl.nametable();
                self.registers.loopy_t.set_nametable(nametable);
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
                    let fine_y = data & 0x7;
                    let coarse_y = data >> 3;

                    self.registers.loopy_t.set_coarse_y(coarse_y);
                    self.registers.loopy_t.set_fine_y(fine_y);
                } else {
                    *self.registers.write_latch.borrow_mut() = true;
                    self.registers.fine_x = data & 0x7;
                    let coarse_x = data >> 3;

                    self.registers.loopy_t.set_coarse_x(coarse_x);
                }
            }
            0x6 => {
                let latch = *self.registers.write_latch.borrow();
                if latch {
                    *self.registers.write_latch.borrow_mut() = false;

                    self.registers.loopy_t.set_low_byte(data);
                    *self.registers.loopy_v.borrow_mut() = self.registers.loopy_t;
                } else {
                    *self.registers.write_latch.borrow_mut() = true;
                    self.registers.loopy_t.borrow_mut().set_high_byte(data);
                    self.registers.loopy_t.borrow_mut().set_bit_15_to_0();
                }
            }
            0x7 => {
                let mut loopy_v = self.registers.loopy_v.borrow_mut();
                self.ppu_bus.write((*loopy_v).get_raw() & 0x3fff, data);

                if self.registers.ppu_ctrl.increment() {
                    (*loopy_v).increment(32);
                } else {
                    (*loopy_v).increment(1);
                };
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

#[cfg(test)]
mod ppu_tests {
    use std::borrow::Borrow;

    use mockall::predicate::eq;

    use crate::{bus::MockBus, ppu::registers::PPUStatus};

    use super::*;

    #[test]
    fn test_ppu_ctrl_write_sets_nt_bits_in_t_reg() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));

        ppu.write(0x2000, 0x3);
        assert_eq!(
            0b0000_1100_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(0x3, ppu.registers.ppu_ctrl.into_bytes()[0]);

        ppu.write(0x2000, 0x0);
        assert_eq!(
            0b0000_0000_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(0x0, ppu.registers.ppu_ctrl.into_bytes()[0]);

        ppu.write(0x2000, 0x2);
        assert_eq!(
            0b0000_1000_0000_0000,
            ppu.registers.loopy_t.borrow().get_raw()
        );
        assert_eq!(0x2, ppu.registers.ppu_ctrl.into_bytes()[0]);
    }

    #[test]
    fn test_ppu_status_read_resets_latch() {
        let ppu = NESPPU::new(Box::new(MockBus::new()));

        *ppu.registers.write_latch.borrow_mut() = true;
        *ppu.registers.ppu_status.borrow_mut() = PPUStatus::from_bytes([0xff]);

        assert_eq!(0xff, ppu.read(0x2002));
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(0x7f, ppu.registers.ppu_status.borrow().into_bytes()[0]);

        assert_eq!(0x7f, ppu.read(0x2002));
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

        assert_eq!(0x0, ppu.read(0x2007));
        assert_eq!(0x2001, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xff, *ppu.registers.ppu_data_buffer.borrow());

        assert_eq!(0xff, ppu.read(0x2007));
        assert_eq!(0x2002, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xee, *ppu.registers.ppu_data_buffer.borrow());

        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0x00, 0x3f]));
        assert_eq!(0xdd, ppu.read(0x2007));
        assert_eq!(0x3f01, ppu.registers.loopy_v.borrow().get_raw());
        assert_eq!(0xdd, *ppu.registers.ppu_data_buffer.borrow());

        ppu.registers.loopy_v = RefCell::new(LoopyRegister::from_bytes([0x00, 0x24]));
        ppu.registers.ppu_ctrl.set_increment(true);
        assert_eq!(0xdd, ppu.read(0x2007));
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
