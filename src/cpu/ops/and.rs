use super::super::CPU;

impl CPU {
    pub fn and_imm(&mut self, imm: u8) -> u8 {
        self.a &= imm;

        self.z = self.a == 0;
        self.n = (self.a & 0x80) > 0;
        
        2
    }

    pub fn and_zp(&mut self, addr: u8, mem: &[u8]) -> u8 {
        1 + self.and_imm(mem[addr as usize])
    }

    pub fn and_zpx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let resolved_addr = addr.wrapping_add(self.x);

        1 + self.and_zp(resolved_addr, mem)
    }

    pub fn and_abs(&mut self, addr: u16, mem: &[u8]) -> u8 {
        2 + self.and_imm(mem[addr as usize])
    }

    pub fn and_absx(&mut self, addr: u16, mem: &[u8]) -> u8 {
        let page_before: u8 = (addr >> 8) as u8;
        let resolved_addr: u16 = addr.wrapping_add(self.x as u16);
        let page_after: u8 = (resolved_addr >> 8) as u8;

        if page_before == page_after { self.and_abs(resolved_addr, mem) } else { 1 + self.and_abs(resolved_addr, mem) }
    }
}

#[cfg(test)]
mod and_imm_tests {
    use super::*;

    #[test]
    fn test_and_imm_correct_cycles() {
        let mut cpu = CPU::new();
        assert_eq!(2, cpu.and_imm(0xff));
    }

    #[test]
    fn test_and_imm() {
        let mut cpu = CPU::new();
        cpu.a = 0b1010_1010_u8;

        cpu.and_imm(0b0101_0101_u8);

        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_imm_all_ones() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.and_imm(0xff);

        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_and_imm_half_ones() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and_imm(0b0000_1111_u8);

        assert_eq!(0xf, cpu.a);
    }

    #[test]
    fn test_and_imm_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and_imm(0b0000_1111_u8);

        assert_eq!(false, cpu.z);

        cpu.and_imm(0b0000_0000_u8);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_imm_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.and_imm(0xff);

        assert_eq!(true, cpu.n)
    }
}

#[cfg(test)]
mod and_zp_tests {
    use super::*;

    #[test]
    fn test_and_zp_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(3, cpu.and_zp(0x0, &[0x00u8]));
    }

    #[test]
    fn test_and_zp_correct_output() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.and_zp(0x0, &[0x7u8]);

        assert_eq!(0x7, cpu.a);
    }

    #[test]
    fn test_and_zp_zero_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.and_zp(0x0, &[0xff]);

        assert_eq!(false, cpu.z);

        cpu.and_zp(0x0, &[0x0]);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_zp_negative_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.and_zp(0x0, &[0xff]);

        assert_eq!(true, cpu.n);

        cpu.and_zp(0x0, &[0x0]);

        assert_eq!(false, cpu.n);
    }
}

#[cfg(test)]
mod and_zpx_tests {
    use super::*;

    #[test]
    fn test_and_zpx_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(4, cpu.and_zpx(0x0, &[0x00u8]));
    }

    #[test]
    fn test_and_zpx() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;
        cpu.a = 0b1010_1010;

        cpu.and_zpx(0xff, &[0b0101_0101u8]);
        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_zpx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;
        cpu.a = 0xff;

        cpu.and_zpx(0xff, &[0xff]);
        assert_eq!(true, cpu.n);
        cpu.and_zpx(0xff, &[0x00]);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_and_zpx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;
        cpu.a = 0xff;

        cpu.and_zpx(0xff, &[0xff]);
        assert_eq!(false, cpu.z);
        cpu.and_zpx(0xff, &[0x00]);
        assert_eq!(true, cpu.z);
    }
}

#[cfg(test)]
mod and_abs_tests {
    use super::*;

    #[test]
    fn test_and_abs_correct_cycles() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        assert_eq!(4, cpu.and_abs(0x0, &mem));
    }

    #[test]
    fn test_and_abs() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0b1010_1010;
        mem[0x1111] = 0b0101_0101;
        cpu.and_abs(0x1111, &mem);

        assert_eq!(0x0, cpu.a)
    }

    #[test]
    fn test_and_abs_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff_u8;
        mem[0x1111] = 0xff_u8;
        cpu.and_abs(0x1111, &mem);

        assert_eq!(false, cpu.z);

        mem[0x1111] = 0x00_u8;
        cpu.and_abs(0x1111, &mem);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_abs_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff_u8;
        mem[0x1111] = 0xff_u8;
        cpu.and_abs(0x1111, &mem);
        assert_eq!(true, cpu.n);

        mem[0x1111] = 0x00_u8;
        cpu.and_abs(0x1111, &mem);
        assert_eq!(false, cpu.n);
    }
}

#[cfg(test)]
mod and_absx_tests {
    use super::*;

    #[test]
    fn test_and_absx_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        assert_eq!(4, cpu.and_absx(0x0, &mem));
    }

    #[test]
    fn test_and_absx_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0xff;

        assert_eq!(5, cpu.and_absx(0x1111, &mem));
    }

    #[test]
    fn test_and_absx() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x11;
        mem[0x0] = 0xef;

        cpu.x = 0x1;
        cpu.and_absx(0xffff, &mem);

        assert_eq!(0x1, cpu.a);
    }

    #[test]
    fn test_and_absx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x11;
        mem[0x0] = 0xef;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(false, cpu.z);

        cpu.a = 0x11;
        mem[0x0] = 0xee;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_absx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff;
        mem[0x0] = 0x80;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(true, cpu.n);

        cpu.a = 0xff;
        mem[0x0] = 0x00;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(false, cpu.n);
    }
}
