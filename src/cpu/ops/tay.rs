/*
    TAY - Transfer Accumula Tor To Index Y
    Operation: A → Y

    This instruction moves the value of the accumulator into index
    register Y without affecting the accumulator.

    TAY instruction only affects the Y register and does not affect
    either the carry or overflow flags. If the index register Y has
    bit 7 on, then N is set, otherwise it is reset. If the content
    of the index register Y equals 0 as a result of the operation,
    Z is set on, otherwise it is reset.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn tayc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn tay(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.y = self.a;

        self.n = (self.y & 0x80) > 0;
        self.z = self.y == 0;
    }
}

#[cfg(test)]
mod tay_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_tay_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.tayc(&cpu._imp()));
    }

    #[test]
    fn test_tay() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xcc;

        cpu.tay(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(0xcc, cpu.y);
        assert_eq!(0xcc, cpu.a);
    }

    #[test]
    fn test_tay_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0x80;

        cpu.tay(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tay_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.y = 0xff;

        cpu.tay(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(true, cpu.z);
    }
}
