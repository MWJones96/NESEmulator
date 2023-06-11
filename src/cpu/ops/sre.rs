/*
    SRE - Logical Shift Right then "Exclusive OR" Memory with Accumulator
    Operation: M / 2 → M, A ⊻ M → A

    The undocumented SRE instruction shifts the specified memory location 1
    bit to the right, with the higher bit of the result always being set to 0,
    and the low bit which is shifted out of the field being stored in the
    carry flag. It then performs a bit-by-bit "EXCLUSIVE OR" of the result and
    the value of the accumulator and stores the result in the accumulator.

    This instruction affects the accumulator. It does not affect the overflow
    flag. The negative flag is set if the accumulator result contains bit 7 on,
    otherwise the negative flag is reset. The Z flag is set if the result is 0
    and reset otherwise. The carry is set equal to input bit 0.
*/

use crate::{
    bus::Bus,
    cpu::{
        addr::{AddrModeResult, AddrModeType},
        NESCPU,
    },
};

impl NESCPU {
    pub(in crate::cpu) fn srec(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absx => 7,
            AddrModeType::Absy => 7,
            AddrModeType::Indy => 8,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn sre(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        let data_to_write = mode.data.unwrap() >> 1;
        bus.write(mode.addr.unwrap(), data_to_write);
        self.a ^= data_to_write;

        self.c = (mode.data.unwrap() & 0x1) != 0;
        self.n = (self.a & 0x80) != 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod sre_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_sre_zp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.srec(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_sre_zpx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.srec(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_sre_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.srec(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_sre_absx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.srec(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_sre_absy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.srec(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_sre_indx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.srec(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_sre_indy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.srec(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_sre() {
        let mut cpu = NESCPU::new();
        cpu.a = 0x1;
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0xff);
        bus.expect_write()
            .with(eq(0x0), eq(0x7f))
            .once()
            .return_const(());

        cpu.sre(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.c);
        assert_eq!(0x7e, cpu.a);
    }

    #[test]
    fn test_sre_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0xff);
        bus.expect_write()
            .with(eq(0x0), eq(0x7f))
            .once()
            .return_const(());

        cpu.sre(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.n);
        assert_eq!(0x80, cpu.a);
    }

    #[test]
    fn test_sre_zero_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x00);
        bus.expect_write()
            .with(eq(0x0), eq(0x00))
            .once()
            .return_const(());

        cpu.sre(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.z);
        assert_eq!(0x0, cpu.a);
    }
}
