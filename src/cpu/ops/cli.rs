/*
    CLI - Clear Interrupt Disable
    Operation: 0 → I

    This instruction initializes the interrupt disable to a 0.
    This allows the microprocessor to receive interrupts.

    It affects no registers in the microprocessor and no flags
    other than the interrupt disable which is cleared.
*/

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn cli_cycles(&mut self) -> u8 {
        2
    }

    pub(in crate::cpu) fn cli(&mut self) {
        self.i = false;
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_cli_correct_number_of_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(2, cpu.cli_cycles());
    }

    #[test]
    fn test_cli_carry_flag() {
        let mut cpu = CPU::new();
        cpu.i = true;

        cpu.cli();
        assert_eq!(false, cpu.i);

        cpu.cli();
        assert_eq!(false, cpu.i);
    }
}
