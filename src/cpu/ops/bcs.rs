/*
   BCS - Branch on Carry Set
   Operation: Branch on C = 1

   This instruction takes the conditional branch if the carry flag is on.

   BCS does not affect any of the flags or registers except for the program
   counter and only then if the carry flag is on.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn bcsc(&self, mode: &AddrModeResult) -> u8 {
        if self.c {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    pub(in crate::cpu) fn bcs(&mut self, mode: &AddrModeResult, _bus: &mut dyn Bus) {
        if self.c {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bcs_tests {

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_bcs_no_branch_no_page_cross() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.bcsc(&cpu._rel(0x1)));
    }

    #[test]
    fn test_bcs_no_branch_with_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x1234;

        assert_eq!(2, cpu.bcsc(&cpu._rel(0xaa)));
    }

    #[test]
    fn test_bcs_with_branch_no_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.c = true;

        assert_eq!(3, cpu.bcsc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bcs_with_branch_and_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.c = true;
        cpu.pc = 0x12ff;

        assert_eq!(4, cpu.bcsc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_bcs_pc_no_branch_no_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x1234;
        cpu.c = false;
        cpu.bcs(&cpu._rel(0x1), &mut MockBus::new());
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bcs_pc_no_branch_with_page_cross() {
        let mut cpu = NESCPU::new();

        cpu.pc = 0x12ff;
        cpu.c = false;
        cpu.bcs(&cpu._rel(0xa), &mut MockBus::new());
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bcs_pc_with_branch_no_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x81;
        cpu.c = true;

        cpu.bcs(&cpu._rel(0x80), &mut MockBus::new());
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bcs_pc_with_branch_and_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.pc = 0x8081;
        cpu.c = true;

        cpu.bcs(&cpu._rel(0x7f), &mut MockBus::new());
        assert_eq!(0x8100, cpu.pc);
    }
}
