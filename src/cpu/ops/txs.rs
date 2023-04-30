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
    pub(in crate::cpu) fn txs_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn txs(&mut self, _mode: &AddrModeResult) {
        self.sp = self.x;

        self.n = (self.sp & 0x80) > 0;
        self.z = self.sp == 0;
    }
}

#[cfg(test)]
mod txs_tests {
    use super::*;

    #[test]
    fn test_txs_returns_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.txs_cycles(&cpu.imp()));
    }

    #[test]
    fn test_txs() {
        let mut cpu = CPU::new();
        cpu.x = 0xcc;

        cpu.txs(&cpu.imp());
        assert_eq!(0xcc, cpu.sp);
        assert_eq!(0xcc, cpu.x);
    }

    #[test]
    fn test_txs_negative_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x80;

        cpu.txs(&cpu.imp());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_txs_zero_flag() {
        let mut cpu = CPU::new();
        cpu.sp = 0xff;

        cpu.txs(&cpu.imp());
        assert_eq!(true, cpu.z);
    }
}
