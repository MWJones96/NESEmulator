/*
    INY - Increment Index Register Y By One
    Operation: Y + 1 → Y

    Increment Y increments or adds one to the current
    value in the Y register, storing the result in the
    Y register. As in the case of INX the primary
    application is to step thru a set of values using
    the Y register.

    The INY does not affect the carry or overflow flags,
    sets the N flag if the result of the increment has a
    one in bit 7, otherwise resets N, sets Z if as a
    result of the increment the Y register is zero
    otherwise resets the Z flag.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    pub(in crate::cpu) fn iny_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn iny(&mut self, _mode: &AddrModeResult) {
        self.y = self.y.wrapping_add(1);

        self.n = (self.y & 0x80) > 0;
        self.z = self.y == 0;
    }
}

#[cfg(test)]
mod iny_tests {
    use super::*;

    #[test]
    fn test_iny_returns_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.iny_cycles(&cpu.imp()));
    }

    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.y = 0x80;

        cpu.iny(&cpu.imp());

        assert_eq!(0x81, cpu.y);
    }

    #[test]
    fn test_iny_negative_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0x7f;

        cpu.iny(&cpu.imp());

        assert_eq!(0x80, cpu.y);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_iny_zero_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0xff;

        cpu.iny(&cpu.imp());

        assert_eq!(0x0, cpu.y);
        assert_eq!(true, cpu.z);
    }
}
