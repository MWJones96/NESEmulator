/*
    CLC - Clear Carry Flag
    Operation: 0 → C

    This instruction initializes the carry flag to a 0.
    This op­ eration should normally precede an ADC loop.
    It is also useful when used with a R0L instruction to
    clear a bit in memory.

    This instruction affects no registers in the microprocessor
    and no flags other than the carry flag which is reset.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn clcc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn clc(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.c = false;
    }
}

#[cfg(test)]
mod clc_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_clc_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        assert_eq!(2, cpu.clcc(&cpu._imp()));
    }

    #[test]
    fn test_clc_carry_flag() {
        let mut cpu = NESCPU::new();
        cpu.c = true;

        cpu.clc(&cpu._imp(), &mut MockBus::new());
        assert_eq!(false, cpu.c);

        cpu.clc(&cpu._imp(), &mut MockBus::new());
        assert_eq!(false, cpu.c);
    }
}
