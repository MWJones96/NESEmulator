/*
    RLA - Rotate Left then "AND" with Accumulator
    Operation: C ← /M7...M0/ ← C, A ∧ M → A

    The undocumented RLA instruction shifts the addressed memory
    left 1 bit, with the input carry being stored in bit 0 and
    with the input bit 7 being stored in the carry flags. It then
    performs a bit-by-bit AND operation of the result and the
    value of the accumulator and stores the result back in the
    accumulator.

    This instruction affects the accumulator; sets the zero flag
    if the result in the accumulator is 0, otherwise resets the
    zero flag; sets the negative flag if the result in the
    accumulator has bit 7 on, otherwise resets the negative flag.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
    CPU,
};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn rla_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::ABSX => 7,
            AddrModeType::ABSY => 7,
            AddrModeType::INDY => 8,
            _ => 4 + mode.cycles,
        }
    }

    #[inline]
    pub(in crate::cpu) fn rla(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let data = mode.data.unwrap();
        let data_to_write = data << 1 | (self.c as u8);
        bus.write(mode.addr.unwrap(), data_to_write);
        self.c = (data & 0x80) != 0;

        self.a &= data_to_write;
        self.z = self.a == 0;
        self.n = (self.a & 0x80) != 0;
    }
}

#[cfg(test)]
mod rla_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_rla_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.rla_cycles(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_rla_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.rla_cycles(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_rla_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.rla_cycles(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_rla_absx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.rla_cycles(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_rla_absy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.rla_cycles(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_rla_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.rla_cycles(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_rla_indy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.rla_cycles(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_rla() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;
        cpu.c = true;

        let mut bus = MockCPUBus::new();
        bus.expect_read()
            .with(eq(0x0))
            .once()
            .return_const(0b1000_0000);
        bus.expect_write()
            .with(eq(0x0), eq(0b0000_0001))
            .once()
            .return_const(());

        cpu.rla(&cpu._zp(0x0, &bus), &mut bus);

        assert_eq!(true, cpu.c);
        assert_eq!(0x1, cpu.a);
    }

    #[test]
    fn test_rla_zero_flag() {
        let mut cpu = CPU::new();

        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x0)).once().return_const(0x0);
        bus.expect_write()
            .with(eq(0x0), eq(0x0))
            .once()
            .return_const(());

        cpu.rla(&cpu._zp(0x0, &bus), &mut bus);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_rla_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0b1000_0000;

        let mut bus = MockCPUBus::new();
        bus.expect_read()
            .with(eq(0x0))
            .once()
            .return_const(0b0100_0000);
        bus.expect_write()
            .with(eq(0x0), eq(0b1000_0000))
            .once()
            .return_const(());

        cpu.rla(&cpu._zp(0x0, &bus), &mut bus);

        assert_eq!(true, cpu.n);
    }
}
