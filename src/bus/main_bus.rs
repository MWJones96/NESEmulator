use crate::cpu::bus::CPUBus;

pub struct MainBus {
    mem: [u8; 0x10000],
}

impl MainBus {
    pub fn new() -> Self {
        Self { mem: [0; 0x10000] }
    }
}

impl CPUBus for MainBus {
    fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}

#[cfg(test)]
mod main_bus_tests {
    use super::*;

    #[test]
    fn test_cpu_bus_read() {
        let mut main_bus = MainBus::new();
        main_bus.mem[0x1234] = 0xee;
        assert_eq!(0xee, main_bus.read(0x1234));
    }

    #[test]
    fn test_cpu_bus_write() {
        let mut main_bus = MainBus::new();
        main_bus.write(0xeeee, 0x34);
        assert_eq!(0x34, main_bus.mem[0xeeee]);
    }
}
