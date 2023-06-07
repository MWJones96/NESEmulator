/*
    BMI - Branch on Result Minus
    Operation: Branch on N = 1

    This instruction takes the conditional branch if the N bit is set.

    BMI does not affect any of the flags or any other part of the machine
    other than the program counter and then only if the N bit is on.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn bmi_cycles(&self, mode: &AddrModeResult) -> u8 {
        if self.n {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    #[inline]
    pub(in crate::cpu) fn bmi(&mut self, mode: &AddrModeResult) {
        if self.n {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod bmi_tests {
    use super::*;

    #[test]
    fn test_bmi_no_branch_no_page_cross() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.bmi_cycles(&cpu.rel(0x1)));
    }

    #[test]
    fn test_bmi_no_branch_with_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x1234;

        assert_eq!(2, cpu.bmi_cycles(&cpu.rel(0xaa)));
    }

    #[test]
    fn test_bmi_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.n = true;

        assert_eq!(3, cpu.bmi_cycles(&cpu.rel(0x7f)));
    }

    #[test]
    fn test_bmi_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.n = true;
        cpu.pc = 0x12ff;

        assert_eq!(4, cpu.bmi_cycles(&cpu.rel(0x7f)));
    }

    #[test]
    fn test_bmi_pc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.n = false;
        cpu.bmi(&cpu.rel(0x1));
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_bmi_pc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.n = false;
        cpu.bmi(&cpu.rel(0xa));
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_bmi_pc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.n = true;

        cpu.bmi(&cpu.rel(0x80));
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_bmi_pc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.n = true;

        cpu.bmi(&cpu.rel(0x7f));
        assert_eq!(0x8100, cpu.pc);
    }
}
