/*
    Immediate

    In immediate addressing, the operand is contained
    in the second byte of the instruction, with no
    further memory addressing required.

    Bytes: 2
*/

use crate::cpu::CPU;

use super::AddrModeResult;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn imm(&self, imm: u8) -> AddrModeResult {
        AddrModeResult {
            data: Some(imm),
            cycles: 0,
            mode: super::AddrMode::IMM,
            addr: None,
            bytes: 2,
        }
    }
}

#[cfg(test)]
mod imm_tests {
    use crate::cpu::addr::AddrModeResult;

    use super::*;

    #[test]
    fn test_imm_addressing_mode() {
        let cpu = CPU::new();
        let imm = cpu.imm(0x88);

        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 0,
                mode: crate::cpu::addr::AddrMode::IMM,
                addr: None,
                bytes: 2,
            },
            imm
        );
    }
}
