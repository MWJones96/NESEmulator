/*
    ASR - "AND" then Logical Shift Right
    Operation: (A ∧ M) / 2 → A

    The undocumented ASR instruction performs a bit-by-bit AND
    operation of the accumulator and memory, then shifts the
    accumulator 1 bit to the right, with the higher bit of the
    result always being set to 0, and the low bit which is
    shifted out of the field being stored in the carry flag.

    This instruction affects the accumulator. It does not affect
    the overflow flag. The N flag is always reset. The Z flag is
    set if the result of the shift is 0 and reset otherwise. The
    carry is set equal to bit 0 of the result of the "AND" operation.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn asrc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn asr(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        self.and(mode, bus);
        self.lsr(&self._acc(), bus)
    }
}

#[cfg(test)]
mod asr_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_asr_imm_correctc() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.asrc(&cpu._imm(0xff)));
    }

    #[test]
    fn test_asr() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b1111_1111;
        cpu.n = true;

        cpu.asr(&cpu._imm(0b1111_0000), &mut bus);

        assert_eq!(false, cpu.n);
        assert_eq!(0b0111_1000, cpu.a);
    }

    #[test]
    fn test_asr_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_0001;

        cpu.asr(&cpu._imm(0b0000_0001), &mut bus);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_asr_carry_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_0001;

        cpu.asr(&cpu._imm(0b0000_0001), &mut bus);

        assert_eq!(true, cpu.c);
    }
}
