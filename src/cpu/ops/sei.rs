/*
    SEI - Set Interrupt Disable
    Operation: 1 → I

    This instruction initializes the interrupt disable to a 1.
    It is used to mask interrupt requests during system reset
    operations and during interrupt commands.

    It affects no registers in the microprocessor and no flags
    other than the interrupt disable which is set.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn seic(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn sei(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.i = true;
    }
}

#[cfg(test)]
mod sei_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_sei_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        assert_eq!(2, cpu.seic(&cpu._imp()));
    }

    #[test]
    fn test_sei_carry_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;

        cpu.sei(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(true, cpu.i);

        cpu.sei(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(true, cpu.i);
    }
}
