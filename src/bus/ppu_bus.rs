use std::rc::Rc;

use crate::cartridge::Cartridge;

use super::Bus;

pub struct PPUBus<'a> {
    cartridge: Rc<dyn Cartridge + 'a>,
    nametable_0: [u8; 0x400],
    _nametable_1: [u8; 0x400],
    _palette: [u8; 0x20],
}

impl<'a> PPUBus<'a> {
    pub fn new(cartridge: Rc<dyn Cartridge + 'a>) -> Self {
        PPUBus {
            cartridge,
            nametable_0: [0x0; 0x400],
            _nametable_1: [0x0; 0x400],
            _palette: [0x0; 0x20],
        }
    }
}

impl Bus for PPUBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        assert!(addr <= 0x3fff);
        match addr {
            0x0000..=0x1fff => self.cartridge.ppu_read(addr),
            0x2000..=0x23ff => self.nametable_0[(addr - 0x2000) as usize],
            _ => 0x0, //Open bus read
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        assert!(addr <= 0x3fff);
        match addr {
            0x0000..=0x1fff => self.cartridge.ppu_write(addr, data),
            0x2000..=0x23ff => self.nametable_0[(addr - 0x2000) as usize] = data,
            _ => {} //Open bus write
        }
    }
}

#[cfg(test)]
mod ppu_bus_tests {
    use mockall::predicate::eq;

    use crate::cartridge::MockCartridge;

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
        let cartridge = MockCartridge::new();
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
}
