use self::{addr::AddrModeResult, bus::Bus};

mod addr;
mod bus;
mod ops;

pub struct CPU {
    pc: u16,
    sp: u8,

    a: u8,
    x: u8,
    y: u8,

    n: bool, //Bit 7
    v: bool, //Bit 6
    //Bit 5 (unused)
    b: bool, //Bit 4
    d: bool, //Bit 3
    i: bool, //Bit 2
    z: bool, //Bit 1
    c: bool, //Bit 0
}

impl CPU {
    const INTERRUPT_VECTOR: u16 = 0xfffe;

    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0xff,

            a: 0,
            x: 0,
            y: 0,

            n: false, //Bit 7
            v: false, //Bit 6
            //Bit 5 unused (always 1)
            b: false, //Bit 4
            d: false, //Bit 3
            i: false, //Bit 2
            z: false, //Bit 1
            c: false, //Bit 0
        }
    }

    fn execute(&mut self, opcode: u8, mode: &AddrModeResult, bus: &dyn Bus) {
        match opcode {
            0x00 => self.brk(mode, bus),
            0x08 => self.php(mode, bus),
            0x09 | 0x0D | 0x1D | 0x19 | 0x05 | 0x15 | 0x01 | 0x11 => self.ora(mode),
            0x0A | 0x0E | 0x1E | 0x06 | 0x16 => self.asl(mode, bus),
            0x10 => self.bpl(mode),
            0x18 => self.clc(mode),
            0x20 => self.jsr(mode, bus),
            0x28 => self.plp(mode, bus),
            0x29 | 0x2D | 0x3D | 0x39 | 0x25 | 0x35 | 0x21 | 0x31 => self.and(mode),
            0x2A | 0x2E | 0x3E | 0x26 | 0x36 => self.rol(mode, bus),
            0x2C | 0x24 => self.bit(mode),
            0x30 => self.bmi(mode),
            0x38 => self.sec(mode),
            0x40 => self.rti(mode, bus),
            0x48 => self.pha(mode, bus),
            0x49 | 0x4D | 0x5D | 0x59 | 0x45 | 0x55 | 0x41 | 0x51 => self.eor(mode),
            0x4A | 0x4E | 0x5E | 0x46 | 0x56 => self.lsr(mode, bus),
            0x4C | 0x6C => self.jmp(mode),
            0x50 => self.bvc(mode),
            0x58 => self.cli(mode),
            0x60 => self.rts(mode, bus),
            0x68 => self.pla(mode, bus),
            0x69 | 0x6D | 0x7D | 0x79 | 0x65 | 0x75 | 0x61 | 0x71 => self.adc(mode),
            0x6A | 0x6E | 0x7E | 0x66 | 0x76 => self.ror(mode, bus),
            0x70 => self.bvs(mode),
            0x78 => self.sei(mode),
            0x88 => self.dey(mode),
            0x8C | 0x84 | 0x94 => self.sty(mode, bus),
            0x8A => self.txa(mode),
            0x8D | 0x9D | 0x99 | 0x85 | 0x95 | 0x81 | 0x91 => self.sta(mode, bus),
            0x8E | 0x86 | 0x96 => self.stx(mode, bus),
            0x90 => self.bcc(mode),
            0x98 => self.tya(mode),
            0x9A => self.txs(mode),
            0xA2 | 0xAE | 0xBE | 0xA6 | 0xB6 => self.ldx(mode),
            0xA8 => self.tay(mode),
            0xA9 | 0xAD | 0xBD | 0xB9 | 0xA5 | 0xB5 | 0xA1 | 0xB1 => self.lda(mode),
            0xA0 | 0xAC | 0xBC | 0xA4 | 0xB4 => self.ldy(mode),
            0xAA => self.tax(mode),
            0xB0 => self.bcs(mode),
            0xB8 => self.clv(mode),
            0xBA => self.tsx(mode),
            0xC0 | 0xCC | 0xC4 => self.cpy(mode),
            0xC8 => self.iny(mode),
            0xC9 | 0xCD | 0xDD | 0xD9 | 0xC5 | 0xD5 | 0xC1 | 0xD1 => self.cmp(mode),
            0xCA => self.dex(mode),
            0xCE | 0xDE | 0xC6 | 0xD6 => self.dec(mode, bus),
            0xD0 => self.bne(mode),
            0xD8 => self.cld(mode),
            0xE0 | 0xEC | 0xE4 => self.cpx(mode),
            0xE8 => self.inx(mode),
            0xE9 | 0xED | 0xFD | 0xF9 | 0xE5 | 0xF5 | 0xE1 | 0xF1 => self.sbc(mode),
            0xEA => self.nop(mode),
            0xEE | 0xFE | 0xE6 | 0xF6 => self.inc(mode, bus),
            0xF0 => self.beq(mode),
            0xF8 => self.sed(mode),

            _ => panic!("Opcode {:#02x} is not implemented", opcode),
        }
    }

    fn get_status_byte(&self) -> u8 {
        (self.n as u8) << 7
            | (self.v as u8) << 6
            | 0x1 << 5
            | (self.b as u8) << 4
            | (self.d as u8) << 3
            | (self.i as u8) << 2
            | (self.z as u8) << 1
            | (self.c as u8) << 0
    }
}

#[cfg(test)]
mod cpu_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cpu_initial_state() {
        let cpu = CPU::new();

        assert_eq!(0, cpu.pc);
        assert_eq!(0xff, cpu.sp);

        assert_eq!(0, cpu.a);
        assert_eq!(0, cpu.x);
        assert_eq!(0, cpu.y);

        assert_eq!(false, cpu.n); //Bit 7
        assert_eq!(false, cpu.v); //Bit 6
                                  //Bit 5 (unused)
        assert_eq!(false, cpu.b); //Bit 4
        assert_eq!(false, cpu.d); //Bit 3
        assert_eq!(false, cpu.i); //Bit 2
        assert_eq!(false, cpu.z); //Bit 1
        assert_eq!(false, cpu.c); //Bit 0
    }

    #[test]
    fn test_get_status_byte_no_flags() {
        let cpu = CPU::new();
        assert_eq!(0b0010_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_negative_flag() {
        let mut cpu = CPU::new();
        cpu.n = true;
        assert_eq!(0b1010_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.v = true;
        assert_eq!(0b0110_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_break_flag() {
        let mut cpu = CPU::new();
        cpu.b = true;
        assert_eq!(0b0011_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_decimal_flag() {
        let mut cpu = CPU::new();
        cpu.d = true;
        assert_eq!(0b0010_1000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_interrupt_flag() {
        let mut cpu = CPU::new();
        cpu.i = true;
        assert_eq!(0b0010_0100, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_zero_flag() {
        let mut cpu = CPU::new();
        cpu.z = true;
        assert_eq!(0b0010_0010, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_carry_flag() {
        let mut cpu = CPU::new();
        cpu.c = true;
        assert_eq!(0b0010_0001, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_all_flags() {
        let mut cpu = CPU::new();
        cpu.n = true;
        cpu.v = true;
        cpu.b = true;
        cpu.d = true;
        cpu.i = true;
        cpu.z = true;
        cpu.c = true;

        assert_eq!(0b1111_1111, cpu.get_status_byte())
    }
}

#[cfg(test)]
mod execute_tests {
    use super::{bus::MockBus, *};

    #[test]
    #[should_panic]
    fn test_panic_on_invalid_opcode() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();

        cpu.execute(0xff, &cpu.imp(), &bus);
    }
}
