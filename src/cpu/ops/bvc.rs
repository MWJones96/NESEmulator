/*
    BVC - Branch on Overflow Clear
    Operation: Branch on V = 0

    This instruction tests the status of the V flag and takes
    the conditional branch if the flag is not set.

    BVC does not affect any of the flags and registers other
    than the program counter and only when the overflow
    flag is reset.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn bvc(&mut self, mode: &AddrModeResult) -> u8 {
        self.branch_helper(!self.v, mode)
    }
}

#[cfg(test)]
mod bvc_tests {
    use super::*;

    #[test]
    fn test_bvc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.v = true;
        assert_eq!(2, cpu.bvc(&cpu.rel(0x1)));
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bvc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.v = true;
        assert_eq!(3, cpu.bvc(&cpu.rel(0xa)));
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bvc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.v = false;

        assert_eq!(3, cpu.bvc(&cpu.rel(0x80)));
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bvc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.v = false;

        assert_eq!(4, cpu.bvc(&cpu.rel(0x7f)));
        assert_eq!(0x8100, cpu.pc);
    }
}
