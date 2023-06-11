/*
    Immediate

    In immediate addressing, the operand is contained
    in the second byte of the instruction, with no
    further memory addressing required.

    Bytes: 2
*/

use crate::{bus::Bus, cpu::NESCPU};

use super::{AddrModeResult, AddrModeType};

impl NESCPU {
    pub(in crate::cpu) fn imm(&mut self, bus: &dyn Bus) -> AddrModeResult {
        let data = self.fetch_byte(bus);
        self._imm(data)
    }

    pub(in crate::cpu) fn _imm(&self, imm: u8) -> AddrModeResult {
        AddrModeResult {
            data: Some(imm),
            cycles: 0,
            mode: AddrModeType::Imm,
            addr: None,
            bytes: 2,
            operands: format!("{:02X}", imm),
            repr: format!("#${:02X}", imm),
        }
    }
}

#[cfg(test)]
mod imm_tests {
    use crate::cpu::addr::{AddrModeResult, AddrModeType};

    use super::*;

    #[test]
    fn test_imm_addressing_mode() {
        let cpu = NESCPU::new();
        let imm = cpu._imm(0x88);

        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 0,
                mode: AddrModeType::Imm,
                addr: None,
                bytes: 2,
                operands: "88".to_owned(),
                repr: "#$88".to_owned()
            },
            imm
        );
    }
}
