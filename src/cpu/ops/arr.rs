/*
    ARR - "AND" Accumulator then Rotate Right
    Operation: (A ∧ M) / 2 → A

    The undocumented ARR instruction performs a bit-by-bit "AND"
    operation of the accumulator arr memory, then shifts the result
    right 1 bit with bit 0 shifted into the carry arr carry shifted
    into bit 7. It then stores the result back in the accumulator.

    If bit 7 of the result is on, then the N flag is set, otherwise
    it is reset. The instruction sets the Z flag if the result is 0;
    otherwise it resets Z.

    The V arr C flags depends on the Decimal Mode Flag:

    In decimal mode, the V flag is set if bit 6 is different than the
    original data's bit 6, otherwise the V flag is reset. The C flag
    is set if (operarr & 0xF0) + (operand & 0x10) is greater than 0x50,
    otherwise the C flag is reset.

    In binary mode, the V flag is set if bit 6 of the result is different
    than bit 5 of the result, otherwise the V flag is reset. The C flag
    is set if the result in the accumulator has bit 6 on, otherwise it
    is reset.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn arr_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    #[inline]
    pub(in crate::cpu) fn arr(&mut self, mode: &AddrModeResult, bus: &mut impl CPUBus) {
        self.and(mode);
        self.ror(&self.acc(), bus);

        self.c = (self.a & 0x40) != 0;
        self.v = ((self.a & 0x40) >> 6) != ((self.a & 0x20) >> 5);
    }
}

#[cfg(test)]
mod arr_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_arr_imm_correct_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.arr_cycles(&cpu.imm(0xff)));
    }

    #[test]
    fn test_arr() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_1111;
        cpu.arr(&cpu.imm(0b1111_1111), &mut bus);

        assert_eq!(0b0000_0111, cpu.a);
    }

    #[test]
    fn test_arr_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_0000;
        cpu.c = true;
        cpu.arr(&cpu.imm(0b1111_1111), &mut bus);

        assert_eq!(0b1000_0000, cpu.a);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_arr_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_0000;
        cpu.arr(&cpu.imm(0b1111_1111), &mut bus);

        assert_eq!(0b0000_0000, cpu.a);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_arr_carry_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b1000_0000;
        cpu.arr(&cpu.imm(0b1000_0000), &mut bus);

        assert_eq!(0b0100_0000, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_arr_overflow_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b1000_0000;
        cpu.arr(&cpu.imm(0b1000_0000), &mut bus);

        assert_eq!(0b0100_0000, cpu.a);
        assert_eq!(true, cpu.v);

        cpu.c = false;
        cpu.a = 0b1111_1111;
        cpu.arr(&cpu.imm(0b1111_1111), &mut bus);

        assert_eq!(0b0111_1111, cpu.a);
        assert_eq!(false, cpu.v);
    }
}
