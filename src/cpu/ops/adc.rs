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
    negative flag is reset. 
    
    The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.
*/

use super::super::CPU;

impl CPU {
    pub fn adc_imm(&mut self, imm: u8) -> u8 {
        let sign_before: u8 = self.a & 0x80;

        let calc: u16 = self.a as u16 + imm as u16 + (if self.c { 1 } else { 0 });
        self.a = calc as u8;

        let sign_after: u8 = self.a & 0x80;

        self.c = calc > (u8::MAX as u16);
        self.z = self.a == 0_u8;
        self.n = (self.a & 0b_1000_0000_u8) > 0;
        self.v = sign_before != sign_after;

        2
    }

    pub fn adc_zp(&mut self, addr: u8, mem: &[u8]) -> u8 {
        1 + self.adc_imm(mem[addr as usize])
    }

    pub fn adc_zpx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        1 + self.adc_zp(addr.wrapping_add(self.x), mem)
    }

    pub fn adc_abs(&mut self, addr: u16, mem: &[u8]) -> u8 {
        2 + self.adc_imm(mem[addr as usize])
    }

    pub fn adc_absx(&mut self, addr: u16, mem: &[u8]) -> u8 {
        self._adc_absxy_helper(addr, mem, self.x)
    }

    pub fn adc_absy(&mut self, addr: u16, mem: &[u8]) -> u8 {
        self._adc_absxy_helper(addr, mem, self.y)
    }

    pub fn adc_indx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let low_byte_addr: u8 = addr.wrapping_add(self.x);
        let high_byte_addr: u8 = low_byte_addr.wrapping_add(1);

        let resolved_addr: u16 = ((mem[high_byte_addr as usize] as u16) << 8) 
            | (mem[low_byte_addr as usize] as u16);

        2 + self.adc_abs(resolved_addr, mem)
    }

    pub fn adc_indy(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let low_byte_addr: u8 = mem[addr as usize];
        let high_byte_addr: u8 = mem[addr.wrapping_add(1) as usize];

        let page_before: u8 = high_byte_addr;

        let resolved_addr: u16 = ((high_byte_addr as u16) << 8) | low_byte_addr as u16;
        let resolved_addr: u16 = resolved_addr.wrapping_add(self.y as u16);

        let page_after: u8 = (resolved_addr >> 8) as u8;

        if page_before == page_after { 
            1 + self.adc_abs(resolved_addr, mem) 
        } else { 
            2 + self.adc_abs(resolved_addr, mem) 
        }
    }

    fn _adc_absxy_helper(&mut self, addr: u16, mem: &[u8], register: u8) -> u8 {
        let page_before: u8 = (addr >> 8) as u8;

        let new_addr = addr.wrapping_add(register as u16);

        let page_after: u8 = (new_addr >> 8) as u8;
        
        if page_before == page_after { 
            self.adc_abs(new_addr, mem)
        } 
        else {
            1 + self.adc_abs(new_addr, mem)
        }
    }
}

#[cfg(test)]
mod adc_imm_tests {
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
    fn test_adc_imm_with_carry_bit() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0x80_u8);
        cpu.adc_imm(0x80_u8);

        //Carry should be 1
        cpu.adc_imm(0x80_u8);

        assert_eq!(0x81, cpu.a);
        assert_eq!(false, cpu.c);
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

#[cfg(test)]
mod adc_zp_tests {
    use super::*;

    #[test]
    fn test_adc_zp_number_of_cycles() {
        let mut cpu = CPU::new();

        let cycles: u8 = cpu.adc_zp(0x00_u8, &[0x00u8]);
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_adc_zp() {
        let mut cpu = CPU::new();
        cpu.adc_zp(0x00_u8, &[0x81_u8]);

        assert_eq!(0x81_u8, cpu.a);
    }

    #[test]
    fn test_adc_zp_carry_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zp(0x00_u8, &[0x80_u8]);

        assert_eq!(false, cpu.c);

        cpu.adc_zp(0x00_u8, &[0x81_u8]);

        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_adc_zp_zero_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zp(0x00_u8, &[0x80_u8]);
        cpu.adc_zp(0x00_u8, &[0x80_u8]);

        assert_eq!(true, cpu.z);

        cpu.adc_zp(0x00_u8, &[0x01_u8]);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_zp_negative_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zp(0x00_u8, &[0b_1000_0000_u8]);

        assert_eq!(true, cpu.n);

        cpu.adc_zp(0x00_u8, &[0b_1000_0000_u8]);

        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_zp_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zp(0x00_u8, &[0b_1000_0000_u8]);

        assert_eq!(true, cpu.v);

        cpu.adc_zp(0x00_u8, &[0b_0000_0001_u8]);

        assert_eq!(false, cpu.v);

        cpu.adc_zp(0x00_u8, &[0b_1000_0000_u8]);

        assert_eq!(true, cpu.v);
    }

    #[test]
    fn test_adc_zp_different_mem_address() {
        let mut cpu = CPU::new();
        cpu.adc_zp(0x01_u8, &[0b_1111_1111_u8, 0b_1010_1010_u8]);

        assert_eq!(0b_1010_1010_u8, cpu.a);
    }
}

#[cfg(test)]
mod adc_zpx_tests {
    use super::*;

