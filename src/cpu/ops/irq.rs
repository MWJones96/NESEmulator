use crate::{bus::Bus, cpu::NESCPU};

impl NESCPU {
    pub(in crate::cpu) fn irqc(&self) -> u8 {
        7
    }

    pub(in crate::cpu) fn irq(&mut self, bus: &mut dyn Bus) {
        let pc_high: u8 = (self.pc >> 8) as u8;
        let pc_low: u8 = self.pc as u8;

        bus.write(0x100 + self.sp.wrapping_sub(0) as u16, pc_high);
        bus.write(0x100 + self.sp.wrapping_sub(1) as u16, pc_low);
        bus.write(
            0x100 + self.sp.wrapping_sub(2) as u16,
            self.get_status_byte(false),
        );

        self.i = true;
        self.sp = self.sp.wrapping_sub(3);
        self.pc =
            (bus.read(NESCPU::IRQ_VECTOR + 1) as u16) << 8 | bus.read(NESCPU::IRQ_VECTOR) as u16;
    }
}

#[cfg(test)]
mod irq_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_irq_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(7, cpu.irqc());
    }

    #[test]
    fn test_irq() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.pc = 0x2040;
        cpu.sp = 0xff;

        bus.expect_write()
            .with(eq(0x1ff), eq(0x20))
            .once()
            .return_const(());

        bus.expect_write()
            .with(eq(0x1fe), eq(0x40))
            .once()
            .return_const(());

        bus.expect_write()
            .with(eq(0x1fd), eq(0b0010_0100))
            .once()
            .return_const(());

        bus.expect_read()
            .with(eq(NESCPU::IRQ_VECTOR))
            .once()
            .return_const(0x00);

        bus.expect_read()
            .with(eq(NESCPU::IRQ_VECTOR + 1))
            .once()
            .return_const(0x80);

        cpu.irq(&mut bus);

        assert_eq!(0x8000, cpu.pc);
        assert_eq!(0xfc, cpu.sp);
    }
}
