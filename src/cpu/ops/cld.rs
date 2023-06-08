/*
    CLD - Clear Decimal Mode
    Operation: 0 → D

    This instruction sets the decimal mode flag to a 0.
    This all subsequent ADC and SBC instructions to operate
    as simple operations.

    CLD affects no registers in the microprocessor and no
    flags other than the decimal mode flag which is set
    to a 0.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn cldc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn cld(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.d = false;
    }
}

#[cfg(test)]
mod cld_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_cld_correct_number_of_cycles() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.cldc(&cpu._imp()));
    }

    #[test]
    fn test_cld_carry_flag() {
        let mut cpu = CPU::new();
        cpu.d = true;

        cpu.cld(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(false, cpu.d);

        cpu.cld(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(false, cpu.d);
    }
}
