/*
    X-Indexed Zero Page

    This form of addressing is used in conjunction with the
    X index register. The effective address is calculated by
    adding the second byte to the contents of the index
    register. Since this is a form of "Zero Page" addressing,
    the content of the second byte references a location in
    page zero. Additionally, due to the “Zero Page" addressing
    nature of this mode, no carry is added to the high order
    8 bits of memory and crossing of page boundaries does
    not occur.

    Bytes: 2
*/

use crate::{bus::Bus, cpu::NESCPU};

use super::{AddrModeResult, AddrModeType};

impl NESCPU {
    pub(in crate::cpu) fn zpx(&mut self, bus: &dyn Bus) -> AddrModeResult {
        let addr = self.fetch_byte(bus);
        self._zpx(addr, bus)
    }

    pub(in crate::cpu) fn _zpx(&self, addr: u8, _bus: &dyn Bus) -> AddrModeResult {
        let resolved_addr = addr.wrapping_add(self.x) as u16;

        AddrModeResult {
            data: None,
            cycles: 2,
            mode: AddrModeType::Zpx,
            addr: Some(resolved_addr),
            bytes: 2,
            operands: format!("{:02X}", addr),
            repr: format!("${:02X},X", addr),
        }
    }
}

#[cfg(test)]
mod zpx_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::{bus::MockBus, cpu::addr::AddrModeResult};

    #[test]
    fn test_zpx_addressing_mode() {
        let mut cpu = NESCPU::new();
        let mut mock_bus = MockBus::new();
        cpu.x = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);

        let result = cpu._zpx(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 2,
                mode: AddrModeType::Zpx,
                addr: Some(0x1),
                bytes: 2,
                operands: "FF".to_owned(),
                repr: "$FF,X".to_owned()
            },
            result
        );
    }
}
