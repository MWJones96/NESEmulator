/*
    ISC - Increment Memory By One then SBC then Subtract Memory
    from Accumulator with Borrow
    Operation: M + 1 → M, A - M - ~C → A

    This undocumented instruction adds 1 to the contents of the
    addressed memory loca­tion. It then subtracts the value of the
    result in memory and borrow from the value of the accumulator,
    using two's complement arithmetic, and stores the result in
    the accumulator.

    This instruction affects the accumulator. The carry flag is
    set if the result is greater than or equal to 0. The carry
    flag is reset when the result is less than 0, indicating a
    borrow. The over­flow flag is set when the result exceeds
    +127 or -127, otherwise it is reset. The negative flag is
    set if the result in the accumulator has bit 7 on, otherwise
    it is reset. The Z flag is set if the result in the
    accumulator is 0, otherwise it is reset.
*/

use crate::cpu::{
    addr::{AddrMode, AddrModeResult},
    bus::CPUBus,
    CPU,
};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn isc_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrMode::ABSX => 7,
            AddrMode::ABSY => 7,
            AddrMode::INDY => 8,
            _ => 4 + mode.cycles,
        }
    }

    #[inline]
    pub(in crate::cpu) fn isc(&mut self, mode: &AddrModeResult, bus: &mut impl CPUBus) {
        let data = mode.data.unwrap();
        let data_to_write = data.wrapping_add(1);
        bus.write(mode.addr.unwrap(), data_to_write);
        self.sbc(&self.imm(data_to_write));
    }
}

#[cfg(test)]
mod isc_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_isc_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.isc_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_isc_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.isc_cycles(&cpu.zpx(0x0, &bus)));
    }

    #[test]
    fn test_isc_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.isc_cycles(&cpu.abs(0x0, &bus)));
    }

    #[test]
    fn test_isc_absx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.isc_cycles(&cpu.absx(0x0, &bus)));
    }

    #[test]
    fn test_isc_absy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.isc_cycles(&cpu.absy(0x0, &bus)));
    }

    #[test]
    fn test_isc_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.isc_cycles(&cpu.indx(0x0, &bus)));
    }

    #[test]
    fn test_isc_indy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.isc_cycles(&cpu.indy(0x0, &bus)));
    }

    #[test]
    fn test_isc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x1;
        cpu.c = true; //No borrow

        bus.expect_read().with(eq(0x0)).once().return_const(0x0);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.isc(&cpu.zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_isc_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x0;
        cpu.c = true; //No borrow

        bus.expect_read().with(eq(0x0)).once().return_const(0x0);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.isc(&cpu.zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.n);
        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_isc_overflow_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x80;
        cpu.c = true; //No borrow

        bus.expect_read().with(eq(0x0)).once().return_const(0x0);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.isc(&cpu.zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.v);
        assert_eq!(0x7f, cpu.a);
    }

    #[test]
    fn test_isc_overflow_flag_positive() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x7f;
        cpu.c = true; //No borrow

        bus.expect_read().with(eq(0x0)).once().return_const(0xfe);
        bus.expect_write()
            .with(eq(0x0), eq(0xff))
            .once()
            .return_const(());

        cpu.isc(&cpu.zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.v);
        assert_eq!(0x80, cpu.a);
    }

    #[test]
    fn test_isc_overflow_carry_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x2;
        cpu.c = false; //Borrow

        bus.expect_read().with(eq(0x0)).once().return_const(0x0);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.isc(&cpu.zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.c);
        assert_eq!(0x0, cpu.a);
    }
}
