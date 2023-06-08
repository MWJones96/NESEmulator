/*
    DCP - Decrement Memory By One then Compare with Accumulator
    Operation: M - 1 → M, A - M

    This undocumented instruction subtracts 1, in two's complement,
    from the contents of the addressed memory location. It then
    subtracts the contents of memory from the contents of the
    accumulator.

    The DCP instruction does not affect any internal register in the
    microprocessor. It does not affect the overflow flag. Z flag is
    set on an equal comparison, reset otherwise; the N flag is set or
    reset by the result bit 7, the carry flag is set when the result
    in memory is less than or equal to the accumulator, reset when it
    is greater than the accumulator.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
    CPU,
};

impl CPU {
    pub(in crate::cpu) fn dcpc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absx => 7,
            AddrModeType::Absy => 7,
            AddrModeType::Indy => 8,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn dcp(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let data = mode.data.unwrap();
        let data_to_write = data.wrapping_sub(1);

        bus.write(mode.addr.unwrap(), data_to_write);

        let cmp = self.a.wrapping_sub(data_to_write);
        self.n = (cmp & 0x80) != 0;
        self.z = data_to_write == self.a;
        self.c = data_to_write <= self.a;
    }
}

#[cfg(test)]
mod dcp_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_dcp_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.dcpc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_dcp_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.dcpc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_dcp_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.dcpc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_dcp_absx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.dcpc(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_dcp_absy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.dcpc(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_dcp_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.dcpc(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_dcp_indy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.dcpc(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_dcp() {
        let mut cpu = CPU::new();
        cpu.a = 0x10;

        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x0)).once().return_const(0x2);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.dcp(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(0x10, cpu.a);
    }

    #[test]
    fn test_dcp_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0x81;

        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x0)).once().return_const(0x2);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.dcp(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.n);
        assert_eq!(0x81, cpu.a);
    }

    #[test]
    fn test_dcp_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0x1;

        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x0)).once().return_const(0x2);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .once()
            .return_const(());

        cpu.dcp(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.z);
        assert_eq!(0x1, cpu.a);
    }

    #[test]
    fn test_dcp_carry_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0x10;

        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x0)).times(3).return_const(0x2);
        bus.expect_write()
            .with(eq(0x0), eq(0x1))
            .times(3)
            .return_const(());

        cpu.dcp(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.c);
        assert_eq!(0x10, cpu.a);

        cpu.a = 0x1;
        cpu.dcp(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.c);
        assert_eq!(0x1, cpu.a);

        cpu.a = 0x0;
        cpu.dcp(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(false, cpu.c);
        assert_eq!(0x0, cpu.a);
    }
}
