/*
    CLC - Clear Carry Flag
    Operation: 0 → C

    This instruction initializes the carry flag to a 0.
    This op­ eration should normally precede an ADC loop.
    It is also useful when used with a R0L instruction to
    clear a bit in memory.

    This instruction affects no registers in the microprocessor
    and no flags other than the carry flag which is reset.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn clc_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn clc(&mut self, _mode: &AddrModeResult) {
        self.c = false;
    }
}

#[cfg(test)]
mod clc_tests {
    use super::*;

    #[test]
    fn test_clc_correct_number_of_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(2, cpu.clc_cycles(&cpu.imp()));
    }

    #[test]
    fn test_clc_carry_flag() {
        let mut cpu = CPU::new();
        cpu.c = true;

        cpu.clc(&cpu.imp());
        assert_eq!(false, cpu.c);

        cpu.clc(&cpu.imp());
        assert_eq!(false, cpu.c);
    }
}
