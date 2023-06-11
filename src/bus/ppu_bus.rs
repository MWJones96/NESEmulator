use std::rc::Rc;

use crate::cartridge::Cartridge;

use super::Bus;

pub struct PPUBus<'a> {
    cartridge: Rc<dyn Cartridge + 'a>,
}

impl<'a> PPUBus<'a> {
    pub fn new(cartridge: Rc<dyn Cartridge + 'a>) -> Self {
        PPUBus { cartridge }
    }
}

impl Bus for PPUBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        assert!(addr <= 0x3fff);
        0
    }

    fn write(&mut self, addr: u16, data: u8) {
        assert!(addr <= 0x3fff)
    }
}

#[cfg(test)]
mod ppu_bus_tests {
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
}
