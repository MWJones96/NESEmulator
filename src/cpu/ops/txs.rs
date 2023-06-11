/*
    TXS - Transfer Index X To Stack Pointer
    Operation: X → S

    This instruction transfers the value in the index register
    X to the stack pointer.

    TXS changes only the stack pointer, making it equal to the
    content of the index register X. It does not affect any of
    the flags.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn txsc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn txs(&mut self, _mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.sp = self.x;
    }
}

#[cfg(test)]
mod txs_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_txs_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.txsc(&cpu._imp()));
    }

    #[test]
    fn test_txs() {
        let mut cpu = NESCPU::new();
        cpu.x = 0xcc;

        cpu.txs(&cpu._imp(), &mut MockCPUBus::new());
        assert_eq!(0xcc, cpu.sp);
        assert_eq!(0xcc, cpu.x);
    }
}
