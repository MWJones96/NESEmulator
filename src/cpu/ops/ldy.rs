/*
    LDY - Load Index Register Y From Memory
    Operation: M → Y

    Load the index register Y from memory.

    LDY does not affect the C or V flags, sets the
    N flag if the value loaded in bit 7 is a 1,
    otherwise resets N, sets Z flag if the loaded
    value is zero otherwise resets Z and only affects
    the Y register.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn ldyc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn ldy(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        self.y = match mode.addr {
            Some(addr) => bus.read(addr),
            None => mode.data.unwrap(),
        };

        self.n = (self.y & 0x80) > 0;
        self.z = self.y == 0;
    }
}

#[cfg(test)]
mod ldy_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_ldy_imm_correctc() {
        let cpu = NESCPU::new();
        let cycles: u8 = cpu.ldyc(&cpu._imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_ldy_zp_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldyc(&cpu._zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_ldy_zpx_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldyc(&cpu._zpx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldy_abs_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldyc(&cpu._abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldy_absx_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldyc(&cpu._absx(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldy_absx_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.x = 0xff;
        let cycles: u8 = cpu.ldyc(&cpu._absx(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_ldy_value_goes_to_y_register() {
        let mut cpu = NESCPU::new();
        cpu.ldy(&cpu._imm(0xff), &mut MockBus::new());
        assert_eq!(0xff, cpu.y);
    }

    #[test]
    fn test_ldy_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.ldy(&cpu._imm(0x80), &mut MockBus::new());
        assert_eq!(true, cpu.n);
        cpu.ldy(&cpu._imm(0x7f), &mut MockBus::new());
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_ldy_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.ldy(&cpu._imm(0x0), &mut MockBus::new());
        assert_eq!(true, cpu.z);
        cpu.ldy(&cpu._imm(0x1), &mut MockBus::new());
        assert_eq!(false, cpu.z);
    }
}
