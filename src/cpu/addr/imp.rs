/*
    Implied

    In the implied addressing mode, the address containing the
    operand is implicitly stated in the operation code of the
    instruction.

    Bytes: 1
*/

use crate::{bus::Bus, cpu::NESCPU};

use super::{AddrModeResult, AddrModeType};

impl NESCPU {
    pub(in crate::cpu) fn imp(&mut self, _bus: &dyn Bus) -> AddrModeResult {
        self._imp()
    }

    pub(in crate::cpu) fn _imp(&self) -> AddrModeResult {
        AddrModeResult {
            data: None,
            cycles: 0,
            mode: AddrModeType::Imp,
            addr: None,
            bytes: 1,
            operands: "".to_owned(),
            repr: "".to_owned(),
        }
    }
}

#[cfg(test)]
mod imp_tests {
    use super::*;

    #[test]
    fn test_imp_addressing_mode() {
        let cpu = NESCPU::new();
        let imp = cpu._imp();

        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 0,
                mode: AddrModeType::Imp,
                addr: None,
                bytes: 1,
                operands: "".to_owned(),
                repr: "".to_owned()
            },
            imp
        );
    }
}
