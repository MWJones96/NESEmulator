/*
    CLV - Clear Overflow Flag
    Operation: 0 → V

    This instruction clears the overflow flag to a 0.
    This com­mand is used in conjunction with the set
    overflow pin which can change the state of the
    overflow flag with an external signal.

    CLV affects no registers in the microprocessor and
    no flags other than the overflow flag which is set
    to a 0.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn clvc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn clv(&mut self, _mode: &AddrModeResult) {
        self.v = false;
    }
}

#[cfg(test)]
mod clv_tests {
    use super::*;

    #[test]
    fn test_clv_correct_number_ofc() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.clvc(&cpu._imp()));
    }

    #[test]
    fn test_clv_carry_flag() {
        let mut cpu = CPU::new();
        cpu.v = true;

        cpu.clv(&cpu._imp());
        assert_eq!(false, cpu.v);

        cpu.clv(&cpu._imp());
        assert_eq!(false, cpu.v);
    }
}
