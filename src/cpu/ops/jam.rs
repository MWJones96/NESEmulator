/*
    JAM - Halt the CPU
    Operation: Stop execution

    This undocumented instruction stops execution. The microprocessor
    will not fetch further instructions, and will neither handle IRQs
    nor NMIs. It will handle a RESET though.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn jamc(&self, _mode: &AddrModeResult) -> u8 {
        0
    }

    #[inline]
    pub(in crate::cpu) fn _jam(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        panic!("JAM: This should not be called")
    }
}

#[cfg(test)]
mod jam_tests {
    use super::*;

    #[test]
    fn test_jam_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(0, cpu.jamc(&cpu._imp()));
    }
}
