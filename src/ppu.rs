use std::cell::RefCell;

use mockall::automock;
use rand::Rng;

use crate::bus::Bus;

use self::registers::Registers;

mod registers;

pub type Frame = [[u8; 256]; 240];

#[automock]
pub trait PPU {
    fn clock(&mut self);
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

            scanline: 0,
            cycle: 0,

            front_buffer: [[0x0; 256]; 240],
            back_buffer: [[0x0; 256]; 240],

            completed_frame: RefCell::new(false),
        }
    }
}

impl PPU for NESPPU<'_> {
    fn clock(&mut self) {
        if self.scanline >= 0 && self.scanline < 240 && self.cycle < 256 {
            self.back_buffer[self.scanline as usize][self.cycle as usize] =
                if rand::thread_rng().gen_bool(0.5) {
                    0x1A
                } else {
                    0x12
                };
        }

        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
                self.scanline = -1;
                self.completed_frame = RefCell::new(true);
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
                *self.registers.write_latch.borrow_mut() = false;
                0x0
            }
            0x3 => 0x0,
            0x4 => 0x0,
            0x5 => 0x0, //PPUSCROLL is write-only
            0x6 => 0x0, //PPUADDR is write-only
            0x7 => 0x0,
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
                self.registers.loopy_t &= 0b1111_0011_1111_1111;
                let nametable = data & 0x3;
                self.registers.loopy_t |= (nametable as u16) << 10;
            }
            0x1 => {}
            0x2 => {} //PPUSTATUS is read-only
            0x3 => {}
            0x4 => {}
            0x5 => {
                let latch = *self.registers.write_latch.borrow();
                if latch {
                    *self.registers.write_latch.borrow_mut() = false;
                    self.registers.loopy_t &= 0b0001_1000_0011_1111;
                    let fine_y = data & 0x7;
                    let coarse_y = data >> 3;

                    self.registers.loopy_t |= (coarse_y as u16) << 5;
                    self.registers.loopy_t |= (fine_y as u16) << 12;
                } else {
                    *self.registers.write_latch.borrow_mut() = true;
                    self.registers.fine_x = data & 0x7;

                    self.registers.loopy_t &= 0b1111_1111_1110_0000;
                    self.registers.loopy_t |= (data >> 3) as u16;
                }
            }
            0x6 => {
                let latch = *self.registers.write_latch.borrow();
                if latch {
                    *self.registers.write_latch.borrow_mut() = false;
                    self.registers.loopy_t &= 0b1111_1111_0000_0000;
                    self.registers.loopy_t |= data as u16;
                    self.registers.loopy_v = self.registers.loopy_t;
                } else {
                    *self.registers.write_latch.borrow_mut() = true;
                    self.registers.loopy_t &= 0b0000_0000_1111_1111;
                    self.registers.loopy_t |= (data as u16) << 8;
                    self.registers.loopy_t &= 0x3fff;
                }
            }
            0x7 => {}
            _ => panic!("Register {offset} is invalid, must be from 0x0 to 0x7"),
        }
    }

    fn reset(&mut self) {}

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
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_ppu_ctrl_write_sets_nt_bits_in_t_reg() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));

        ppu.write(0x2000, 0x03);
        assert_eq!(0b0000_1100_0000_0000, ppu.registers.loopy_t);

        ppu.write(0x2000, 0x0);
        assert_eq!(0b0000_0000_0000_0000, ppu.registers.loopy_t);

        ppu.write(0x2000, 0x2);
        assert_eq!(0b0000_1000_0000_0000, ppu.registers.loopy_t);
    }

    #[test]
    fn test_ppu_status_read_resets_latch() {
        let ppu = NESPPU::new(Box::new(MockBus::new()));
        *ppu.registers.write_latch.borrow_mut() = true;

        ppu.read(0x2002);
        assert_eq!(false, *ppu.registers.write_latch.borrow());

        ppu.read(0x2002);
        assert_eq!(false, *ppu.registers.write_latch.borrow());
    }

    #[test]
    fn test_ppu_write_scroll() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2005, 0b1000_1101);

        assert_eq!(true, *ppu.registers.write_latch.borrow());
        assert_eq!(0b101, ppu.registers.fine_x);
        assert_eq!(0b10001, ppu.registers.loopy_t);

        ppu.write(0x2005, 0b10001101);
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(0b0101_0010_0011_0001, ppu.registers.loopy_t);
    }

    #[test]
    fn test_ppu_write_addr() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2006, 0xff);

        assert_eq!(true, *ppu.registers.write_latch.borrow());
        assert_eq!(0b0011_1111_0000_0000, ppu.registers.loopy_t);

        ppu.write(0x2006, 0xfe);
        assert_eq!(false, *ppu.registers.write_latch.borrow());
        assert_eq!(0b0011_1111_1111_1110, ppu.registers.loopy_t);
        assert_eq!(0b0011_1111_1111_1110, ppu.registers.loopy_v);
    }
}
