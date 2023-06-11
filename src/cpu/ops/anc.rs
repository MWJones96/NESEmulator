/*
    ANC - "AND" Memory with Accumulator then Move
    Negative Flag to Carry Flag (Undocumented)
    Operation: A ∧ M → A, N → C

    The undocumented ANC instruction performs a
    bit-by-bit AND operation of the accumulator and
    memory and stores the result back in the accumulator.

    This instruction affects the accumulator; sets the
    zero flag if the result in the accumulator is 0,
    otherwise resets the zero flag; sets the negative
    flag and the carry flag if the result in the accumulator
    has bit 7 on, otherwise resets the negative flag and
    the carry flag.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn ancc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn anc(&mut self, mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.a &= mode.data.unwrap();

        self.z = self.a == 0;
        self.n = (self.a & 0x80) != 0;
        self.c = self.n;
    }
}

#[cfg(test)]
mod anc_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_anc_imm_correctc() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.ancc(&cpu._imm(0xff)));
    }

    #[test]
    fn test_anc() {
        let mut cpu = NESCPU::new();
        cpu.a = 0b1010_1010_u8;

        cpu.anc(&cpu._imm(0b0101_0101_u8), &mut MockCPUBus::new());

        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_anc_all_ones() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;

        cpu.anc(&cpu._imm(0xff), &mut MockCPUBus::new());

        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_anc_half_ones() {
        let mut cpu = NESCPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.anc(&cpu._imm(0b0000_1111_u8), &mut MockCPUBus::new());

        assert_eq!(0xf, cpu.a);
    }

    #[test]
    fn test_anc_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.anc(&cpu._imm(0b0000_1111_u8), &mut MockCPUBus::new());

        assert_eq!(false, cpu.z);

        cpu.anc(&cpu._imm(0b0000_0000_u8), &mut MockCPUBus::new());

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_anc_negative_and_carry_flags() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;

        cpu.anc(&cpu._imm(0xff), &mut MockCPUBus::new());

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.c);
    }
}
