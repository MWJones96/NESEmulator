/*
    XAA - Non-deterministic Operation of Accumulator, Index Register X,
    Memory and Bus Contents
    Operation: (A ∨ V) ∧ X ∧ M → A

    The operation of the undocumented XAA instruction depends on the
    individual microprocessor. On most machines, it performs a bit-by-bit
    AND operation of the following three operands: The first two are the
    index register X and memory.

    The third operand is the result of a bit-by-bit AND operation of the
    accumulator and a magic component. This magic component depends on the
    individual microprocessor and is usually one of $00, $EE, $EF, $FE and $FF,
    and may be influenced by the RDY pin, leftover contents of the data bus,
    the temperature of the microprocessor, the supplied voltage, and other factors.

    On some machines, additional bits of the result may be set or reset depending
    on non-deterministic factors.

    It then transfers the result to the accumulator.

    XAA does not affect the C or V flags; sets Z if the value loaded was zero,
    otherwise resets it; sets N if the result in bit 7 is a 1; otherwise N is
    reset.
*/

use rand::seq::SliceRandom;

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn xaac(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn xaa(&mut self, mode: &AddrModeResult, _bus: &mut dyn Bus) {
        let magic_constant = *vec![0x00, 0xee, 0xef, 0xfe, 0xff]
            .choose(&mut rand::thread_rng())
            .unwrap() as u8;
        self.a = (self.a | magic_constant) & self.x & mode.data.unwrap();

        self.n = (self.a & 0x80) != 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod xaa_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_xaa_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.xaac(&cpu._imm(0x00)));
    }

    #[test]
    fn test_xaa() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.x = 0xff;
        cpu.xaa(&cpu._imm(0xff), &mut bus);

        assert_eq!(true, vec![0x00, 0xee, 0xef, 0xfe, 0xff].contains(&cpu.a));
    }
}
