use crate::cpu::{bus::CPUBus, CPU};

impl CPU {
    pub(in crate::cpu) fn reset_cycles(&self) -> u8 {
        8
    }

    pub(in crate::cpu) fn reset(&mut self, bus: &dyn CPUBus) {
        let low_byte = bus.read(CPU::RESET_VECTOR) as u16;
        let high_byte = bus.read(CPU::RESET_VECTOR + 1) as u16;

        self.pc = high_byte << 8 | low_byte;
        self.sp = 0xFD;

        self.a = 0x0;
        self.x = 0x0;
        self.y = 0x0;

        self.n = false;
        self.v = false;
        self.d = false;
        self.i = false;
        self.z = false;
        self.c = false;
    }
}

#[cfg(test)]
mod reset_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_reset_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(8, cpu.reset_cycles())
    }

    #[test]
    fn test_reset() {
        let mut cpu = CPU::new();
        cpu.a = 0x1;
        cpu.x = 0x1;
        cpu.y = 0x1;

        cpu.n = true;
        cpu.v = true;
        cpu.d = true;
        cpu.i = true;
        cpu.z = true;
        cpu.c = true;

        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0xFFFC)).once().return_const(0x40);

        bus.expect_read().with(eq(0xFFFD)).once().return_const(0x20);

        cpu.reset(&bus);

        assert_eq!(0x2040, cpu.pc);
        assert_eq!(0xFD, cpu.sp);

        assert_eq!(0x0, cpu.a);
        assert_eq!(0x0, cpu.x);
        assert_eq!(0x0, cpu.y);

        assert_eq!(0b0010_0000, cpu.get_status_byte(false))
    }
}
