/*
    BCC - Branch on Carry Clear
    Operation: Branch on C = 0

    This instruction tests the state of the carry bit and takes
    a conditional branch if the carry bit is reset.

    It affects no flags or registers other than the program counter
    and then only if the C flag is not on.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn bcc_cycles(&self, mode: &AddrModeResult) -> u8 {
        if !self.c {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    #[inline]
    pub(in crate::cpu) fn bcc(&mut self, mode: &AddrModeResult) {
        if !self.c {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bcc_tests {
    use super::*;

    #[test]
    fn test_bcc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.c = true;
        assert_eq!(2, cpu.bcc_cycles(&cpu._rel(0x1)));
    }

    #[test]
    fn test_bcc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.c = true;
        assert_eq!(2, cpu.bcc_cycles(&cpu._rel(0xa)));
    }

    #[test]
    fn test_bcc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.c = false;

        assert_eq!(3, cpu.bcc_cycles(&cpu._rel(0x80)));
    }

    #[test]
    fn test_bcc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.c = false;

        assert_eq!(4, cpu.bcc_cycles(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bcc_pc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.c = true;
        cpu.bcc(&cpu._rel(0x1));
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bcc_pc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.c = true;
        cpu.bcc(&cpu._rel(0xa));
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bcc_pc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.c = false;

        cpu.bcc(&cpu._rel(0x80));
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bcc_pc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.c = false;

        cpu.bcc(&cpu._rel(0x7f));
        assert_eq!(0x8100, cpu.pc);
    }
}
