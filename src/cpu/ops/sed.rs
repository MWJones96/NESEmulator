/*
    SED - Set Decimal Mode
    Operation: 1 → D

    This instruction sets the decimal mode flag D to a 1. This makes
    all subsequent ADC and SBC instructions operate as a decimal
    arithmetic operation.

    SED affects no registers in the microprocessor and no flags other
    than the decimal mode which is set to a 1.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn sedc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn sed(&mut self, _mode: &AddrModeResult) {
        self.d = true;
    }
}

#[cfg(test)]
mod sed_tests {
    use super::*;

    #[test]
    fn test_sed_correct_number_ofc() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.sedc(&cpu._imp()));
    }

    #[test]
    fn test_sed_carry_flag() {
        let mut cpu = CPU::new();
        cpu.d = false;

        cpu.sed(&cpu._imp());
        assert_eq!(true, cpu.d);

        cpu.sed(&cpu._imp());
        assert_eq!(true, cpu.d);
    }
}
