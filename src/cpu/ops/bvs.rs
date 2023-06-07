/*
    BVS - Branch on Overflow Set
    Operation: Branch on V = 1

    This instruction tests the V flag and takes the conditional
    branch if V is on.

    BVS does not affect any flags or registers other than the
    program, counter and only when the overflow flag is set.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn bvs_cycles(&self, mode: &AddrModeResult) -> u8 {
        if self.v {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    #[inline]
    pub(in crate::cpu) fn bvs(&mut self, mode: &AddrModeResult) {
        if self.v {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bvs_tests {
    use super::*;

    #[test]
    fn test_bvs_no_branch_no_page_cross() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.bvs_cycles(&cpu.rel(0x1)));
    }

    #[test]
    fn test_bvs_no_branch_with_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x1234;

        assert_eq!(2, cpu.bvs_cycles(&cpu.rel(0xaa)));
    }

    #[test]
    fn test_bvs_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.v = true;

        assert_eq!(3, cpu.bvs_cycles(&cpu.rel(0x7f)));
    }

    #[test]
    fn test_bvs_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.v = true;
        cpu.pc = 0x12ff;

        assert_eq!(4, cpu.bvs_cycles(&cpu.rel(0x7f)));
    }

    #[test]
    fn test_bvs_pc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.v = false;
        cpu.bvs(&cpu.rel(0x1));
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bvs_pc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.v = false;
        cpu.bvs(&cpu.rel(0xa));
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bvs_pc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.v = true;

        cpu.bvs(&cpu.rel(0x80));
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bvs_pc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.v = true;

        cpu.bvs(&cpu.rel(0x7f));
        assert_eq!(0x8100, cpu.pc);
    }
}
