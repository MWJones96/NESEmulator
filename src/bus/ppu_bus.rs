use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::util::Mirroring;

use super::Bus;

pub struct PPUBus<'a> {
    cartridge: Rc<dyn Cartridge + 'a>,
    nametable_0: [u8; 0x400],
    nametable_1: [u8; 0x400],
    palette: [u8; 0x20],
}

impl<'a> PPUBus<'a> {
    pub fn new(cartridge: Rc<dyn Cartridge + 'a>) -> Self {
        PPUBus {
            cartridge,
            nametable_0: [0x0; 0x400],
            nametable_1: [0x0; 0x400],
            palette: [0x0; 0x20],
        }
    }
}

impl Bus for PPUBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        assert!(addr <= 0x3fff);
        match addr {
            0x0000..=0x1fff => self.cartridge.ppu_read(addr),
            0x2000..=0x23ff => self.nametable_0[(addr - 0x2000) as usize],
            0x2400..=0x27ff => match self.cartridge.get_mirroring() {
                Mirroring::HORIZONTAL => self.nametable_0[(addr - 0x2400) as usize],
                Mirroring::VERTICAL => self.nametable_1[(addr - 0x2400) as usize],
            },
            0x2800..=0x2bff => match self.cartridge.get_mirroring() {
                Mirroring::HORIZONTAL => self.nametable_1[(addr - 0x2800) as usize],
                Mirroring::VERTICAL => self.nametable_0[(addr - 0x2800) as usize],
            },
            0x2c00..=0x2fff => self.nametable_1[(addr - 0x2c00) as usize],
            0x3000..=0x3eff => self.read(addr - 0x1000),
            0x3f00..=0x3fff => {
                let mut offset = addr - 0x3f00;
                if offset % 4 == 0 {
                    offset = 0;
                }

                self.palette[offset as usize]
            }
            _ => 0x0, //Open bus read
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        assert!(addr <= 0x3fff);
        match addr {
            0x0000..=0x1fff => self.cartridge.ppu_write(addr, data),
            0x2000..=0x23ff => {
                self.nametable_0[(addr - 0x2000) as usize] = data;
            }
            0x2400..=0x27ff => match self.cartridge.get_mirroring() {
                Mirroring::HORIZONTAL => self.nametable_0[(addr - 0x2400) as usize] = data,
                Mirroring::VERTICAL => self.nametable_1[(addr - 0x2400) as usize] = data,
            },
            0x2800..=0x2bff => match self.cartridge.get_mirroring() {
                Mirroring::HORIZONTAL => self.nametable_1[(addr - 0x2800) as usize] = data,
                Mirroring::VERTICAL => self.nametable_0[(addr - 0x2800) as usize] = data,
            },
            0x2c00..=0x2fff => self.nametable_1[(addr - 0x2c00) as usize] = data,
            0x3000..=0x3eff => self.write(addr - 0x1000, data),
            0x3f00..=0x3fff => {
                let mut offset = addr - 0x3f00;
                if offset % 4 == 0 {
                    offset = 0;
                }

                self.palette[offset as usize] = data;
            }
            _ => {} //Open bus write
        }
    }
}

#[cfg(test)]
mod ppu_bus_tests {
    use mockall::predicate::eq;

    use crate::{cartridge::MockCartridge, util::Mirroring};

    use super::*;

    #[test]
    #[should_panic]
    fn test_ppu_bus_read_panics_on_addr_out_of_range() {
        let cartridge = Rc::new(MockCartridge::new());
        let ppu_bus = PPUBus::new(cartridge);

        ppu_bus.read(0x4000);
    }

    #[test]
    #[should_panic]
    fn test_ppu_bus_write_panics_on_addr_out_of_range() {
        let cartridge = Rc::new(MockCartridge::new());
        let mut ppu_bus = PPUBus::new(cartridge);

        ppu_bus.write(0x4000, 0x0);
    }

    #[test]
    fn test_ppu_bus_reads_from_chr() {
        let mut cartridge = MockCartridge::new();

        cartridge
            .expect_ppu_read()
            .with(eq(0x0))
            .once()
            .return_const(0x0);

        cartridge
            .expect_ppu_read()
            .with(eq(0x1fff))
            .once()
            .return_const(0x0);

        cartridge
            .expect_ppu_read()
            .with(eq(0x2000))
            .never()
            .return_const(0x0);

        let ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.read(0x0);
        ppu_bus.read(0x1fff);
        ppu_bus.read(0x2000);
    }

    #[test]
    fn test_ppu_bus_writes_to_chr() {
        let mut cartridge = MockCartridge::new();

        cartridge
            .expect_ppu_write()
            .with(eq(0x0), eq(0x0))
            .once()
            .return_const(());

        cartridge
            .expect_ppu_write()
            .with(eq(0x1fff), eq(0x0))
            .once()
            .return_const(());

        cartridge
            .expect_ppu_write()
            .with(eq(0x2000), eq(0x0))
            .never()
            .return_const(());

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.write(0x0, 0x0);
        ppu_bus.write(0x1fff, 0x0);
        ppu_bus.write(0x2000, 0x0);
    }

    #[test]
    fn test_read_from_nametable_0() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .return_const(Mirroring::VERTICAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.nametable_0[0x0] = 0xff;
        ppu_bus.nametable_0[0x3ff] = 0xff;

        assert_eq!(0xff, ppu_bus.read(0x2000));
        assert_eq!(0xff, ppu_bus.read(0x23ff));
        assert_eq!(0x0, ppu_bus.read(0x2400));
    }

