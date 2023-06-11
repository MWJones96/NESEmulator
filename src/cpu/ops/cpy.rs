/*
    CPY - Compare Index Register Y To Memory
    Operation: Y - M

    This instruction performs a two's complement subtraction
    between the index register Y and the specified memory location.
    The results of the subtraction are not stored anywhere.
    The instruction is strict­ly used to set the flags.

    CPY affects no registers in the microprocessor and also does
    not affect the overflow flag. If the value in the index
    register Y is equal to or greater than the value in the
    memory, the carry flag will be set, otherwise it will
    be cleared. If the results of the subtract- tion contain
    bit 7 on the N bit will be set, otherwise it will be cleared.
    If the value in the index register Y and the value in the
    memory are equal, the zero flag will be set, otherwise
    it will be cleared.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn cpyc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn cpy(&mut self, mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        let data = mode.data.unwrap();
        let result = self.y.wrapping_add(!data).wrapping_add(1);

        self.n = (result & 0x80) > 0;
        self.z = self.y == data;
        self.c = self.y >= data;
    }
}

#[cfg(test)]
mod cpy_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_cpy_imm_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        assert_eq!(2, cpu.cpyc(&cpu._imm(0x0)));
    }

    #[test]
    fn test_cpy_zp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.cpyc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_cpy_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.cpyc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_cpy_negative_flag() {
        let mut cpu = NESCPU::new();

        cpu.y = 0x10;
        cpu.cpy(&cpu._imm(0x11), &mut MockCPUBus::new());

        assert_eq!(true, cpu.n);
        assert_eq!(0x10, cpu.y);
    }

    #[test]
    fn test_cpy_zero_flag() {
        let mut cpu = NESCPU::new();

        cpu.y = 0x20;
        cpu.cpy(&cpu._imm(0x20), &mut MockCPUBus::new());

        assert_eq!(true, cpu.z);
        assert_eq!(0x20, cpu.y);
    }

    #[test]
    fn test_cpy_carry_flag() {
        let mut cpu = NESCPU::new();

        cpu.y = 0x20;
        cpu.cpy(&cpu._imm(0x20), &mut MockCPUBus::new());

        assert_eq!(true, cpu.c);
        assert_eq!(0x20, cpu.y);

        cpu.y = 0x20;
        cpu.cpy(&cpu._imm(0x10), &mut MockCPUBus::new());

        assert_eq!(true, cpu.c);
        assert_eq!(0x20, cpu.y);

        cpu.y = 0x20;
        cpu.cpy(&cpu._imm(0x21), &mut MockCPUBus::new());

        assert_eq!(false, cpu.c);
        assert_eq!(0x20, cpu.y);
    }
}
