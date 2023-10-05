use std::{cell::RefCell, rc::Rc};

use crate::{
    cartridge::Cartridge,
    controller::Controller,
    cpu::CPU,
    ppu::{Frame, PPU},
};

#[derive(Debug, PartialEq, Copy, Clone)]
struct DMA {
    cycles: i16,
    page: u8,
}

use super::Bus;
pub struct CPUBus<'a> {
    ppu: Box<dyn PPU + 'a>,
    cartridge: Rc<dyn Cartridge + 'a>,

    controller_1: Rc<RefCell<dyn Controller + 'a>>,
    controller_2: Rc<RefCell<dyn Controller + 'a>>,

    dma: Option<DMA>,

    ram: [u8; 0x800],

    #[allow(arithmetic_overflow)]
    nes_cycles: u64,
}

impl<'a> CPUBus<'a> {
    pub fn new(
        ppu: Box<dyn PPU + 'a>,
        cartridge: Rc<dyn Cartridge + 'a>,
        controller_1: Rc<RefCell<dyn Controller + 'a>>,
        controller_2: Rc<RefCell<dyn Controller + 'a>>,
    ) -> Self {
        Self {
            ppu,
            cartridge,

            controller_1,
            controller_2,

            dma: None,

            ram: [0; 0x800],
            nes_cycles: 0,
        }
    }

    pub fn clock(&mut self, cpu: &mut dyn CPU) {
        self.nes_cycles += 1;
        self.ppu.clock(cpu);

        if self.nes_cycles % 3 == 0 {
            if let Some(mut dma) = self.dma {
                dma.cycles -= 1;
                if dma.cycles <= 510 && dma.cycles % 2 == 0 {
                    let index = ((512 - (dma.cycles + 2)) / 2) as u8;
                    let addr: u16 = ((dma.page as u16) << 8) | (index as u16);
                    self.ppu.write(0x2004, self.read(addr));
                }
                self.dma = if dma.cycles <= 0 { None } else { Some(dma) };
            } else {
                cpu.clock(self);
            }
        }
    }

    pub fn reset(&mut self, cpu: &mut dyn CPU) {
        cpu.cpu_reset();
        self.ppu.reset();
    }

    pub fn is_frame_completed(&self) -> bool {
        self.ppu.is_frame_completed()
    }

    pub fn get_frame_from_ppu(&self) -> Frame {
        self.ppu.get_frame()
    }
}

impl Bus for CPUBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.ram[(addr & 0x7ff) as usize],
            0x2000..=0x3fff => self.ppu.read(addr, false),
            0x4016 => {
                let c1 = (*self.controller_1.as_ref()).borrow();
                c1.read()
            }
            0x4017 => {
                let c2 = (*self.controller_2.as_ref()).borrow();
                c2.read()
            }
            0x8000..=0xffff => self.cartridge.cpu_read(addr),
            _ => 0x0, //Open Bus Read
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1fff => {
                self.ram[(addr & 0x7ff) as usize] = data;
            }
            0x2000..=0x3fff => self.ppu.write(addr, data),
            0x4014 => {
                self.dma = Some(DMA {
                    cycles: 513,
                    page: data,
                });
                self.ppu.write(addr, data);
            }
            0x4016 => {
                let mut c1 = (*self.controller_1.as_ref()).borrow_mut();
                let mut c2 = (*self.controller_2.as_ref()).borrow_mut();
                c1.write(data);
                c2.write(data);
            }
            0x8000..=0xffff => self.cartridge.cpu_write(addr, data),
            _ => {} //Open Bus Write
        }
    }
}

#[cfg(test)]
mod cpu_bus_tests {
    use mockall::predicate::eq;

    use crate::{
        bus::MockBus,
        cartridge::MockCartridge,
        controller::MockController,
        cpu::MockCPU,
        ppu::{MockPPU, NESPPU},
    };

    use super::*;

    #[test]
    fn test_cpu_bus_read() {
        let cartridge = MockCartridge::new();
        let mut main_bus = CPUBus::new(
            Box::new(MockPPU::new()),
            Rc::new(cartridge),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );

        main_bus.ram[0x0] = 0xff;

        assert_eq!(0xff, main_bus.read(0x0));
        assert_eq!(0xff, main_bus.read(0x800));
        assert_eq!(0xff, main_bus.read(0x1000));
        assert_eq!(0xff, main_bus.read(0x1800));
    }

