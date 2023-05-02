/*
    SEC - Set Carry Flag
    Operation: 1 → C

    This instruction initializes the carry flag to a 1. This operation should
    normally precede a SBC loop. It is also useful when used with a ROL
    instruction to initialize a bit in memory to a 1.

    This instruction affects no registers in the microprocessor and no flags
    other than the carry flag which is set.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn sec_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn sec(&mut self, _mode: &AddrModeResult) {
        self.c = true;
    }
}

#[cfg(test)]
mod sec_tests {
    use super::*;

    #[test]
    fn test_sec_correct_number_of_cycles() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.sec_cycles(&cpu.imp()));
    }

    #[test]
    fn test_sec_carry_flag() {
        let mut cpu = CPU::new();
        cpu.c = false;

        cpu.sec(&cpu.imp());
        assert_eq!(true, cpu.c);

        cpu.sec(&cpu.imp());
        assert_eq!(true, cpu.c);
    }
}
