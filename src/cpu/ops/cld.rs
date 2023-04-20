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

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn cld(&mut self) -> u8 {
        self.d = false;
        2
    }
}

#[cfg(test)]
mod cld_tests {
    use super::*;

    #[test]
    fn test_cld_correct_number_of_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(2, cpu.cld());
    }

    #[test]
    fn test_cld_carry_flag() {
        let mut cpu = CPU::new();
        cpu.d = true;

        cpu.cld();
        assert_eq!(false, cpu.d);

        cpu.cld();
        assert_eq!(false, cpu.d);
    }
}
