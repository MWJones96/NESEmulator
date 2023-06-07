/*
    BNE - Branch on Result Not Zero
    Operation: Branch on Z = 0

    This instruction could also be called "Branch on Not Equal."
    It tests the Z flag and takes the conditional branch if the
    Z flag is not on, indicating that the previous result was not zero.

    BNE does not affect any of the flags or registers other than the
    program counter and only then if the Z flag is reset.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn bnec(&self, mode: &AddrModeResult) -> u8 {
        if !self.z {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    #[inline]
    pub(in crate::cpu) fn bne(&mut self, mode: &AddrModeResult) {
        if !self.z {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bne_tests {
    use super::*;

    #[test]
    fn test_bne_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.z = true;
        assert_eq!(2, cpu.bnec(&cpu._rel(0x1)));
    }

    #[test]
    fn test_bne_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.z = true;
        assert_eq!(2, cpu.bnec(&cpu._rel(0xa)));
    }

    #[test]
    fn test_bne_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.z = false;

        assert_eq!(3, cpu.bnec(&cpu._rel(0x80)));
    }

    #[test]
    fn test_bne_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.z = false;

        assert_eq!(4, cpu.bnec(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bne_pc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.z = true;
        cpu.bne(&cpu._rel(0x1));
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bne_pc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.z = true;
        cpu.bne(&cpu._rel(0xa));
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bne_pc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.z = false;

        cpu.bne(&cpu._rel(0x80));
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bne_pc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.z = false;

        cpu.bne(&cpu._rel(0x7f));
        assert_eq!(0x8100, cpu.pc);
    }
}
