/*
    SHA - Store Accumulator "AND" Index Register X "AND" Value
    Operation: A ∧ X ∧ V → M

    The undocumented SHA instruction performs a bit-by-bit AND
    operation of the following three operands: The first two are
    the accumulator and the index register X.

    The third operand depends on the addressing mode. In the zero
    page indirect Y-indexed case, the third operand is the data in
    memory at the given zero page address (ignoring the the addressing
    mode's Y offset) plus 1. In the Y-indexed absolute case, it is the
    upper 8 bits of the given address (ignoring the the addressing
    mode's Y offset), plus 1.

    It then transfers the result to the addressed memory location.

    No flags or registers in the microprocessor are affected by the store
    operation.
*/

use crate::{
    bus::Bus,
    cpu::{
        addr::{AddrModeResult, AddrModeType},
        NESCPU,
    },
};

impl NESCPU {
    pub(in crate::cpu) fn shac(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absy => 5,
            AddrModeType::Indy => 6,
            addr => panic!("Addressing mode {:?} not implemented for SHA", addr),
        }
    }

    pub(in crate::cpu) fn sha(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        let write_addr = mode.addr.unwrap();
        let ax = self.a & self.x;

        match mode.mode {
            AddrModeType::Absy => {
                let v = ((write_addr.wrapping_sub(self.y as u16) >> 8) as u8).wrapping_add(1);
                bus.write(write_addr, ax & v);
            }
            AddrModeType::Indy => {
                let v = bus.read(write_addr.wrapping_sub(self.y as u16)) + 1;
                bus.write(write_addr, ax & v);
            }
            mode => panic!("Addressing mode {:?} not implemented for SHA", mode),
        }
    }
}

#[cfg(test)]
mod sha_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::bus::MockBus;

    #[test]
    fn test_sha_absy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.shac(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_sha_indy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.shac(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_sha_absy() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;
        cpu.x = 0xff;
        cpu.y = 0x1;

        let mut bus = MockBus::new();
        bus.expect_write()
            .with(eq(0x1235), eq(0xff & 0x13))
            .once()
            .return_const(());

        cpu.sha(&cpu._absy(0x1234, &bus), &mut bus);
    }

    #[test]
    fn test_sha_indy() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;
        cpu.x = 0xff;
        cpu.y = 0x1;

        let mut bus = MockBus::new();
        bus.expect_read().with(eq(0x0)).once().return_const(0x40);
        bus.expect_read().with(eq(0x1)).once().return_const(0x20);
        bus.expect_read().with(eq(0x2040)).once().return_const(0x10);
        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0x2041), eq(0xff & 0x11))
            .once()
            .return_const(());

        cpu.sha(&cpu._indy(0x0, &bus), &mut bus);
    }
}