    #[test]
    fn test_adc_zpx_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(4, cpu.adc_zpx(0x00_u8, &[0x00_u8]));
    }

    #[test]
    fn test_adc_zpx_with_x_set_to_zero() {
        let mut cpu = CPU::new();
        cpu.x = 0;

        cpu.adc_zpx(0x00_u8, &[0x77_u8]);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_zpx_with_x_overflow() {
        let mut cpu = CPU::new();
        cpu.x = 0xff_u8;

        cpu.adc_zpx(0x01_u8, &[0x77_u8]);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_zpx_with_carry_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);

        assert_eq!(false, cpu.c);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);

        assert_eq!(true, cpu.c);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_zpx_with_zero_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x00_u8]);
        assert_eq!(true, cpu.z);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(false, cpu.z);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_adc_zpx_with_negative_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(true, cpu.n);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_zpx_with_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(true, cpu.v);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(true, cpu.v);

        cpu.adc_zpx(0x00_u8, &[0x01_u8]);
        assert_eq!(false, cpu.v);
    }

    #[test]
    fn test_adc_zpx_with_different_memory_address() {
        let mut cpu = CPU::new();
        cpu.x = 1;

        cpu.adc_zpx(0x00_u8, &[0x00_u8, 0x10_u8]);

        assert_eq!(0x10_u8, cpu.a);
    }
}

#[cfg(test)]
mod adc_abs_tests {
    use super::*;

    #[test]
    fn test_adc_abs_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(4, cpu.adc_abs(0x0000_u16, &[0x00_u8]));
    }

    #[test]
    fn test_adc_abs_fetch_mem_addr() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0xffff] = 0x77_u8;

        cpu.adc_abs(0xffff_u16, &mem);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_abs_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(0x80_u8, cpu.a);
        assert_eq!(false, cpu.c);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(0x00_u8, cpu.a);
        assert_eq!(true, cpu.c);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(0x81_u8, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_abs_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.z);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_abs_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.n);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_abs_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;
        mem[0x1] = 0x01_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.v);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.v);

        cpu.adc_abs(0x1, &mem);

        assert_eq!(false, cpu.v);
    }
}

#[cfg(test)]
mod adc_absx_tests {
    use super::*;

    #[test]
    fn test_adc_absx_no_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff_u8;

        assert_eq!(4, cpu.adc_absx(0x0000_u16, &mem));
    }

    #[test]
    fn test_adc_absx_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff_u8;

        assert_eq!(5, cpu.adc_absx(0x0001_u16, &mem));
    }

    #[test]
    fn test_adc_absx_page_boundary_crossed_at_end_of_memory() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff_u8;

        assert_eq!(5, cpu.adc_absx(0xffff_u16, &mem));
    }

    #[test]
    fn test_adc_absx_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.c);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_absx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_absx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.n);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_absx_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.v);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.v);

        mem[0x1] = 0x01;
        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.v);
    }
}

#[cfg(test)]
mod adc_absy_tests {
    use super::*;

    #[test]
    fn test_adc_absy_no_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.y = 0xff_u8;

        assert_eq!(4, cpu.adc_absy(0x0000_u16, &mem));
    }

    #[test]
    fn test_adc_absy_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.y = 0xff_u8;

        assert_eq!(5, cpu.adc_absy(0x0001_u16, &mem));
    }

    #[test]
    fn test_adc_absy_page_boundary_crossed_at_end_of_memory() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.y = 0xff_u8;

        assert_eq!(5, cpu.adc_absy(0xffff_u16, &mem));
    }

    #[test]
    fn test_adc_absy_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.c);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_absy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_absy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.n);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_absy_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.v);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.v);

        mem[0x1] = 0x01;
        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.v);
    }
}

#[cfg(test)]
mod adc_indx_tests {
    use super::*;

    #[test]
    fn test_adc_indx_correct_cycles() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(6, cpu.adc_indx(0x00, &mem));
    }

    #[test]
    fn test_adc_indx() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        //Lower byte
        mem[0x00] = 0x40;
        //Upper byte
        mem[0x01] = 0x20;

        mem[0x2040] = 0x77;

        cpu.adc_indx(0x00, &mem);
        assert_eq!(0x77, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0xff] = 0x77;
        mem[0x00] = 0x88;

        mem[0x8877] = 0x11;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(0x11, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap_with_x() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x22;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(0x22, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap_with_x_twice() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x22;

        cpu.adc_indx(0xff, &mem);
        cpu.adc_indx(0xff, &mem);

        assert_eq!(0x44, cpu.a);
    }

    #[test]
    fn test_adc_indx_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(false, cpu.c);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.c);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_indx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(false, cpu.z);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_indx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(true, cpu.n);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.n);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_indx_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(true, cpu.v);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.v);

        mem[0x8877] = 0x01;

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.v);
    }
}

#[cfg(test)]
mod adc_indy_tests {
    use super::*;

    #[test]
    fn test_adc_indy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(5, cpu.adc_indy(0x00, &mem));
    }

    #[test]
    fn test_adc_indy_correct_cycles_page_cross() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x80] = 0x11;
        mem[0x81] = 0x15;

        cpu.y = 0xff;

        assert_eq!(6, cpu.adc_indy(0x80, &mem));
    }

    #[test]
    fn test_adc_indy_end_of_zero_page() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.y = 0x2;

        mem[0x0] = 0xff;
        mem[0xff] = 0xff;

        mem[0x1] = 0xee;

        cpu.adc_indy(0xff, &mem);

        assert_eq!(0xee, cpu.a);
    }

    #[test]
    fn test_adc_indy_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.c);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.c);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_indy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.z);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.z);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_indy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.n);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.n);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_indy_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.v);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.v);

        mem[0x1010] = 0x01;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.v);
    }
}