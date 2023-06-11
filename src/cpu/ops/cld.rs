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

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn cldc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn cld(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.d = false;
    }
}

#[cfg(test)]
mod cld_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_cld_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        assert_eq!(2, cpu.cldc(&cpu._imp()));
    }

    #[test]
    fn test_cld_carry_flag() {
        let mut cpu = NESCPU::new();
        cpu.d = true;

        cpu.cld(&cpu._imp(), &mut MockBus::new());
        assert_eq!(false, cpu.d);

        cpu.cld(&cpu._imp(), &mut MockBus::new());
        assert_eq!(false, cpu.d);
    }
}
