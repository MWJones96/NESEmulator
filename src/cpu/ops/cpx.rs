/*
    CPX - Compare Index Register X To Memory
    Operation: X - M

    This instruction subtracts the value of the addressed
    memory location from the content of index register X
    using the adder but does not store the result;
    therefore, its only use is to set the N, Z and C flags
    to allow for comparison between the index register X
    and the value in memory.

    The CPX instruction does not affect any register in the
    machine; it also does not affect the overflow flag.
    It causes the carry to be set on if the absolute value
    of the index register X is equal to or greater than the
    data from memory. If the value of the memory is greater
    than the content of the index register X, carry is reset.
    If the results of the subtraction contain a bit 7,
    then the N flag is set, if not, it is reset.
    If the value in memory is equal to the value in index
    register X, the Z flag is set, otherwise it is reset.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn cpxc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn cpx(&mut self, mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        let data = mode.data.unwrap();
        let result = self.x.wrapping_add(!data).wrapping_add(1);

        self.n = (result & 0x80) > 0;
        self.z = self.x == data;
        self.c = self.x >= data;
    }
}

#[cfg(test)]
mod cpx_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_cpx_imm_correct_number_of_cycles() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.cpxc(&cpu._imm(0x0)));
    }

    #[test]
    fn test_cpx_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.cpxc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_cpx_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.cpxc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_cpx_negative_flag() {
        let mut cpu = CPU::new();

        cpu.x = 0x10;
        cpu.cpx(&cpu._imm(0x11), &mut MockCPUBus::new());

        assert_eq!(true, cpu.n);
        assert_eq!(0x10, cpu.x);
    }

    #[test]
    fn test_cpx_zero_flag() {
        let mut cpu = CPU::new();

        cpu.x = 0x20;
        cpu.cpx(&cpu._imm(0x20), &mut MockCPUBus::new());

        assert_eq!(true, cpu.z);
        assert_eq!(0x20, cpu.x);
    }

    #[test]
    fn test_cpx_carry_flag() {
        let mut cpu = CPU::new();

        cpu.x = 0x20;
        cpu.cpx(&cpu._imm(0x20), &mut MockCPUBus::new());

        assert_eq!(true, cpu.c);
        assert_eq!(0x20, cpu.x);

        cpu.x = 0x20;
        cpu.cpx(&cpu._imm(0x10), &mut MockCPUBus::new());

        assert_eq!(true, cpu.c);
        assert_eq!(0x20, cpu.x);

        cpu.x = 0x20;
        cpu.cpx(&cpu._imm(0x21), &mut MockCPUBus::new());

        assert_eq!(false, cpu.c);
        assert_eq!(0x20, cpu.x);
    }
}
