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

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn tayc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn tay(&mut self, _mode: &AddrModeResult) {
        self.y = self.a;

        self.n = (self.y & 0x80) > 0;
        self.z = self.y == 0;
    }
}

#[cfg(test)]
mod tay_tests {
    use super::*;

    #[test]
    fn test_tay_returns_correct_number_ofc() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.tayc(&cpu._imp()));
    }

    #[test]
    fn test_tay() {
        let mut cpu = CPU::new();
        cpu.a = 0xcc;

        cpu.tay(&cpu._imp());
        assert_eq!(0xcc, cpu.y);
        assert_eq!(0xcc, cpu.a);
    }

    #[test]
    fn test_tay_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0x80;

        cpu.tay(&cpu._imp());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tay_zero_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0xff;

        cpu.tay(&cpu._imp());
        assert_eq!(true, cpu.z);
    }
}
