/*
    JMP - JMP Indirect
    Operation: [PC + 1] → PCL, [PC + 2] → PCH

    This instruction establishes a new value for the program counter.

    It affects only the program counter in the microprocessor and
    affects no flags in the status register.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn jmp_cycles(&self, mode: &AddrModeResult) -> u8 {
        1 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn jmp(&mut self, mode: &AddrModeResult) {
        self.pc = mode.addr.unwrap();
    }
}

#[cfg(test)]
mod jmp_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_jmp_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.jmp_cycles(&cpu._abs(0x0000, &bus)));
    }

    #[test]
    fn test_jmp_ind_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.jmp_cycles(&cpu._ind(0x0000, &bus)));
    }

    #[test]
    fn test_jmp_goes_to_correct_pc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        cpu.jmp(&cpu._abs(0x1234, &bus));
        assert_eq!(0x1234, cpu.pc);
    }
}
