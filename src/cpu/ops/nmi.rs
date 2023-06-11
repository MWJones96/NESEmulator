use crate::{bus::Bus, cpu::NESCPU};

impl NESCPU {
    pub(in crate::cpu) fn nmic(&self) -> u8 {
        7
    }

    pub(in crate::cpu) fn nmi(&mut self, bus: &mut dyn Bus) {
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
            (bus.read(NESCPU::NMI_VECTOR + 1) as u16) << 8 | bus.read(NESCPU::NMI_VECTOR) as u16;

        self.pending_nmi = false;
    }
}

#[cfg(test)]
mod nmi_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_nmi_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(7, cpu.nmic())
    }

    #[test]
    fn test_nmi() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.pc = 0x2040;
        cpu.sp = 0xff;

        //NMI should not interrupt an NMI immediately after the NMI routine
        cpu.pending_nmi = true;

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
            .with(eq(NESCPU::NMI_VECTOR))
            .once()
            .return_const(0x00);

        bus.expect_read()
            .with(eq(NESCPU::NMI_VECTOR + 1))
            .once()
            .return_const(0x80);

        cpu.nmi(&mut bus);

        assert_eq!(0x8000, cpu.pc);
        assert_eq!(0xfc, cpu.sp);

        assert_eq!(false, cpu.pending_nmi);
    }
}
