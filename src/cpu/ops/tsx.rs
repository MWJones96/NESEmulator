/*
    TSX - Transfer Stack Pointer To Index X
    Operation: S → X

    This instruction transfers the value in the stack pointer
    to the index register X.

    TSX does not affect the carry or overflow flags. It sets N
    if bit 7 is on in index X as a result of the instruction,
    otherwise it is reset. If index X is zero as a result of
    the TSX, the Z flag is set, other­ wise it is reset. TSX
    changes the value of index X, making it equal to the
    content of the stack pointer.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn tsx_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn tsx(&mut self, _mode: &AddrModeResult) {
        self.x = self.sp;

        self.n = (self.x & 0x80) > 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod tsx_tests {
    use super::*;

    #[test]
    fn test_tsx_returns_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.tsx_cycles(&cpu.imp()));
    }

    #[test]
    fn test_tsx() {
        let mut cpu = CPU::new();
        cpu.sp = 0xcc;

        cpu.tsx(&cpu.imp());
        assert_eq!(0xcc, cpu.x);
        assert_eq!(0xcc, cpu.sp);
    }

    #[test]
    fn test_tsx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.sp = 0x80;

        cpu.tsx(&cpu.imp());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tsx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0xff;
        cpu.sp = 0x0;

        cpu.tsx(&cpu.imp());
        assert_eq!(true, cpu.z);
    }
}
