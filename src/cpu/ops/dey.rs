/*
    DEY - Decrement Index Register Y By One
    Operation: Y - 1 → Y

    This instruction subtracts one from the current value
    in the in­ dex register Y and stores the result into the
    index register Y. The result does not affect or consider
    carry so that the value in the index register Y is
    decremented to 0 and then through 0 to FF.

    Decrement Y does not affect the carry or overflow flags;
    if the Y register contains bit 7 on as a result of the
    decrement the N flag is set, otherwise the N flag is
    reset. If the Y register is 0 as a result of the decrement,
    the Z flag is set otherwise the Z flag is reset. This
    instruction only affects the index register Y.
*/

use crate::cpu::CPU;

impl CPU {
    pub(in crate::cpu) fn dey(&mut self) -> u8 {
        self.y = self.y.wrapping_sub(1);

        self.n = (self.y & 0x80) > 0;
        self.z = self.y == 0;

        2
    }
}

#[cfg(test)]
mod dey_tests {
    use super::*;

    #[test]
    fn test_dey_returns_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        assert_eq!(2, cpu.dey());
    }

    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.y = 0x80;

        cpu.dey();

        assert_eq!(0x7f, cpu.y);
    }

    #[test]
    fn test_dey_negative_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0x0;

        cpu.dey();

        assert_eq!(0xff, cpu.y);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_dey_zero_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0x1;

        cpu.dey();

        assert_eq!(0x0, cpu.y);
        assert_eq!(true, cpu.z);
    }
}
