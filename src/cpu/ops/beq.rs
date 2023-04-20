/*
    BEQ - Branch on Result Zero
    Operation: Branch on Z = 1

    This instruction could also be called "Branch on Equal."

    It takes a conditional branch whenever the Z flag is on or
    the previ­ous result is equal to 0.

    BEQ does not affect any of the flags or registers other than
    the program counter and only then when the Z flag is set.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn beq(&mut self, mode: &AddrModeResult) -> u8 {
        self.branch_helper(self.z, mode)
    }
}

#[cfg(test)]
mod beq_tests {
    use super::*;

    #[test]
    fn test_beq_no_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x1234;
        cpu.z = false;

        assert_eq!(2, cpu.beq(&cpu.rel(0x1)));
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_beq_no_branch_with_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x12ff;
        cpu.z = false;

        assert_eq!(3, cpu.beq(&cpu.rel(0xa)));
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_beq_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.z = true;

        assert_eq!(3, cpu.beq(&cpu.rel(0x80)));
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_beq_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.z = true;

        assert_eq!(4, cpu.beq(&cpu.rel(0x7f)));
        assert_eq!(0x8100, cpu.pc);
    }
}
