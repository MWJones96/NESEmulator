use crate::cpu::CPU;

impl CPU {
    pub(in crate::cpu) fn jam_cycles(&self) -> u8 {
        0
    }
}

#[cfg(test)]
mod jam_tests {
    use super::*;

    #[test]
    fn test_jam_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(0, cpu.jam_cycles());
    }
}
