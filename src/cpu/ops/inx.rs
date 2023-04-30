/*
    INX - Increment Index Register X By One
    Operation: X + 1 → X

    Increment X adds 1 to the current value of the X register.
    This is an 8-bit increment which does not affect the carry
    operation, therefore, if the value of X before the increment
    was FF, the resulting value is 00.

    INX does not affect the carry or overflow flags; it sets the
    N flag if the result of the increment has a one in bit 7,
    otherwise resets N; sets the Z flag if the result of the
    increment is 0, otherwise it resets the Z flag.

    INX does not affect any other register other than the X register.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    pub(in crate::cpu) fn inx_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn inx(&mut self, _mode: &AddrModeResult) {
        self.x = self.x.wrapping_add(1);

        self.n = (self.x & 0x80) > 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod inx_tests {
    use super::*;

    #[test]
    fn test_inx_returns_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        assert_eq!(2, cpu.inx_cycles(&cpu.imp()));
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.x = 0x80;

        cpu.inx(&cpu.imp());

        assert_eq!(0x81, cpu.x);
    }

    #[test]
    fn test_inx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x7f;

        cpu.inx(&cpu.imp());

        assert_eq!(0x80, cpu.x);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_inx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0xff;

        cpu.inx(&cpu.imp());

        assert_eq!(0x0, cpu.x);
        assert_eq!(true, cpu.z);
    }
}
