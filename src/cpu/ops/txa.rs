/*
    TXA - Transfer Index X To Accumulator
    Operation: X → A

    This instruction moves the value that is in the index register
    X to the accumulator A without disturbing the content of the
    index register X.

    TXA does not affect any register other than the accumula­tor and
    does not affect the carry or overflow flag. If the result in A
    has bit 7 on, then the N flag is set, otherwise it is reset. If
    the resultant value in the accumulator is 0, then the Z flag is
    set, other­ wise it is reset.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn txac(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn txa(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.a = self.x;

        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod txa_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_txa_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.txac(&cpu._imp()));
    }

    #[test]
    fn test_txa() {
        let mut cpu = NESCPU::new();
        cpu.x = 0xcc;

        cpu.txa(&cpu._imp(), &mut MockBus::new());
        assert_eq!(0xcc, cpu.a);
        assert_eq!(0xcc, cpu.x);
    }

    #[test]
    fn test_txa_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.x = 0x80;

        cpu.txa(&cpu._imp(), &mut MockBus::new());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_txa_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;

        cpu.txa(&cpu._imp(), &mut MockBus::new());
        assert_eq!(true, cpu.z);
    }
}
