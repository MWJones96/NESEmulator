/*
    X-Indexed Zero Page Indirect

    In indexed indirect addressing, the second byte of the
    instruction is added to the contents of the X index
    register, discarding the carry. The result of this
    addition points to a memory location on page zero whose
    contents is the low order eight bits of the effective
    address. The next memory location in page zero contains
    the high order eight bits of the effective address. Both
    memory locations specifying the high and low order bytes
    of the effective address must be in page zero.

    Bytes: 2
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrModeResult, AddrModeType};

impl CPU {
    pub(in crate::cpu) fn indx(&mut self, bus: &dyn CPUBus) -> AddrModeResult {
        let addr = self.fetch_byte(bus);
        self._indx(addr, bus)
    }

    pub(in crate::cpu) fn _indx(&self, addr: u8, bus: &dyn CPUBus) -> AddrModeResult {
        let low_byte_addr = addr.wrapping_add(self.x);
        let high_byte_addr = low_byte_addr.wrapping_add(1);

        let resolved_addr =
            ((bus.read(high_byte_addr as u16) as u16) << 8) | bus.read(low_byte_addr as u16) as u16;

        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 4,
            mode: AddrModeType::Indx,
            addr: Some(resolved_addr),
            bytes: 2,
            operands: format!("{:02X}", addr),
            repr: format!("(${:02X},X)", addr),
        }
    }
}

#[cfg(test)]
mod indx_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    #[test]
    fn test_indx_addressing_mode() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);
        mock_bus.expect_read().with(eq(0x2)).return_const(0x88);

        mock_bus.expect_read().with(eq(0x8877)).return_const(0xaa);

        cpu.x = 0x2;
        let result = cpu._indx(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0xaa),
                cycles: 4,
                mode: AddrModeType::Indx,
                addr: Some(0x8877),
                bytes: 2,
                operands: "FF".to_owned(),
                repr: "($FF,X)".to_owned()
            },
            result
        );
    }
}
