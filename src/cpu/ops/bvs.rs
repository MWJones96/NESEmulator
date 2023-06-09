/*
    BVS - Branch on Overflow Set
    Operation: Branch on V = 1

    This instruction tests the V flag and takes the conditional
    branch if V is on.

    BVS does not affect any flags or registers other than the
    program, counter and only when the overflow flag is set.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn bvsc(&self, mode: &AddrModeResult) -> u8 {
        if self.v {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    pub(in crate::cpu) fn bvs(&mut self, mode: &AddrModeResult, _bus: &mut dyn Bus) {
        if self.v {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bvs_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_bvs_no_branch_no_page_cross() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.bvsc(&cpu._rel(0x1)));
    }

    #[test]
    fn test_bvs_no_branch_with_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x1234;

        assert_eq!(2, cpu.bvsc(&cpu._rel(0xaa)));
    }

    #[test]
    fn test_bvs_with_branch_no_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.v = true;

        assert_eq!(3, cpu.bvsc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bvs_with_branch_and_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.v = true;
        cpu.pc = 0x12ff;

        assert_eq!(4, cpu.bvsc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bvs_pc_no_branch_no_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x1234;
        cpu.v = false;
        cpu.bvs(&cpu._rel(0x1), &mut MockBus::new());
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bvs_pc_no_branch_with_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x12ff;
        cpu.v = false;
        cpu.bvs(&cpu._rel(0xa), &mut MockBus::new());
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bvs_pc_with_branch_no_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x81;
        cpu.v = true;

        cpu.bvs(&cpu._rel(0x80), &mut MockBus::new());
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bvs_pc_with_branch_and_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x8081;
        cpu.v = true;

        cpu.bvs(&cpu._rel(0x7f), &mut MockBus::new());
        assert_eq!(0x8100, cpu.pc);
    }
}
