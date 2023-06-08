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

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn clcc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn clc(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.c = false;
    }
}

#[cfg(test)]
mod clc_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_clc_correct_number_of_cycles() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.clcc(&cpu._imp()));
    }

    #[test]
    fn test_clc_carry_flag() {
        let mut cpu = CPU::new();
        cpu.c = true;

        cpu.clc(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(false, cpu.c);

        cpu.clc(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(false, cpu.c);
    }
}
