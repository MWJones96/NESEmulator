/*
    Accumulator

    This form of addressing is represented with a one byte
    instruction, implying an operation on the accumulator.

    Bytes: 1
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrModeResult, AddrModeType};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn acc(&mut self, _bus: &dyn CPUBus) -> AddrModeResult {
        self._acc()
    }

    #[inline]
    pub(in crate::cpu) fn _acc(&self) -> AddrModeResult {
        AddrModeResult {
            data: Some(self.a),
            cycles: 0,
            mode: AddrModeType::Acc,
            addr: None,
            bytes: 1,
            operands: "".to_owned(),
            repr: "A".to_owned(),
        }
    }
}

#[cfg(test)]
mod acc_tests {
    use crate::cpu::addr::AddrModeResult;

    use super::*;

    #[test]
    fn test_acc_addressing_mode() {
        let mut cpu = CPU::new();
        cpu.a = 0xcc;

        assert_eq!(
            AddrModeResult {
                data: Some(0xcc),
                cycles: 0,
                mode: AddrModeType::Acc,
                addr: None,
                bytes: 1,
                operands: "".to_owned(),
                repr: "A".to_owned()
            },
            cpu._acc()
        );
    }
}
