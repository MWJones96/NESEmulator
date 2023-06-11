/*
    BPL - Branch on Result Plus
    Operation: Branch on N = 0

    This instruction is the complementary branch to branch
    on result minus. It is a conditional branch which takes
    the branch when the N bit is reset (0).

    BPL is used to test if the previous result bit 7 was off (0)
    and branch on result minus is used to determine if the
    previous result was minus or bit 7 was on (1).

    The instruction affects no flags or other registers other
    than the P counter and only affects the P counter when
    the N bit is reset.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn bplc(&self, mode: &AddrModeResult) -> u8 {
        if !self.n {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    pub(in crate::cpu) fn bpl(&mut self, mode: &AddrModeResult, _bus: &mut dyn Bus) {
        if !self.n {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bpl_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_bpl_no_branch_no_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x1234;
        cpu.n = true;
        assert_eq!(2, cpu.bplc(&cpu._rel(0x1)));
    }

    #[test]
    fn test_bpl_no_branch_with_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x12ff;
        cpu.n = true;
        assert_eq!(2, cpu.bplc(&cpu._rel(0xa)));
    }

    #[test]
    fn test_bpl_with_branch_no_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x81;
        cpu.n = false;

        assert_eq!(3, cpu.bplc(&cpu._rel(0x80)));
    }

    #[test]
    fn test_bpl_with_branch_and_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x8081;
        cpu.n = false;

        assert_eq!(4, cpu.bplc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bpl_pc_no_branch_no_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x1234;
        cpu.n = true;
        cpu.bpl(&cpu._rel(0x1), &mut MockBus::new());
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bpl_pc_no_branch_with_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x12ff;
        cpu.n = true;
        cpu.bpl(&cpu._rel(0xa), &mut MockBus::new());
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bpl_pc_with_branch_no_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x81;
        cpu.n = false;

        cpu.bpl(&cpu._rel(0x80), &mut MockBus::new());
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bpl_pc_with_branch_and_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x8081;
        cpu.n = false;

        cpu.bpl(&cpu._rel(0x7f), &mut MockBus::new());
        assert_eq!(0x8100, cpu.pc);
    }
}
