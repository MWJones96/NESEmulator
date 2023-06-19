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
    _registers: Registers,
    _ppu_bus: Box<dyn Bus + 'a>,
    _ppu_buffer: RefCell<u8>,

    scanline: i16,
    cycle: u16,

    front_buffer: Frame,
    back_buffer: Frame,

    completed_frame: RefCell<bool>,
}

impl<'a> NESPPU<'a> {
    pub fn new(_ppu_bus: Box<dyn Bus + 'a>) -> Self {
        NESPPU {
            _registers: Registers::new(),
            _ppu_bus,

            _ppu_buffer: RefCell::new(0x0),
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
                    0x3f
                } else {
                    0x30
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

        let offset: u8 = ((addr - 0x2000) & 0x7) as u8;
        match offset {
            0x0 => 0x0, //PPUCTRL is write-only
            0x1 => 0x0, //PPUMASK is write-only
            0x2 => 0x0,
            0x3 => 0x0,
            0x4 => 0x0,
            0x5 => 0x0, //PPUSCROLL is write-only
            0x6 => 0x0, //PPUADDR is write-only
            0x7 => 0x0,
            _ => panic!("PPU Register {offset} is invalid, must be from 0x0 to 0x7"),
        }
    }

    fn write(&mut self, addr: u16, _data: u8) {
        assert!((0x2000..=0x3fff).contains(&addr) || addr == 0x4014);

        if addr == 0x4014 {
            return;
        }

        let offset: u8 = ((addr - 0x2000) & 0x7) as u8;
        match offset {
            0x0 => {}
            0x1 => {}
            0x2 => {} //PPUSTATUS is read-only
            0x3 => {}
            0x4 => {}
            0x5 => {}
            0x6 => {}
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
mod ppu_tests {}
