/*
    NOP - No Operation
    Operation: No operation
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn nop_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn nop(&self, _mode: &AddrModeResult) {
        //No operation
    }
}

#[cfg(test)]
mod nop_tests {
    use super::*;

    #[test]
    fn test_nop_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.nop_cycles(&cpu.imp()));
    }

    #[test]
    fn test_nop_does_not_crash() {
        let cpu = CPU::new();
        cpu.nop(&cpu.imp());
    }
}
