use self::registers::Registers;

mod registers;

pub struct PPU {
    registers: Registers,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            registers: Registers::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
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

    pub fn get_screen(&self) -> &[[u8; 256]; 240] {
        &[[0x0; 256]; 240] //Black screen
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod ppu_tests {
    use std::cell::RefCell;

    use super::*;

    #[test]
    fn test_get_screen() {
        let ppu = PPU::new();
        let expected = &[[0x0u8; 256]; 240];

        assert_eq!(expected, ppu.get_screen());
    }

    #[test]
    fn test_ppu_read_ppu_ctrl() {
        let mut ppu = PPU::default();
        ppu.registers.ppu_ctrl = 0xff;

        assert_eq!(0x0, ppu.read(0x2000));
        assert_eq!(0x0, ppu.read(0x2008));
        assert_eq!(0x0, ppu.read(0x2010));
    }

    #[test]
    fn test_ppu_read_ppu_mask() {
        let mut ppu = PPU::new();
        ppu.registers.ppu_mask = 0xff;

        assert_eq!(0x0, ppu.read(0x2001));
        assert_eq!(0x0, ppu.read(0x2009));
        assert_eq!(0x0, ppu.read(0x2011));
    }

    #[test]
    fn test_ppu_read_ppu_status() {
        let mut ppu = PPU::new();
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
        let mut ppu = PPU::new();
        ppu.registers.oam_addr = 0xff;

        assert_eq!(0x0, ppu.read(0x2003));
        assert_eq!(0x0, ppu.read(0x200B));
        assert_eq!(0x0, ppu.read(0x2013));
    }
    #[test]
    fn test_ppu_read_oam_data() {
        let mut ppu = PPU::new();
        ppu.registers.oam_data = 0xff;

        assert_eq!(0xff, ppu.read(0x2004));
        assert_eq!(0xff, ppu.read(0x200C));
        assert_eq!(0xff, ppu.read(0x2014));
    }

    #[test]
    fn test_ppu_read_ppu_scroll() {
        let mut ppu = PPU::new();
        ppu.registers.ppu_scroll_x = 0xff;
        ppu.registers.ppu_scroll_y = 0xff;

        assert_eq!(0x0, ppu.read(0x2005));
        assert_eq!(0x0, ppu.read(0x200D));
        assert_eq!(0x0, ppu.read(0x2015));
        assert_eq!(false, *ppu.registers.scroll_latch.borrow());
    }

    #[test]
    fn test_ppu_read_ppu_addr() {
        let mut ppu = PPU::new();
        ppu.registers.ppu_addr = 0xff;

        assert_eq!(0x0, ppu.read(0x2006));
        assert_eq!(0x0, ppu.read(0x200E));
        assert_eq!(0x0, ppu.read(0x2016));
        assert_eq!(false, *ppu.registers.addr_latch.borrow());
    }

    #[test]
    fn test_ppu_read_ppu_data() {
        let mut ppu = PPU::new();
        ppu.registers.ppu_data = 0xff;

        assert_eq!(0xff, ppu.read(0x2007));
        assert_eq!(0xff, ppu.read(0x200F));
        assert_eq!(0xff, ppu.read(0x2017));
    }

    #[test]
    fn test_ppu_read_oam_dma() {
        let mut ppu = PPU::new();
        ppu.registers.oam_data = 0xff;

        assert_eq!(0x0, ppu.read(0x4014));
    }
}
