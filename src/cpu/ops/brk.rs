/*
    BRK - Break Command
    Operation: PC + 2↓, [FFFE] → PCL, [FFFF] → PCH

    The break command causes the microprocessor to go
    through an inter­rupt sequence under program control.
    This means that the program counter of the second byte
    after the BRK. is automatically stored on the stack
    along with the processor status at the beginning of
    the break instruction. The microprocessor then transfers
    control to the interrupt vector.

    Other than changing the program counter, the break instruction
    changes no values in either the registers or the flags.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn brkc(&self, _mode: &AddrModeResult) -> u8 {
        7
    }

    #[inline]
    pub(in crate::cpu) fn brk(&mut self, _mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let pc_lsb = (self.pc & 0xff) as u8;
        let pc_msb = (self.pc >> 8) as u8;

        bus.write(0x100 + (self.sp.wrapping_sub(0)) as u16, pc_msb);
        bus.write(0x100 + (self.sp.wrapping_sub(1)) as u16, pc_lsb);
        bus.write(
            0x100 + (self.sp.wrapping_sub(2)) as u16,
            self.get_status_byte(true),
        );

        self.i = true;
        self.sp = self.sp.wrapping_sub(3);
        self.pc = (bus.read(CPU::IRQ_VECTOR + 1) as u16) << 8 | bus.read(CPU::IRQ_VECTOR) as u16;
    }
}

#[cfg(test)]
mod brk_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_brk_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_write().return_const(());
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.brkc(&cpu._imp()));
    }

    #[test]
    fn test_brk_sets_interrupt_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_write().return_const(());
        bus.expect_read().return_const(0x0);

        cpu.brk(&cpu._imp(), &mut bus);

        assert_eq!(true, cpu.i);
    }

    #[test]
    fn test_brk_pushes_pc_on_the_stack() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.pc = 0x2000;

        bus.expect_write()
            .with(eq(0x1ff), eq(0x20))
            .times(1)
            .return_const(());

        bus.expect_write()
            .with(eq(0x1fe), eq(0x0))
            .times(1)
            .return_const(());

        bus.expect_write().return_const(());
        bus.expect_read().return_const(0x0);

        cpu.brk(&cpu._imp(), &mut bus);
    }

    #[test]
    fn test_push_status_register_on_the_stack() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.c = true;
        cpu.z = true;

        bus.expect_write()
            .with(eq(0x1fd), eq(0b0011_0111))
            .times(1)
            .return_const(());

        bus.expect_write().return_const(());

        bus.expect_read().return_const(0x0);

        cpu.brk(&cpu._imp(), &mut bus);

        assert_eq!(0xfc, cpu.sp);
    }

    #[test]
    fn test_program_goes_to_correct_address() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_write().return_const(());

        bus.expect_read()
            .with(eq(CPU::IRQ_VECTOR))
            .times(1)
            .return_const(0x20);

        bus.expect_read()
            .with(eq(CPU::IRQ_VECTOR + 1))
            .times(1)
            .return_const(0x40);

        cpu.brk(&cpu._imp(), &mut bus);

        assert_eq!(0x4020, cpu.pc);
        assert_eq!(true, cpu.i);
        assert_eq!(0b0011_0100, cpu.get_status_byte(true));
    }

    #[test]
    fn test_push_onto_full_stack_underflow() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0x0;

        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0x100), eq(0x0))
            .times(1)
            .return_const(());

        bus.expect_write()
            .with(eq(0x1ff), eq(0x0))
            .times(1)
            .return_const(());

        bus.expect_write()
            .with(eq(0x1fe), eq(0b0011_0100))
            .times(1)
            .return_const(());

        cpu.brk(&cpu._imp(), &mut bus);

        assert_eq!(0xfd, cpu.sp);
    }
}
