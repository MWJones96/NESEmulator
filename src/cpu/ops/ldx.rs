/*
    LDX - Load Index Register X From Memory
    Operation: M → X

    Load the index register X from memory.

    LDX does not affect the C or V flags; sets Z if
    the value loaded was zero, otherwise resets it;
    sets N if the value loaded in bit 7 is a 1;
    otherwise N is reset, and affects only the
    X register.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn ldxc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn ldx(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        if let Some(addr) = mode.addr {
            self.x = bus.read(addr);
        } else {
            self.x = mode.data.unwrap();
        }
        self.n = (self.x & 0x80) != 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod ldx_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_ldx_imm_correctc() {
        let cpu = NESCPU::new();
        let cycles: u8 = cpu.ldxc(&cpu._imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_ldx_zp_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldxc(&cpu._zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_ldx_zpy_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldxc(&cpu._zpy(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldx_abs_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldxc(&cpu._abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldx_absy_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldxc(&cpu._absy(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldx_absy_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.ldxc(&cpu._absy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_ldx_value_goes_to_x_register() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.ldx(&cpu._imm(0xff), &mut bus);
        assert_eq!(0xff, cpu.x);
    }

    #[test]
    fn test_ldx_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.ldx(&cpu._imm(0x80), &mut MockBus::new());
        assert_eq!(true, cpu.n);
        cpu.ldx(&cpu._imm(0x7f), &mut MockBus::new());
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_ldx_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.ldx(&cpu._imm(0x0), &mut MockBus::new());
        assert_eq!(true, cpu.z);
        cpu.ldx(&cpu._imm(0x1), &mut MockBus::new());
        assert_eq!(false, cpu.z);
    }
}
