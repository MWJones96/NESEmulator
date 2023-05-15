use std::rc::Rc;

use crate::{
    cpu::{bus::CPUBus, CPU},
    mapper::PRGRomMapper,
};

pub struct MainBus {
    cpu: CPU,
    mapper: Rc<dyn PRGRomMapper>,
    ram: [u8; 0x800],
}

impl MainBus {
    pub fn new(cpu: CPU, mapper: Rc<impl PRGRomMapper + 'static>) -> Self {
        Self {
            cpu,
            mapper,
            ram: [0; 0x800],
        }
    }
}

impl CPUBus for MainBus {
    fn read(&self, addr: u16) -> u8 {
        self.ram[(addr & 0x7ff) as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[(addr & 0x7ff) as usize] = data;
    }
}

#[cfg(test)]
mod main_bus_tests {
    use crate::mapper::MockPRGRomMapper;

    use super::*;

    #[test]
    fn test_cpu_bus_read() {
        let cpu = CPU::new();
        let mapper = MockPRGRomMapper::new();
        let mut main_bus = MainBus::new(cpu, Rc::new(mapper));

        main_bus.ram[0x0] = 0xff;

        assert_eq!(0xff, main_bus.read(0x0));
        assert_eq!(0xff, main_bus.read(0x800));
        assert_eq!(0xff, main_bus.read(0x1000));
        assert_eq!(0xff, main_bus.read(0x1800));
    }

    #[test]
    fn test_cpu_bus_write() {
        let cpu = CPU::new();
        let mapper = MockPRGRomMapper::new();
        let mut main_bus = MainBus::new(cpu, Rc::new(mapper));
        
        main_bus.write(0x1, 0x34);
        assert_eq!(0x34, main_bus.ram[0x1]);

        main_bus.write(0x801, 0x35);
        assert_eq!(0x35, main_bus.ram[0x1]);

        main_bus.write(0x1001, 0x36);
        assert_eq!(0x36, main_bus.ram[0x1]);

        main_bus.write(0x1001, 0x37);
        assert_eq!(0x37, main_bus.ram[0x1]);
    }
}
