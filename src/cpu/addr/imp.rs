/*
    Implied

    In the implied addressing mode, the address containing the
    operand is implicitly stated in the operation code of the
    instruction.

    Bytes: 1
*/

use crate::cpu::CPU;

use super::{AddrModeResult, AddrModeType};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn imp(&self) -> AddrModeResult {
        AddrModeResult {
            data: None,
            cycles: 0,
            mode: AddrModeType::IMP,
            addr: None,
            bytes: 1,
        }
    }
}

#[cfg(test)]
mod imp_tests {
    use super::*;

    #[test]
    fn test_imp_addressing_mode() {
        let cpu = CPU::new();
        let imp = cpu.imp();

        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 0,
                mode: AddrModeType::IMP,
                addr: None,
                bytes: 1,
            },
            imp
        );
    }
}
