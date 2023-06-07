/*
    TXS - Transfer Index X To Stack Pointer
    Operation: X → S

    This instruction transfers the value in the index register
    X to the stack pointer.

    TXS changes only the stack pointer, making it equal to the
    content of the index register X. It does not affect any of
    the flags.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn txsc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn txs(&mut self, _mode: &AddrModeResult) {
        self.sp = self.x;
    }
}

#[cfg(test)]
mod txs_tests {
    use super::*;

    #[test]
    fn test_txs_returns_correct_number_ofc() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.txsc(&cpu._imp()));
    }

    #[test]
    fn test_txs() {
        let mut cpu = CPU::new();
        cpu.x = 0xcc;

        cpu.txs(&cpu._imp());
        assert_eq!(0xcc, cpu.sp);
        assert_eq!(0xcc, cpu.x);
    }
}
