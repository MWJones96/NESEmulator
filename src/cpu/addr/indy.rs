/*
    Zero Page Indirect Y-Indexed

    In indirect indexed addressing, the second byte of the instruction
    points to a memory location in page zero. The contents of this memory
    location is added to the contents of the Y index register, the result
    being the low order eight bits of the effective address. The carry
    from this addition is added to the contents of the next page zero
    memory location, the result being the high order eight bits of the
    effective address.

    Bytes: 2
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrModeResult, AddrModeType};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn indy(&mut self, bus: &impl CPUBus) -> AddrModeResult {
        let addr = self.fetch_byte(bus);
        self._indy(addr, bus)
    }

    #[inline]
    pub(in crate::cpu) fn _indy(&self, addr: u8, bus: &impl CPUBus) -> AddrModeResult {
        let low_byte_addr = addr;
        let high_byte_addr = low_byte_addr.wrapping_add(1);

        let resolved_addr =
            (bus.read(high_byte_addr as u16) as u16) << 8 | (bus.read(low_byte_addr as u16) as u16);

        let page_before = (resolved_addr >> 8) as u8;
        let resolved_addr = resolved_addr.wrapping_add(self.y as u16);
        let page_after = (resolved_addr >> 8) as u8;

        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 3 + ((page_before != page_after) as u8),
            mode: AddrModeType::INDY,
            addr: Some(resolved_addr),
            bytes: 2,
            operands: format!("{:02X}", addr),
            repr: format!("(${:02X}),Y", addr),
        }
    }
}

#[cfg(test)]
mod indy_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    #[test]
    fn test_indy_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        mock_bus.expect_read().with(eq(0xff)).return_const(0x77);
        mock_bus.expect_read().with(eq(0x0)).return_const(0x88);

        cpu.y = 0x2;
        mock_bus.expect_read().with(eq(0x8879)).return_const(0xbb);

        let result = cpu._indy(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0xbb),
                cycles: 3,
                mode: AddrModeType::INDY,
                addr: Some(0x8879),
                bytes: 2,
                operands: "FF".to_owned(),
                repr: "($FF),Y".to_owned()
            },
            result
        );
    }

    #[test]
    fn test_indy_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        mock_bus.expect_read().with(eq(0xff)).return_const(0x77);
        mock_bus.expect_read().with(eq(0x0)).return_const(0x88);

        cpu.y = 0xff;
        mock_bus.expect_read().with(eq(0x8976)).return_const(0xcc);

        let result = cpu._indy(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0xcc),
                cycles: 4,
                mode: AddrModeType::INDY,
                addr: Some(0x8976),
                bytes: 2,
                operands: "FF".to_owned(),
                repr: "($FF),Y".to_owned()
            },
            result
        );
    }
}
