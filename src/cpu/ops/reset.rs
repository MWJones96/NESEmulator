use crate::{bus::Bus, cpu::NESCPU};

impl NESCPU {
    pub(in crate::cpu) fn resetc(&self) -> u8 {
        7
    }

    pub(in crate::cpu) fn reset(&mut self, bus: &dyn Bus) {
        let low_byte = bus.read(NESCPU::RESET_VECTOR) as u16;
        let high_byte = bus.read(NESCPU::RESET_VECTOR + 1) as u16;

        self.i = true;
        self.pc = high_byte << 8 | low_byte;
        self.sp = 0xFD;
    }
}

#[cfg(test)]
mod reset_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_reset_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(7, cpu.resetc())
    }

    #[test]
    fn test_reset() {
        let mut cpu = NESCPU::new();
        cpu.a = 0x1;
        cpu.x = 0x1;
        cpu.y = 0x1;

        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .once()
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .once()
            .return_const(0x20);

        cpu.reset(&bus);

        assert_eq!(0x2040, cpu.pc);
        assert_eq!(0xFD, cpu.sp);

        assert_eq!(0x1, cpu.a);
        assert_eq!(0x1, cpu.x);
        assert_eq!(0x1, cpu.y);

        assert_eq!(0b0010_0100, cpu.get_status_byte(false))
    }
}