    #[test]
    fn test_cpu_bus_write() {
        let cartridge = MockCartridge::new();
        let mut main_bus = CPUBus::new(
            Box::new(MockPPU::new()),
            Rc::new(cartridge),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );

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

        cartridge.expect_cpu_read().with(eq(0x7fff)).never();

        cartridge
            .expect_cpu_read()
            .with(eq(0x8000))
            .once()
            .return_const(0x0);

        cartridge
            .expect_cpu_read()
            .with(eq(0xffff))
            .once()
            .return_const(0x0);

        let main_bus = CPUBus::new(
            Box::new(MockPPU::new()),
            Rc::new(cartridge),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );

        main_bus.read(0x7fff);
        main_bus.read(0x8000);
        main_bus.read(0xffff);
    }

    #[test]
    fn test_cartridge_write() {
        let mut cartridge = MockCartridge::new();

        cartridge
            .expect_cpu_write()
            .with(eq(0x7fff), eq(0x0))
            .never();

        cartridge
            .expect_cpu_write()
            .with(eq(0x8000), eq(0x0))
            .once()
            .return_const(());

        cartridge
            .expect_cpu_write()
            .with(eq(0xffff), eq(0x0))
            .once()
            .return_const(());

        let mut main_bus = CPUBus::new(
            Box::new(MockPPU::new()),
            Rc::new(cartridge),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );

        main_bus.write(0x7fff, 0x0);
        main_bus.write(0x8000, 0x0);
        main_bus.write(0xffff, 0x0);
    }

    #[test]
    fn test_ppu_read() {
        let mut ppu = MockPPU::new();
        let mut cartridge = MockCartridge::new();
        cartridge.expect_cpu_read().return_const(0x0);

        ppu.expect_read()
            .with(eq(0x2000), eq(false))
            .once()
            .return_const(0x0);
        ppu.expect_read()
            .with(eq(0x3fff), eq(false))
            .once()
            .return_const(0x0);
        ppu.expect_read()
            .with(eq(0x1fff), eq(false))
            .never()
            .return_const(0x0);
        ppu.expect_read()
            .with(eq(0x4000), eq(false))
            .never()
            .return_const(0x0);

        let main_bus = CPUBus::new(
            Box::new(ppu),
            Rc::new(cartridge),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );
        main_bus.read(0x2000);
        main_bus.read(0x3fff);
        main_bus.read(0x1fff);
        main_bus.read(0x4000);
    }

    #[test]
    fn test_ppu_write() {
        let mut ppu = MockPPU::new();
        let mut cartridge = MockCartridge::new();
        cartridge.expect_cpu_write().return_const(());

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

        let mut main_bus = CPUBus::new(
            Box::new(ppu),
            Rc::new(cartridge),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );
        main_bus.write(0x2000, 0x0);
        main_bus.write(0x3fff, 0x0);
        main_bus.write(0x1fff, 0x0);
        main_bus.write(0x4000, 0x0);
        main_bus.write(0x4014, 0x0);
    }

    #[test]
    fn test_dma_init() {
        let mut ppu = NESPPU::new(Box::new(MockBus::new()));

        let mut main_bus = CPUBus::new(
            Box::new(ppu),
            Rc::new(MockCartridge::new()),
            Rc::new(RefCell::new(MockController::new())),
            Rc::new(RefCell::new(MockController::new())),
        );
        assert_eq!(None, main_bus.dma);
        main_bus.write(0x4014, 0x03);
        assert_eq!(
            Some(DMA {
                cycles: 513,
                page: 0x03
            }),
            main_bus.dma
        );

        main_bus.write(0x0300, 0xff);

        main_bus.write(0x0304, 0xaa);
        main_bus.write(0x0305, 0xbb);
        main_bus.write(0x0306, 0xcc);
        main_bus.write(0x0307, 0xdd);

        main_bus.write(0x03ff, 0xff);

        for _ in 0..513 * 3 {
            let mut cpu = MockCPU::new();
            cpu.expect_clock().never();

            main_bus.clock(&mut cpu);
        }
        assert_eq!(None, main_bus.dma);

        main_bus.write(0x2003, 0x0);
        assert_eq!(0xff, main_bus.read(0x2004));

        main_bus.write(0x2003, 0x4);
        assert_eq!(0xaa, main_bus.read(0x2004));
        main_bus.write(0x2003, 0x5);
        assert_eq!(0xbb, main_bus.read(0x2004));
        main_bus.write(0x2003, 0x6);
        assert_eq!(0xcc, main_bus.read(0x2004));
        main_bus.write(0x2003, 0x7);
        assert_eq!(0xdd, main_bus.read(0x2004));

        main_bus.write(0x2003, 0xff);
        assert_eq!(0xff, main_bus.read(0x2004));
    }
}
