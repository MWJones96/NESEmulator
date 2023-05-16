use crate::{
    cpu::{bus::CPUBus, CPU},
    mapper::PRGRomMapper,
};

pub struct MainBus<'a> {
    mapper: &'a dyn PRGRomMapper,
    ram: [u8; 0x800],
}

impl<'a> MainBus<'a> {
    pub fn new(mapper: &'a impl PRGRomMapper) -> Self {
        Self {
            mapper,
            ram: [0; 0x800],
        }
    }

    pub fn clock(&mut self, cpu: &mut CPU) {
        cpu.clock(self);
    }
}

impl CPUBus for MainBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.ram[(addr & 0x7ff) as usize],
            0x8000..=0xffff => self.mapper.read(addr),
            _ => 0x0, //Open Bus Read
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1fff => {
                self.ram[(addr & 0x7ff) as usize] = data;
            }
            0x8000..=0xffff => {}
            _ => {} //Open Bus Write
        }
    }
}

#[cfg(test)]
mod main_bus_tests {
    use crate::mapper::MockPRGRomMapper;

    use super::*;

    #[test]
    fn test_cpu_bus_read() {
        let mapper = MockPRGRomMapper::new();
        let mut main_bus = MainBus::new(&mapper);

        main_bus.ram[0x0] = 0xff;

        assert_eq!(0xff, main_bus.read(0x0));
        assert_eq!(0xff, main_bus.read(0x800));
        assert_eq!(0xff, main_bus.read(0x1000));
        assert_eq!(0xff, main_bus.read(0x1800));
    }

    #[test]
    fn test_cpu_bus_write() {
        let mapper = MockPRGRomMapper::new();
        let mut main_bus = MainBus::new(&mapper);

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
