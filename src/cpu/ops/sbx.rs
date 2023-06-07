/*
    SBX - Subtract Memory from Accumulator "AND" Index Register X
    Operation: (A ∧ X) - M → X

    This undocumented instruction performs a bit-by-bit "AND" of the
    value of the accumulator and the index register X and subtracts
    the value of memory from this result, using two's complement
    arithmetic, and stores the result in the index register X.

    This instruction affects the index register X. The carry flag is
    set if the result is greater than or equal to 0. The carry flag
    is reset when the result is less than 0, indicating a borrow.
    The negative flag is set if the result in index register X has
    bit 7 on, otherwise it is reset. The Z flag is set if the result
    in index register X is 0, otherwise it is reset. The over­flow flag
    not affected at all.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn sbx_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn sbx(&mut self, mode: &AddrModeResult) {
        self.x = (self.a & self.x).wrapping_sub(mode.data.unwrap());

        self.n = (self.x & 0x80) != 0;
        self.z = self.x == 0;
        self.c = self.x < 0x80;
    }
}

#[cfg(test)]
mod sbx_tests {
    use super::*;

    #[test]
    fn test_sbx_imm_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.sbx_cycles(&cpu._imm(0x0)))
    }

    #[test]
    fn test_sbx() {
        let mut cpu = CPU::new();
        cpu.a = 0b1010_1010;
        cpu.x = 0b0101_0101;

        cpu.sbx(&cpu._imm(0x1));
        assert_eq!(0xff, cpu.x);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_sbx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0b1010_1010;
        cpu.x = 0b0101_0101;

        cpu.sbx(&cpu._imm(0x0));
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_sbx_carry_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1010;
        cpu.x = 0b0000_1111;

        cpu.sbx(&cpu._imm(0b0000_1010));
        assert_eq!(true, cpu.c);

        cpu.sbx(&cpu._imm(0x1));
        assert_eq!(false, cpu.c);
    }
}
