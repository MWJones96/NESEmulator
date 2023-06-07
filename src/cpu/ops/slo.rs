/*
    SLO - Arithmetic Shift Left then "OR" Memory with Accumulator
    Operation: M * 2 → M, A ∨ M → A

    The undocumented SLO instruction shifts the address memory location 1
    bit to the left, with the bit 0 always being set to 0 and the bit 7
    output always being contained in the carry flag. It then performs a
    bit-by-bit "OR" operation on the result and the accumulator and stores
    the result in the accumulator.

    The negative flag is set if the accumulator result contains bit 7 on,
    otherwise the negative flag is reset. It sets Z flag if the result is
    equal to 0, otherwise resets Z and stores the input bit 7 in the carry
    flag.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
    CPU,
};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn slo_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::ABSX => 7,
            AddrModeType::ABSY => 7,
            AddrModeType::INDY => 8,
            _ => 4 + mode.cycles,
        }
    }

    #[inline]
    pub(in crate::cpu) fn slo(&mut self, mode: &AddrModeResult, bus: &mut impl CPUBus) {
        let data_to_write = mode.data.unwrap() << 1;
        bus.write(mode.addr.unwrap(), data_to_write);
        self.a |= data_to_write;

        self.c = (mode.data.unwrap() & 0x80) != 0;
        self.n = (self.a & 0x80) != 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod slo_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_slo_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.slo_cycles(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_slo_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.slo_cycles(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_slo_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.slo_cycles(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_slo_absx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.slo_cycles(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_slo_absy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.slo_cycles(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_slo_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.slo_cycles(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_slo_indy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.slo_cycles(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_slo() {
        let mut cpu = CPU::new();
        cpu.a = 0x1;
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0xff);
        bus.expect_write()
            .with(eq(0x0), eq(0xfe))
            .once()
            .return_const(());

        cpu.slo(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.c);
        assert_eq!(true, cpu.n);
        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_slo_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);
        bus.expect_write()
            .with(eq(0x0), eq(0x0))
            .once()
            .return_const(());

        cpu.slo(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(false, cpu.c);
        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.z);
        assert_eq!(0x0, cpu.a);
    }
}
