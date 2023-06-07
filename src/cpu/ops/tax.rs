/*
    TAX - Transfer Accumulator To Index X
    Operation: A → X

    This instruction takes the value from accumulator A and trans­fers
    or loads it into the index register X without disturbing the content
    of the accumulator A.

    TAX only affects the index register X, does not affect the carry or
    overflow flags. The N flag is set if the resultant value in the index
    register X has bit 7 on, otherwise N is reset. The Z bit is set if
    the content of the register X is 0 as a result of the opera­tion,
    otherwise it is reset.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn tax_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn tax(&mut self, _mode: &AddrModeResult) {
        self.x = self.a;

        self.n = (self.x & 0x80) > 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod tax_tests {
    use super::*;

    #[test]
    fn test_tax_returns_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.tax_cycles(&cpu._imp()));
    }

    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.a = 0xcc;

        cpu.tax(&cpu._imp());
        assert_eq!(0xcc, cpu.x);
        assert_eq!(0xcc, cpu.a);
    }

    #[test]
    fn test_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0x80;

        cpu.tax(&cpu._imp());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0xff;

        cpu.tax(&cpu._imp());
        assert_eq!(true, cpu.z);
    }
}
