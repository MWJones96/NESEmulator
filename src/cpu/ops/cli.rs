/*
    CLI - Clear Interrupt Disable
    Operation: 0 → I

    This instruction initializes the interrupt disable to a 0.
    This allows the microprocessor to receive interrupts.

    It affects no registers in the microprocessor and no flags
    other than the interrupt disable which is cleared.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn clic(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn cli(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.i = false;
    }
}

#[cfg(test)]
mod cli_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_cli_correct_number_of_cycles() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.clic(&cpu._imp()));
    }

    #[test]
    fn test_cli_carry_flag() {
        let mut cpu = CPU::new();
        cpu.i = true;

        cpu.cli(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(false, cpu.i);

        cpu.cli(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(false, cpu.i);
    }
}
