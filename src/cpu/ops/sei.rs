/*
    SEI - Set Interrupt Disable
    Operation: 1 → I

    This instruction initializes the interrupt disable to a 1.
    It is used to mask interrupt requests during system reset
    operations and during interrupt commands.

    It affects no registers in the microprocessor and no flags
    other than the interrupt disable which is set.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn sei_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn sei(&mut self, _mode: &AddrModeResult) {
        self.i = true;
    }
}

#[cfg(test)]
mod sei_tests {
    use super::*;

    #[test]
    fn test_sei_correct_number_of_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(2, cpu.sei_cycles(&cpu.imp()));
    }

    #[test]
    fn test_sei_carry_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;

        cpu.sei(&cpu.imp());
        assert_eq!(true, cpu.i);

        cpu.sei(&cpu.imp());
        assert_eq!(true, cpu.i);
    }
}
