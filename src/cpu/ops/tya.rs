use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn tyac(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn tya(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.a = self.y;

        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod tya_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_tya_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.tyac(&cpu._imp()));
    }

    #[test]
    fn test_tya() {
        let mut cpu = NESCPU::new();
        cpu.y = 0xcc;

        cpu.tya(&cpu._imp(), &mut MockBus::new());
        assert_eq!(0xcc, cpu.a);
        assert_eq!(0xcc, cpu.y);
    }

    #[test]
    fn test_tya_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.y = 0x80;

        cpu.tya(&cpu._imp(), &mut MockBus::new());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tya_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;

        cpu.tya(&cpu._imp(), &mut MockBus::new());
        assert_eq!(true, cpu.z);
    }
}
