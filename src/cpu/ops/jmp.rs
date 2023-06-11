/*
    JMP - JMP Indirect
    Operation: [PC + 1] → PCL, [PC + 2] → PCH

    This instruction establishes a new value for the program counter.

    It affects only the program counter in the microprocessor and
    affects no flags in the status register.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn jmpc(&self, mode: &AddrModeResult) -> u8 {
        1 + mode.cycles
    }

    pub(in crate::cpu) fn jmp(&mut self, mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.pc = mode.addr.unwrap();
    }
}

#[cfg(test)]
mod jmp_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_jmp_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.jmpc(&cpu._abs(0x0000, &bus)));
    }

    #[test]
    fn test_jmp_ind_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.jmpc(&cpu._ind(0x0000, &bus)));
    }

    #[test]
    fn test_jmp_goes_to_correct_pc() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        cpu.jmp(&cpu._abs(0x1234, &bus), &mut MockBus::new());
        assert_eq!(0x1234, cpu.pc);
    }
}
