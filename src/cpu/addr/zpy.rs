/*
    Y-Indexed Zero Page

    This form of addressing is used in conjunction with the
    Y index register. The effective address is calculated
    by adding the second byte to the contents of the index
    register. Since this is a form of "Zero Page" addressing,
    the content of the second byte references a location in
    page zero. Additionally, due to the “Zero Page" addressing
    nature of this mode, no carry is added to the high order
    8 bits of memory and crossing of page boundaries does
    not occur.

    Bytes: 2
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrModeResult, AddrModeType};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn zpy(&mut self, bus: &dyn CPUBus) -> AddrModeResult {
        let addr = self.fetch_byte(bus);
        self._zpy(addr, bus)
    }

    #[inline]
    pub(in crate::cpu) fn _zpy(&self, addr: u8, bus: &dyn CPUBus) -> AddrModeResult {
        let resolved_addr = addr.wrapping_add(self.y) as u16;

        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 2,
            mode: AddrModeType::ZPY,
            addr: Some(resolved_addr),
            bytes: 2,
            operands: format!("{:02X}", addr),
            repr: format!("${:02X},Y", addr),
        }
    }
}

#[cfg(test)]
mod zpy_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    #[test]
    fn test_zpy_addressing_mode() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();
        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);

        let result = cpu._zpy(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x77),
                cycles: 2,
                mode: AddrModeType::ZPY,
                addr: Some(0x1),
                bytes: 2,
                operands: "FF".to_owned(),
                repr: "$FF,Y".to_owned()
            },
            result
        );
    }
}
