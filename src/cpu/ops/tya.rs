use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn tyac(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn tya(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.a = self.y;

        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod tya_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_tya_returns_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.tyac(&cpu._imp()));
    }

    #[test]
    fn test_tya() {
        let mut cpu = CPU::new();
        cpu.y = 0xcc;

        cpu.tya(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(0xcc, cpu.a);
        assert_eq!(0xcc, cpu.y);
    }

    #[test]
    fn test_tya_negative_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0x80;

        cpu.tya(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tya_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.tya(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(true, cpu.z);
    }
}
