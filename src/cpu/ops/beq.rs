/*
    BEQ - Branch on Result Zero
    Operation: Branch on Z = 1

    This instruction could also be called "Branch on Equal."

    It takes a conditional branch whenever the Z flag is on or
    the previ­ous result is equal to 0.

    BEQ does not affect any of the flags or registers other than
    the program counter and only then when the Z flag is set.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn beqc(&self, mode: &AddrModeResult) -> u8 {
        if self.z {
            2 + 1 + mode.cycles
        } else {
            2
        }
    }

    pub(in crate::cpu) fn beq(&mut self, mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        if self.z {
            self.pc = mode.addr.unwrap();
        }
    }
}

#[cfg(test)]
mod beq_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_beq_no_branch_no_page_cross() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.beqc(&cpu._rel(0x1)));
    }

    #[test]
    fn test_beq_no_branch_with_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x1234;

        assert_eq!(2, cpu.beqc(&cpu._rel(0xaa)));
    }

    #[test]
    fn test_beq_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.z = true;

        assert_eq!(3, cpu.beqc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_beq_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.z = true;
        cpu.pc = 0x12ff;

        assert_eq!(4, cpu.beqc(&cpu._rel(0x7f)));
    }

    #[test]
    fn test_beq_pc_no_branch_no_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        cpu.z = false;
        cpu.beq(&cpu._rel(0x1), &mut MockCPUBus::new());
        assert_eq!(0x1234, cpu.pc);
    }

    #[test]
    fn test_beq_pc_no_branch_with_page_cross() {
        let mut cpu = CPU::new();

        cpu.pc = 0x12ff;
        cpu.z = false;
        cpu.beq(&cpu._rel(0xa), &mut MockCPUBus::new());
        assert_eq!(0x12ff, cpu.pc);
    }

    #[test]
    fn test_beq_pc_with_branch_no_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x81;
        cpu.z = true;

        cpu.beq(&cpu._rel(0x80), &mut MockCPUBus::new());
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_beq_pc_with_branch_and_page_cross() {
        let mut cpu = CPU::new();
        cpu.pc = 0x8081;
        cpu.z = true;

        cpu.beq(&cpu._rel(0x7f), &mut MockCPUBus::new());
        assert_eq!(0x8100, cpu.pc);
    }
}
