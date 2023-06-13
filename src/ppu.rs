use mockall::automock;

use crate::bus::Bus;

use self::registers::Registers;

mod registers;

#[automock]
pub trait PPU {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn reset(&mut self);
}

pub struct NESPPU<'a> {
    registers: Registers,
    _ppu_bus: Box<dyn Bus + 'a>,
}

impl<'a> NESPPU<'a> {
    pub fn new(_ppu_bus: Box<dyn Bus + 'a>) -> Self {
        NESPPU {
            registers: Registers::new(),
            _ppu_bus,
        }
    }
}

impl PPU for NESPPU<'_> {
    fn read(&self, addr: u16) -> u8 {
        assert!((0x2000..=0x3fff).contains(&addr) || addr == 0x4014);

        if addr == 0x4014 {
            return 0x0; //OAMDMA is write-only
        }

        let offset: u8 = ((addr - 0x2000) & 0x7) as u8;
        match offset {
            0x0 => 0x0, //PPUCTRL is write-only
            0x1 => 0x0, //PPUMASK is write-only
            0x2 => {
                self.registers.scroll_latch.replace(false);
                self.registers.addr_latch.replace(false);

                self.registers.ppu_status
            }
            0x3 => 0x0, //OAMADDR is write-only,
            0x4 => self.registers.oam_data,
            0x5 => 0x0, //PPUSCROLL is write-only
            0x6 => 0x0, //PPUADDR is write-only
            0x7 => self.registers.ppu_data,
            _ => panic!("Register {offset} is invalid, must be from 0x0 to 0x7"),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        assert!((0x2000..=0x3fff).contains(&addr) || addr == 0x4014);

        if addr == 0x4014 {
            self.registers.oam_dma = data;
            return;
        }

        let offset: u8 = ((addr - 0x2000) & 0x7) as u8;
        match offset {
            0x0 => self.registers.ppu_ctrl = data,
            0x1 => self.registers.ppu_mask = data,
            0x2 => {} //PPUSTATUS is read-only
            0x3 => self.registers.oam_addr = data,
            0x4 => self.registers.oam_data = data,
            0x5 => {
                if *self.registers.scroll_latch.borrow() {
                    self.registers.ppu_scroll_y = data;
                    self.registers.scroll_latch.replace(false);
                } else {
                    self.registers.ppu_scroll_x = data;
                    self.registers.scroll_latch.replace(true);
                }
            }
            0x6 => {
                if *self.registers.addr_latch.borrow() {
                    self.registers.ppu_addr = self.registers.ppu_addr & 0xFF00 | data as u16;
                    self.registers.addr_latch.replace(false);
                } else {
                    self.registers.ppu_addr = self.registers.ppu_addr & 0xff | (data as u16) << 8;
                    self.registers.addr_latch.replace(true);
                }
            }
            0x7 => self.registers.ppu_data = data,
            _ => panic!("Register {offset} is invalid, must be from 0x0 to 0x7"),
        }
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod ppu_tests {
    use std::cell::RefCell;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_ppu_read_ppu_ctrl() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_ctrl = 0xff;

        assert_eq!(0x0, ppu.read(0x2000));
        assert_eq!(0x0, ppu.read(0x2008));
        assert_eq!(0x0, ppu.read(0x2010));
    }

    #[test]
    fn test_ppu_read_ppu_mask() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_mask = 0xff;

        assert_eq!(0x0, ppu.read(0x2001));
        assert_eq!(0x0, ppu.read(0x2009));
        assert_eq!(0x0, ppu.read(0x2011));
    }

    #[test]
    fn test_ppu_read_ppu_status() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.addr_latch = RefCell::new(true);
        ppu.registers.scroll_latch = RefCell::new(true);
        ppu.registers.ppu_status = 0xff;

        assert_eq!(0xff, ppu.read(0x2002));
        assert_eq!(0xff, ppu.read(0x200A));
        assert_eq!(0xff, ppu.read(0x2012));

        assert_eq!(false, *ppu.registers.addr_latch.borrow());
        assert_eq!(false, *ppu.registers.scroll_latch.borrow());
    }

    #[test]
    fn test_ppu_read_oam_addr() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.oam_addr = 0xff;

        assert_eq!(0x0, ppu.read(0x2003));
        assert_eq!(0x0, ppu.read(0x200B));
        assert_eq!(0x0, ppu.read(0x2013));
    }
    #[test]
    fn test_ppu_read_oam_data() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.oam_data = 0xff;

        assert_eq!(0xff, ppu.read(0x2004));
        assert_eq!(0xff, ppu.read(0x200C));
        assert_eq!(0xff, ppu.read(0x2014));
    }

    #[test]
    fn test_ppu_read_ppu_scroll() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_scroll_x = 0xff;
        ppu.registers.ppu_scroll_y = 0xff;

        assert_eq!(0x0, ppu.read(0x2005));
        assert_eq!(0x0, ppu.read(0x200D));
        assert_eq!(0x0, ppu.read(0x2015));
        assert_eq!(false, *ppu.registers.scroll_latch.borrow());
    }

    #[test]
    fn test_ppu_read_ppu_addr() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_addr = 0xff;

        assert_eq!(0x0, ppu.read(0x2006));
        assert_eq!(0x0, ppu.read(0x200E));
        assert_eq!(0x0, ppu.read(0x2016));
        assert_eq!(false, *ppu.registers.addr_latch.borrow());
    }

    #[test]
    fn test_ppu_read_ppu_data() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_data = 0xff;

        assert_eq!(0xff, ppu.read(0x2007));
        assert_eq!(0xff, ppu.read(0x200F));
        assert_eq!(0xff, ppu.read(0x2017));
    }

    #[test]
    fn test_ppu_read_oam_dma() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.oam_data = 0xff;

        assert_eq!(0x0, ppu.read(0x4014));
    }

    #[test]
    fn test_ppu_write_ppu_ctrl() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2000, 0xff);
        ppu.write(0x2008, 0xee);
        ppu.write(0x2010, 0xdd);

        assert_eq!(0xdd, ppu.registers.ppu_ctrl);
    }

    #[test]
    fn test_ppu_write_ppu_mask() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2001, 0xff);
        ppu.write(0x2009, 0xee);
        ppu.write(0x2011, 0xdd);

        assert_eq!(0xdd, ppu.registers.ppu_mask);
    }

    #[test]
    fn test_ppu_write_ppu_status() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2002, 0xff);
        ppu.write(0x200A, 0xee);
        ppu.write(0x2012, 0xdd);

        assert_eq!(0x0, ppu.registers.ppu_status);
    }

    #[test]
    fn test_ppu_write_omm_addr() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2003, 0xff);
        ppu.write(0x200B, 0xee);
        ppu.write(0x2013, 0xdd);

        assert_eq!(0xdd, ppu.registers.oam_addr);
    }

    #[test]
    fn test_ppu_write_omm_data() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.write(0x2004, 0xff);
        ppu.write(0x200C, 0xee);
        ppu.write(0x2014, 0xdd);

        assert_eq!(0xdd, ppu.registers.oam_data);
    }

    #[test]
    fn test_ppu_write_ppu_scroll() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_scroll_x = 0xff;
        ppu.registers.ppu_scroll_y = 0xff;

        ppu.write(0x2005, 0x11);

        assert_eq!(0x11, ppu.registers.ppu_scroll_x);
        assert_eq!(0xff, ppu.registers.ppu_scroll_y);
        assert_eq!(true, *ppu.registers.scroll_latch.borrow());

        ppu.write(0x2005, 0x22);

        assert_eq!(0x11, ppu.registers.ppu_scroll_x);
        assert_eq!(0x22, ppu.registers.ppu_scroll_y);
        assert_eq!(false, *ppu.registers.scroll_latch.borrow());

        ppu.write(0x2005, 0x33);

        assert_eq!(0x33, ppu.registers.ppu_scroll_x);
        assert_eq!(0x22, ppu.registers.ppu_scroll_y);
        assert_eq!(true, *ppu.registers.scroll_latch.borrow());
    }

    #[test]
    fn test_ppu_write_ppu_addr() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));
        ppu.registers.ppu_addr = 0xeeee;

        ppu.write(0x2006, 0x11);

        assert_eq!(0x11ee, ppu.registers.ppu_addr);
        assert_eq!(true, *ppu.registers.addr_latch.borrow());

        ppu.write(0x2006, 0x22);

        assert_eq!(0x1122, ppu.registers.ppu_addr);
        assert_eq!(false, *ppu.registers.addr_latch.borrow());

        ppu.write(0x2006, 0x33);

        assert_eq!(0x3322, ppu.registers.ppu_addr);
        assert_eq!(true, *ppu.registers.addr_latch.borrow());
    }

    #[test]
    fn test_ppu_write_ppu_data() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));

        ppu.write(0x2007, 0xff);
        ppu.write(0x200F, 0xee);
        ppu.write(0x2017, 0xdd);

        assert_eq!(0xdd, ppu.registers.ppu_data);
    }

    #[test]
    fn test_ppu_write_oam_dma() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));

        ppu.write(0x4014, 0xff);

        assert_eq!(0xff, ppu.registers.oam_dma);
    }
}
