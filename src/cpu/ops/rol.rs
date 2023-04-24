/* 
    ROL - Rotate Left
    Operation: C ← /M7...M0/ ← C

    The rotate left instruction shifts either the accumulator 
    or addressed memory left 1 bit, with the input carry being 
    stored in bit 0 and with the input bit 7 being stored in 
    the carry flags.

    The ROL instruction either shifts the accumulator left 1 bit 
    and stores the carry in accumulator bit 0 or does not affect 
    the internal reg­isters at all. The ROL instruction sets carry 
    equal to the input bit 7, sets N equal to the input bit 6, 
    sets the Z flag if the result of the ro­ tate is 0, otherwise 
    it resets Z and does not affect the overflow flag at all.
*/

use crate::cpu::{
    addr::{AddrMode, AddrModeResult},
    bus::Bus,
};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn rol_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrMode::ACC => 2,
            AddrMode::ABSX => 7,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn rol(&mut self, mode: &AddrModeResult, bus: &dyn Bus) {
        let data: u16 = ((mode.data.unwrap() as u16) << 1) | (self.c as u16);

        self.c = data > 0xff;
        self.z = (data as u8) == 0;
        self.n = (data & 0x80) > 0;

        if let Some(addr) = mode.addr {
            bus.write(addr, data as u8);
        } else {
            self.a = data as u8;
        }
    }
}

#[cfg(test)]
mod rol_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_rol_acc() {
        let mut cpu = CPU::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.rol_cycles(&cpu.acc()));
    }

    #[test]
    fn test_rol_zp() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        assert_eq!(5, cpu.rol_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_rol_zpx() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        cpu.x = 0x2;
        bus.expect_read().with(eq(0x2)).times(1).return_const(0x1);

        assert_eq!(6, cpu.rol_cycles(&cpu.zpx(0x0, &bus)));
    }

    #[test]
    fn test_rol_abs() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(0xffff))
            .times(1)
            .return_const(0xaa);

        assert_eq!(6, cpu.rol_cycles(&cpu.abs(0xffff, &bus)));
    }

    #[test]
    fn test_rol_absx() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        cpu.x = 0x2;

        bus.expect_read().with(eq(0x1)).times(1).return_const(0x88);

        assert_eq!(7, cpu.rol_cycles(&cpu.absx(0xffff, &bus)));
    }

    #[test]
    fn test_rol_no_carry_flag_set_carry_flag() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();
        cpu.a = 0xff;

        cpu.rol(&cpu.acc(), &bus);
        assert_eq!(0xfe, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_rol_no_carry_flag_no_set_carry_flag() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();
        cpu.a = 0x1;

        cpu.rol(&cpu.acc(), &bus);
        assert_eq!(0x2, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_rol_with_carry_flag_no_set_carry_flag() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();
        cpu.a = 0x1;
        cpu.c = true;

        cpu.rol(&cpu.acc(), &bus);
        assert_eq!(0x3, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_rol_with_carry_flag_and_set_carry_flag() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();
        cpu.a = 0b1000_0001;
        cpu.c = true;

        cpu.rol(&cpu.acc(), &bus);
        assert_eq!(0b0000_0011, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_rol_zero_flag() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();
        cpu.a = 0b1000_0000;

        cpu.rol(&cpu.acc(), &bus);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_rol_negative_flag() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();
        cpu.a = 0b0100_0000;

        cpu.rol(&cpu.acc(), &bus);
        assert_eq!(true, cpu.n);
    }
}
