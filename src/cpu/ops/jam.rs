/*
    JAM - Halt the CPU
    Operation: Stop execution

    This undocumented instruction stops execution. The microprocessor
    will not fetch further instructions, and will neither handle IRQs
    nor NMIs. It will handle a RESET though.
*/

use crate::cpu::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn jam_cycles(&self) -> u8 {
        0
    }
}

#[cfg(test)]
mod jam_tests {
    use super::*;

    #[test]
    fn test_jam_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(0, cpu.jam_cycles());
    }
}
