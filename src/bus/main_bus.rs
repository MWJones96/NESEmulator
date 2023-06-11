use std::rc::Rc;

use crate::{
    cartridge::Cartridge,
    cpu::{bus::CPUBus, CPU},
    ppu::PPU,
};
pub struct MainBus<'a> {
    ppu: Box<dyn PPU + 'a>,
    cartridge: Rc<dyn Cartridge + 'a>,
    ram: [u8; 0x800],
}

impl<'a> MainBus<'a> {
    pub fn new(ppu: Box<dyn PPU + 'a>, cartridge: Rc<dyn Cartridge + 'a>) -> Self {
        Self {
            ppu,
            cartridge,
            ram: [0; 0x800],
        }
    }

    pub fn clock(&mut self, cpu: &mut dyn CPU) {
        cpu.clock(self);
    }

    pub fn reset(&mut self, cpu: &mut dyn CPU) {
        cpu.cpu_reset();
        self.ppu.reset();
    }
}

impl CPUBus for MainBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.ram[(addr & 0x7ff) as usize],
            0x2000..=0x3fff | 0x4014 => self.ppu.read(addr),
            0x8000..=0xffff => Cartridge::read(self.cartridge.as_ref(), addr),
            _ => 0x0, //Open Bus Read
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1fff => {
                self.ram[(addr & 0x7ff) as usize] = data;
            }
            0x2000..=0x3fff | 0x4014 => self.ppu.write(addr, data),
            0x8000..=0xffff => self.cartridge.write(addr, data),
            _ => {} //Open Bus Write
        }
    }
}

#[cfg(test)]
mod main_bus_tests {
    use mockall::predicate::eq;

    use crate::{cartridge::MockCartridge, ppu::MockPPU};

    use super::*;

    #[test]
    fn test_cpu_bus_read() {
        let cartridge = MockCartridge::new();
        let mut main_bus = MainBus::new(Box::new(MockPPU::new()), Rc::new(cartridge));

        main_bus.ram[0x0] = 0xff;

        assert_eq!(0xff, main_bus.read(0x0));
        assert_eq!(0xff, main_bus.read(0x800));
        assert_eq!(0xff, main_bus.read(0x1000));
        assert_eq!(0xff, main_bus.read(0x1800));
    }

    #[test]
    fn test_cpu_bus_write() {
        let cartridge = MockCartridge::new();
        let mut main_bus = MainBus::new(Box::new(MockPPU::new()), Rc::new(cartridge));

        main_bus.write(0x1, 0x34);
        assert_eq!(0x34, main_bus.ram[0x1]);

        main_bus.write(0x801, 0x35);
        assert_eq!(0x35, main_bus.ram[0x1]);

        main_bus.write(0x1001, 0x36);
        assert_eq!(0x36, main_bus.ram[0x1]);

        main_bus.write(0x1001, 0x37);
        assert_eq!(0x37, main_bus.ram[0x1]);
    }

    #[test]
    fn test_cartridge_read() {
        let mut cartridge = MockCartridge::new();

        cartridge.expect_read().with(eq(0x7fff)).never();

        cartridge
            .expect_read()
            .with(eq(0x8000))
            .once()
            .return_const(0x0);

        cartridge
            .expect_read()
            .with(eq(0xffff))
            .once()
            .return_const(0x0);

        let main_bus = MainBus::new(Box::new(MockPPU::new()), Rc::new(cartridge));

        main_bus.read(0x7fff);
        main_bus.read(0x8000);
        main_bus.read(0xffff);
    }

    #[test]
    fn test_cartridge_write() {
        let mut cartridge = MockCartridge::new();

        cartridge.expect_write().with(eq(0x7fff), eq(0x0)).never();

        cartridge
            .expect_write()
            .with(eq(0x8000), eq(0x0))
            .once()
            .return_const(());

        cartridge
            .expect_write()
            .with(eq(0xffff), eq(0x0))
            .once()
            .return_const(());

        let mut main_bus = MainBus::new(Box::new(MockPPU::new()), Rc::new(cartridge));

        main_bus.write(0x7fff, 0x0);
        main_bus.write(0x8000, 0x0);
        main_bus.write(0xffff, 0x0);
    }

    #[test]
    fn test_ppu_read() {
        let mut ppu = MockPPU::new();
        let mut cartridge = MockCartridge::new();
        cartridge.expect_read().return_const(0x0);

        ppu.expect_read().with(eq(0x2000)).once().return_const(0x0);
        ppu.expect_read().with(eq(0x3fff)).once().return_const(0x0);
        ppu.expect_read().with(eq(0x1fff)).never().return_const(0x0);
        ppu.expect_read().with(eq(0x4000)).never().return_const(0x0);
        ppu.expect_read().with(eq(0x4014)).once().return_const(0x0);

        let main_bus = MainBus::new(Box::new(ppu), Rc::new(cartridge));
        main_bus.read(0x2000);
        main_bus.read(0x3fff);
        main_bus.read(0x1fff);
        main_bus.read(0x4000);
        main_bus.read(0x4014);
    }

    #[test]
    fn test_ppu_write() {
        let mut ppu = MockPPU::new();
        let mut cartridge = MockCartridge::new();
        cartridge.expect_write().return_const(());

        ppu.expect_write()
            .with(eq(0x2000), eq(0x0))
            .once()
            .return_const(());
        ppu.expect_write()
            .with(eq(0x3fff), eq(0x0))
            .once()
            .return_const(());
        ppu.expect_write()
            .with(eq(0x1fff), eq(0x0))
            .never()
            .return_const(());
        ppu.expect_write()
            .with(eq(0x4000), eq(0x0))
            .never()
            .return_const(());
        ppu.expect_write()
            .with(eq(0x4014), eq(0x0))
            .once()
            .return_const(());

        let mut main_bus = MainBus::new(Box::new(ppu), Rc::new(cartridge));
        main_bus.write(0x2000, 0x0);
        main_bus.write(0x3fff, 0x0);
        main_bus.write(0x1fff, 0x0);
        main_bus.write(0x4000, 0x0);
        main_bus.write(0x4014, 0x0);
    }
}
