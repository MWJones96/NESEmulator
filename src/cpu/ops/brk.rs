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

use crate::cpu::bus::Bus;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn brk(&mut self, _reason: u8, bus: &dyn Bus) -> u8 {
        self.i = true;

        let pc_lsb = (self.pc & 0xff) as u8;
        let pc_msb = (self.pc >> 8) as u8;

        bus.write(0x100 + self.sp as u16, pc_msb);
        self.sp -= 1;
        bus.write(0x100 + self.sp as u16, pc_lsb);
        self.sp -= 1;

        self.b = true;
        bus.write(0x100 + self.sp as u16, self.get_status_byte());
        self.sp -= 1;
        self.b = false;

        self.pc = ((bus.read(CPU::INTERRUPT_VECTOR) as u16) << 8) 
            | bus.read(CPU::INTERRUPT_VECTOR.wrapping_add(1)) as u16;

        7
    }
}

#[cfg(test)]
mod brk_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_brk_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_write().return_const(());
        bus.expect_read().return_const(0x0);

        assert_eq!(7, cpu.brk(0x0, &bus));
    }

    #[test]
    fn test_brk_sets_interrupt_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_write().return_const(());
        bus.expect_read().return_const(0x0);

        cpu.brk(0x0, &bus);

        assert_eq!(true, cpu.i);
    }

    #[test]
    fn test_brk_pushes_pc_on_the_stack() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
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

        cpu.brk(0x0, &bus);
    }

    #[test]
    fn test_push_status_register_on_the_stack() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        cpu.c = true;
        cpu.z = true;

        bus.expect_write()
            .with(eq(0x1fd), eq(0b0001_0111))
            .times(1)
            .return_const(());

        bus.expect_write().return_const(());

        bus.expect_read().return_const(0x0);

        cpu.brk(0x0, &bus);

        assert_eq!(0xfc, cpu.sp);
    }

    #[test]
    fn test_program_goes_to_correct_address() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_write().return_const(());

        bus.expect_read()
            .with(eq(CPU::INTERRUPT_VECTOR))
            .times(1)
            .return_const(0x20);

        bus.expect_read()
            .with(eq(CPU::INTERRUPT_VECTOR + 1))
            .times(1)
            .return_const(0x40);

        cpu.brk(0x0, &bus);

        assert_eq!(0x4020, cpu.pc);
        assert_eq!(true, cpu.i);
        assert_eq!(0b0000_0100, cpu.get_status_byte());
    }
}
