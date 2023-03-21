/*
    ADC - Add Memory to Accumulator with Carry

    Operation: A + M + C → A, C

    This instruction adds the value of memory and carry from the previous operation to the value
    of the accumulator and stores the result in the accumulator.

    This instruction affects the accumulator; sets the carry flag when the sum of a binary add 
    exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset. 

    The overflow flag is set when the sign or bit 7 is changed due to the result exceeding 
    +127 or -128, otherwise overflow is reset. 

    The negative flag is set if the accumulator result contains bit 7 on, otherwise the 
    negative flag is reset. The zero flag is set if the accumulator result is 0, 
    otherwise the zero flag is reset.
*/

use super::super::CPU;

impl CPU {
    pub fn adc_imm(&mut self, imm: u8) -> u8 {
        let sign_before: u8 = self.a & 0x80;

        let int: u16 = (self.a as u16) + (imm as u16);
        self.a = (int as u8) & u8::MAX;

        let sign_after: u8 = self.a & 0x80;

        self.c = int > (u8::MAX as u16);
        self.z = self.a == 0_u8;
        self.n = (self.a & 0b_1000_0000_u8) > 0;
        self.v = sign_before != sign_after;

        return 2; //Always takes two cycles
    }

    pub fn adc_zp(&self) {

    }

    pub fn adc_zpx(&self) {

    }

    pub fn adc_abs(&self) {

    }

    pub fn adc_absx(&self) {

    }

    pub fn adc_absy(&self) {

    }

    pub fn adc_indx(&self) {

    }

    pub fn adc_indy(&self) {

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_adc_imm_no_carry() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0x01_u8);

        assert_eq!(0x01_u8, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_imm_with_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0x80_u8;

        cpu.adc_imm(0x80_u8);

        assert_eq!(0x00_u8, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_adc_imm_with_carry_zero_flag() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0x00_u8);

        assert_eq!(true, cpu.z);

        cpu.adc_imm(0x80_u8);

        assert_eq!(false, cpu.z);

        cpu.adc_imm(0x80_u8);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_adc_imm_with_negative_flag() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0b_0111_1111_u8);

        assert_eq!(false, cpu.n);

        cpu.adc_imm(0b_0000_0001_u8);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_imm_with_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.adc_imm(0b_1000_0000_u8);

        assert_eq!(true, cpu.v);

        cpu.adc_imm(0b_0000_0001_u8);

        assert_eq!(false, cpu.v);

        cpu.adc_imm(0b_1000_0000_u8);

        assert_eq!(true, cpu.v);
    }

    #[test]
    fn test_adc_imm_get_cycles() {
        let mut cpu = CPU::new();
        let cycles: u8 = cpu.adc_imm(0b_0000_0000_u8);
        assert_eq!(2, cycles);
    }
}