    #[test]
    fn test_write_to_nametable_0() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.write(0x2000, 0xff);
        ppu_bus.write(0x23ff, 0xff);

        assert_eq!(0xff, ppu_bus.nametable_0[0x0]);
        assert_eq!(0xff, ppu_bus.nametable_0[0x3ff]);
    }

    #[test]
    fn test_read_from_nametable_1() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.nametable_1[0x0] = 0xff;
        ppu_bus.nametable_1[0x3ff] = 0xff;

        assert_eq!(0xff, ppu_bus.read(0x2c00));
        assert_eq!(0xff, ppu_bus.read(0x2fff));
        assert_eq!(0x0, ppu_bus.read(0x3000));
    }

    #[test]
    fn test_write_to_nametable_1() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.write(0x2c00, 0xff);
        ppu_bus.write(0x2fff, 0xff);

        assert_eq!(0xff, ppu_bus.nametable_1[0x0]);
        assert_eq!(0xff, ppu_bus.nametable_1[0x3ff]);
    }

    #[test]
    fn test_read_mirrored_addr() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.nametable_0[0x0] = 0xff;
        ppu_bus.nametable_1[0x2ff] = 0xff;

        assert_eq!(0xff, ppu_bus.read(0x3000));
        assert_eq!(0xff, ppu_bus.read(0x3eff));
    }

    #[test]
    fn test_write_to_mirrored_addr() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.write(0x3000, 0xff);
        ppu_bus.write(0x3eff, 0xff);

        assert_eq!(0xff, ppu_bus.nametable_0[0x0]);
        assert_eq!(0xff, ppu_bus.nametable_1[0x2ff]);
    }

    #[test]
    fn test_read_from_palette_ram() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.palette[0x0] = 0xff;
        ppu_bus.palette[0x1] = 0xee;
        ppu_bus.palette[0x2] = 0xdd;
        ppu_bus.palette[0x3] = 0xcc;
        ppu_bus.palette[0x4] = 0x00;
        ppu_bus.palette[0x5] = 0xbb;

        assert_eq!(0xff, ppu_bus.read(0x3f00));
        assert_eq!(0xee, ppu_bus.read(0x3f01));
        assert_eq!(0xdd, ppu_bus.read(0x3f02));
        assert_eq!(0xcc, ppu_bus.read(0x3f03));
        assert_eq!(0xff, ppu_bus.read(0x3f04));
        assert_eq!(0xbb, ppu_bus.read(0x3f05));
        assert_eq!(0xff, ppu_bus.read(0x3f20));
    }

    #[test]
    fn test_write_to_palette_addr() {
        let cartridge = MockCartridge::new();
        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));

        ppu_bus.write(0x3f00, 0xff);
        ppu_bus.write(0x3f01, 0xee);
        ppu_bus.write(0x3f02, 0xdd);
        ppu_bus.write(0x3f03, 0xcc);
        ppu_bus.write(0x3f04, 0xbb);

        assert_eq!(0xbb, ppu_bus.palette[0x0]);
        assert_eq!(0xee, ppu_bus.palette[0x1]);
        assert_eq!(0xdd, ppu_bus.palette[0x2]);
        assert_eq!(0xcc, ppu_bus.palette[0x3]);

        ppu_bus.write(0x3f20, 0xff);
        assert_eq!(0xff, ppu_bus.palette[0x0]);
    }

    #[test]
    fn test_read_from_logical_nametable_2400_horizontal_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::HORIZONTAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.nametable_0[0x0] = 0xff;
        assert_eq!(0xff, ppu_bus.read(0x2400));
    }

    #[test]
    fn test_read_from_logical_nametable_2400_vertical_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::VERTICAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.nametable_1[0x0] = 0xff;
        assert_eq!(0xff, ppu_bus.read(0x2400));
    }

    #[test]
    fn test_read_from_logical_nametable_2800_horizontal_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::HORIZONTAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.nametable_1[0x0] = 0xff;
        assert_eq!(0xff, ppu_bus.read(0x2800));
    }

    #[test]
    fn test_read_from_logical_nametable_2800_vertical_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::VERTICAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.nametable_0[0x0] = 0xff;
        assert_eq!(0xff, ppu_bus.read(0x2800));
    }

    #[test]
    fn test_write_from_logical_nametable_2400_horizontal_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::HORIZONTAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.write(0x2400, 0xff);
        assert_eq!(0xff, ppu_bus.nametable_0[0x0]);
    }

    #[test]
    fn test_write_from_logical_nametable_2400_vertical_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::VERTICAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.write(0x2400, 0xff);
        assert_eq!(0xff, ppu_bus.nametable_1[0x0]);
    }

    #[test]
    fn test_write_from_logical_nametable_2800_horizontal_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::HORIZONTAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.write(0x2800, 0xff);
        assert_eq!(0xff, ppu_bus.nametable_1[0x0]);
    }

    #[test]
    fn test_write_from_logical_nametable_2800_vertical_mirroring() {
        let mut cartridge = MockCartridge::new();
        cartridge
            .expect_get_mirroring()
            .once()
            .return_const(Mirroring::VERTICAL);

        let mut ppu_bus = PPUBus::new(Rc::new(cartridge));
        ppu_bus.write(0x2800, 0xff);
        assert_eq!(0xff, ppu_bus.nametable_0[0x0]);
    }
}
