/*
    RRA - Rotate Right and Add Memory to Accumulator
    Operation: C → /M7...M0/ → C, A + M + C → A, C

    The undocumented RRA instruction shifts the addressed memory
    right 1 bit with bit 0 shifted into the carry and carry shifted
    into bit 7. It then adds the result and generated carry to the
    value of the accumulator and stores the result in the accumulator.

    This instruction affects the accumulator; sets the carry flag when
    the sum of a binary add exceeds 255 or when the sum of a decimal
    add exceeds 99, otherwise carry is reset. The overflow flag is set
    when the sign or bit 7 is changed due to the result exceeding +127
    or -128, otherwise overflow is reset. The negative flag is set if
    the accumulator result contains bit 7 on, otherwise the negative
    flag is reset. The zero flag is set if the accumulator result is 0,
    otherwise the zero flag is reset.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
    CPU,
};

impl CPU {
    pub(in crate::cpu) fn rrac(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absx => 7,
            AddrModeType::Absy => 7,
            AddrModeType::Indy => 8,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn rra(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let data = mode.data.unwrap();
        let data_to_write = (self.c as u8) << 7 | data >> 1;
        self.c = (data & 0x1) != 0;
        bus.write(mode.addr.unwrap(), data_to_write);
        self.adc(&self._imm(data_to_write), bus);
    }
}

#[cfg(test)]
mod rra_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_rra_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.rrac(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_rra_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.rrac(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_rra_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.rrac(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_rra_absx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.rrac(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_rra_absy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.rrac(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_rra_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.rrac(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_rra_indy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(8, cpu.rrac(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_rra() {
        let mut cpu = CPU::new();
        cpu.c = true;
        cpu.a = 0x1;

        let mut bus = MockCPUBus::new();
        bus.expect_read()
            .with(eq(0x0))
            .once()
            .return_const(0b0000_0001);
        bus.expect_write()
            .with(eq(0x0), eq(0b1000_0000))
            .once()
            .return_const(());

        cpu.rra(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(0x82, cpu.a);
        assert_eq!(false, cpu.c);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_rra_overflow() {
        let mut cpu = CPU::new();
        cpu.a = 0x7f;

        let mut bus = MockCPUBus::new();
        bus.expect_read()
            .with(eq(0x0))
            .once()
            .return_const(0b0000_0010);
        bus.expect_write()
            .with(eq(0x0), eq(0b0000_0001))
            .once()
            .return_const(());

        cpu.rra(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(0x80, cpu.a);
        assert_eq!(true, cpu.v);
    }

    #[test]
    fn test_rra_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_read()
            .with(eq(0x0))
            .once()
            .return_const(0b0000_0010);
        bus.expect_write()
            .with(eq(0x0), eq(0b0000_0001))
            .once()
            .return_const(());

        cpu.rra(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(0x0, cpu.a);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_rra_carry_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_read()
            .with(eq(0x0))
            .once()
            .return_const(0b0000_0010);
        bus.expect_write()
            .with(eq(0x0), eq(0b0000_0001))
            .once()
            .return_const(());

        cpu.rra(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(0x0, cpu.a);
        assert_eq!(true, cpu.c);
    }
}
