use self::{addr::AddrModeResult, bus::CPUBus};

pub mod bus;

mod addr;
mod ops;

#[derive(PartialEq, Debug)]
enum InstructionType {
    Reset,
    NMI,
    IRQ,
    Instruction {
        opcode: u8,
        addressing_mode: AddrModeResult,
    },
}

#[derive(PartialEq, Debug)]
struct CurrentInstruction {
    remaining_cycles: u8,
    instruction_type: InstructionType,
}

pub struct CPU {
    pc: u16,
    sp: u8,

    a: u8,
    x: u8,
    y: u8,

    n: bool, //Bit 7
    v: bool, //Bit 6
    //Bit 5 (unused)
    //Bit 4 (only used for BRK)
    d: bool, //Bit 3
    i: bool, //Bit 2
    z: bool, //Bit 1
    c: bool, //Bit 0

    current_instruction: CurrentInstruction,

    pending_nmi: bool,
    pending_irq: bool,
}

impl CPU {
    const NMI_VECTOR: u16 = 0xfffa;
    const RESET_VECTOR: u16 = 0xfffc;
    const IRQ_VECTOR: u16 = 0xfffe;

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
            //Bit 4 (only used for BRK)
            d: false, //Bit 3
            i: false, //Bit 2
            z: false, //Bit 1
            c: false, //Bit 0

            current_instruction: CurrentInstruction {
                remaining_cycles: 8,
                instruction_type: InstructionType::Reset,
            },

            pending_nmi: false,
            pending_irq: false,
        }
    }

    pub fn system_nmi(&mut self) {
        //Edge-detected
        self.pending_nmi = true;
    }

    pub fn system_irq(&mut self, interrupt: bool) {
        //Level-detected
    }

    pub fn system_reset(&mut self) {
        self.current_instruction = CurrentInstruction {
            remaining_cycles: self.reset_cycles(),
            instruction_type: InstructionType::Reset,
        }
    }

    pub fn clock(&mut self, bus: &mut dyn CPUBus) {
        self.current_instruction.remaining_cycles -= 1;

        if self.current_instruction.remaining_cycles == 0 {
            match self.current_instruction.instruction_type {
                InstructionType::Reset => self.reset(bus),
                InstructionType::NMI => self.nmi(bus),
                InstructionType::IRQ => {}
                InstructionType::Instruction {
                    opcode,
                    addressing_mode,
                } => self.execute(opcode, &addressing_mode, bus),
            }

            if self.pending_nmi {
                self.current_instruction = CurrentInstruction {
                    remaining_cycles: self.nmi_cycles(),
                    instruction_type: InstructionType::NMI,
                }
            } else {
                let opcode = self.fetch_byte(bus);
                if opcode == 0x0 {
                    self.fetch_byte(bus); //Discard next byte for BRK
                }
                let addressing_mode = self.fetch_addr_mode(opcode, bus);

                self.current_instruction = CurrentInstruction {
                    remaining_cycles: self.get_number_of_cycles(opcode, &addressing_mode),
                    instruction_type: InstructionType::Instruction {
                        opcode,
                        addressing_mode,
                    },
                }
            }
        }
    }
}

impl CPU {
    fn get_status_byte(&self, brk: bool) -> u8 {
        (self.n as u8) << 7
            | (self.v as u8) << 6
            | 0x1 << 5
            | (brk as u8) << 4
            | (self.d as u8) << 3
            | (self.i as u8) << 2
            | (self.z as u8) << 1
            | (self.c as u8) << 0
    }

    fn fetch_byte(&mut self, bus: &dyn CPUBus) -> u8 {
        let data = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        data
    }

    fn fetch_two_bytes_as_u16(&mut self, bus: &dyn CPUBus) -> u16 {
        let low_byte: u16 = bus.read(self.pc.wrapping_add(0)) as u16;
        let high_byte: u16 = bus.read(self.pc.wrapping_add(1)) as u16;
        self.pc = self.pc.wrapping_add(2);

        high_byte << 8 | low_byte
    }
}

impl CPU {
    fn fetch_addr_mode(&mut self, opcode: u8, bus: &dyn CPUBus) -> AddrModeResult {
        match opcode {
            0x00 | 0x18 | 0xD8 | 0x58 | 0xB8 | 0xCA | 0x88 | 0xE8 | 0xC8 | 0xEA | 0x48 | 0x08
            | 0x68 | 0x28 | 0x40 | 0x60 | 0x38 | 0xF8 | 0x78 | 0xAA | 0xA8 | 0xBA | 0x8A | 0x9A
            | 0x98 => self.imp(),
            0x0A | 0x4A | 0x2A | 0x6A => self.acc(),
            0x69 | 0x29 | 0xC9 | 0xE0 | 0xC0 | 0x49 | 0xA9 | 0xA2 | 0xA0 | 0x09 | 0xE9 => {
                let byte = self.fetch_byte(bus);
                self.imm(byte)
            }
            0x6D | 0x2D | 0x0E | 0x2C | 0xCD | 0xEC | 0xCC | 0xCE | 0x4D | 0xEE | 0x4C | 0x20
            | 0xAD | 0xAE | 0xAC | 0x4E | 0x0D | 0x2E | 0x6E | 0xED | 0x8D | 0x8E | 0x8C => {
                let abs_addr = self.fetch_two_bytes_as_u16(bus);
                self.abs(abs_addr, bus)
            }
            0x7D | 0x3D | 0x1E | 0xDD | 0xDE | 0x5D | 0xFE | 0xBD | 0xBC | 0x5E | 0x1D | 0x3E
            | 0x7E | 0xFD | 0x9D => {
                let abs_addr = self.fetch_two_bytes_as_u16(bus);
                self.absx(abs_addr, bus)
            }
            0x79 | 0x39 | 0xD9 | 0x59 | 0xB9 | 0xBE | 0x19 | 0xF9 | 0x99 => {
                let abs_addr = self.fetch_two_bytes_as_u16(bus);
                self.absy(abs_addr, bus)
            }
            0x61 | 0x21 | 0xC1 | 0x41 | 0xA1 | 0x01 | 0xE1 | 0x81 => {
                let addr = self.fetch_byte(bus);
                self.indx(addr, bus)
            }
            0x6C => {
                let abs_addr = self.fetch_two_bytes_as_u16(bus);
                self.ind(abs_addr, bus)
            }
            0x65 | 0x25 | 0x06 | 0x24 | 0xC5 | 0xE4 | 0xC4 | 0xC6 | 0x45 | 0xE6 | 0xA5 | 0xA6
            | 0xA4 | 0x46 | 0x05 | 0x26 | 0x66 | 0xE5 | 0x85 | 0x86 | 0x84 => {
                let addr = self.fetch_byte(bus);
                self.zp(addr, bus)
            }
            0x71 | 0x31 | 0xD1 | 0x51 | 0xB1 | 0x11 | 0xF1 | 0x91 => {
                let addr = self.fetch_byte(bus);
                self.indy(addr, bus)
            }
            0x75 | 0x35 | 0x16 | 0xD5 | 0xD6 | 0x55 | 0xF6 | 0xB5 | 0xB4 | 0x56 | 0x15 | 0x36
            | 0x76 | 0xF5 | 0x95 | 0x94 => {
                let addr = self.fetch_byte(bus);
                self.zpx(addr, bus)
            }
            0xB6 | 0x96 => {
                let addr = self.fetch_byte(bus);
                self.zpy(addr, bus)
            }
            0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 => {
                let offset = self.fetch_byte(bus);
                self.rel(offset)
            }
            _ => panic!("Opcode {:#02x} is not implemented", opcode),
        }
    }

    fn get_number_of_cycles(&self, opcode: u8, mode: &AddrModeResult) -> u8 {
        match opcode {
            0x00 => self.brk_cycles(mode),
            0x08 => self.php_cycles(mode),
            0x09 | 0x0D | 0x1D | 0x19 | 0x05 | 0x15 | 0x01 | 0x11 => self.ora_cycles(mode),
            0x0A | 0x0E | 0x1E | 0x06 | 0x16 => self.asl_cycles(mode),
            0x10 => self.bpl_cycles(mode),
            0x18 => self.clc_cycles(mode),
            0x20 => self.jsr_cycles(mode),
            0x28 => self.plp_cycles(mode),
            0x29 | 0x2D | 0x3D | 0x39 | 0x25 | 0x35 | 0x21 | 0x31 => self.and_cycles(mode),
            0x2A | 0x2E | 0x3E | 0x26 | 0x36 => self.rol_cycles(mode),
            0x2C | 0x24 => self.bit_cycles(mode),
            0x30 => self.bmi_cycles(mode),
            0x38 => self.sec_cycles(mode),
            0x40 => self.rti_cycles(mode),
            0x48 => self.pha_cycles(mode),
            0x49 | 0x4D | 0x5D | 0x59 | 0x45 | 0x55 | 0x41 | 0x51 => self.eor_cycles(mode),
            0x4A | 0x4E | 0x5E | 0x46 | 0x56 => self.lsr_cycles(mode),
            0x4C | 0x6C => self.jmp_cycles(mode),
            0x50 => self.bvc_cycles(mode),
            0x58 => self.cli_cycles(mode),
            0x60 => self.rts_cycles(mode),
            0x68 => self.pla_cycles(mode),
            0x69 | 0x6D | 0x7D | 0x79 | 0x65 | 0x75 | 0x61 | 0x71 => self.adc_cycles(mode),
            0x6A | 0x6E | 0x7E | 0x66 | 0x76 => self.ror_cycles(mode),
            0x70 => self.bvs_cycles(mode),
            0x78 => self.sei_cycles(mode),
            0x88 => self.dey_cycles(mode),
            0x8C | 0x84 | 0x94 => self.sty_cycles(mode),
            0x8A => self.txa_cycles(mode),
            0x8D | 0x9D | 0x99 | 0x85 | 0x95 | 0x81 | 0x91 => self.sta_cycles(mode),
            0x8E | 0x86 | 0x96 => self.stx_cycles(mode),
            0x90 => self.bcc_cycles(mode),
            0x98 => self.tya_cycles(mode),
            0x9A => self.txs_cycles(mode),
            0xA2 | 0xAE | 0xBE | 0xA6 | 0xB6 => self.ldx_cycles(mode),
            0xA8 => self.tay_cycles(mode),
            0xA9 | 0xAD | 0xBD | 0xB9 | 0xA5 | 0xB5 | 0xA1 | 0xB1 => self.lda_cycles(mode),
            0xA0 | 0xAC | 0xBC | 0xA4 | 0xB4 => self.ldy_cycles(mode),
            0xAA => self.tax_cycles(mode),
            0xB0 => self.bcs_cycles(mode),
            0xB8 => self.clv_cycles(mode),
            0xBA => self.tsx_cycles(mode),
            0xC0 | 0xCC | 0xC4 => self.cpy_cycles(mode),
            0xC8 => self.iny_cycles(mode),
            0xC9 | 0xCD | 0xDD | 0xD9 | 0xC5 | 0xD5 | 0xC1 | 0xD1 => self.cmp_cycles(mode),
            0xCA => self.dex_cycles(mode),
            0xCE | 0xDE | 0xC6 | 0xD6 => self.dec_cycles(mode),
            0xD0 => self.bne_cycles(mode),
            0xD8 => self.cld_cycles(mode),
            0xE0 | 0xEC | 0xE4 => self.cpx_cycles(mode),
            0xE8 => self.inx_cycles(mode),
            0xE9 | 0xED | 0xFD | 0xF9 | 0xE5 | 0xF5 | 0xE1 | 0xF1 => self.sbc_cycles(mode),
            0xEA => self.nop_cycles(mode),
            0xEE | 0xFE | 0xE6 | 0xF6 => self.inc_cycles(mode),
            0xF0 => self.beq_cycles(mode),
            0xF8 => self.sed_cycles(mode),
            _ => panic!("Opcode {:#02x} is not implemented", opcode),
        }
    }

    fn execute(&mut self, opcode: u8, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
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
}

#[cfg(test)]
mod cpu_tests {
    use mockall::predicate::eq;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::{bus::MockCPUBus, *};

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
        assert_eq!(false, cpu.d); //Bit 3
        assert_eq!(false, cpu.i); //Bit 2
        assert_eq!(false, cpu.z); //Bit 1
        assert_eq!(false, cpu.c); //Bit 0

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 8,
                instruction_type: InstructionType::Reset
            },
            cpu.current_instruction
        );

        assert_eq!(false, cpu.pending_nmi);
        assert_eq!(false, cpu.pending_irq);
    }

    #[test]
    fn test_get_status_byte_no_flags() {
        let cpu = CPU::new();
        assert_eq!(0b0010_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_negative_flag() {
        let mut cpu = CPU::new();
        cpu.n = true;
        assert_eq!(0b1010_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.v = true;
        assert_eq!(0b0110_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_break_flag() {
        let mut cpu = CPU::new();
        assert_eq!(0b0011_0000, cpu.get_status_byte(true))
    }

    #[test]
    fn test_get_status_byte_decimal_flag() {
        let mut cpu = CPU::new();
        cpu.d = true;
        assert_eq!(0b0010_1000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_interrupt_flag() {
        let mut cpu = CPU::new();
        cpu.i = true;
        assert_eq!(0b0010_0100, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_zero_flag() {
        let mut cpu = CPU::new();
        cpu.z = true;
        assert_eq!(0b0010_0010, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_carry_flag() {
        let mut cpu = CPU::new();
        cpu.c = true;
        assert_eq!(0b0010_0001, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_all_flags() {
        let mut cpu = CPU::new();
        cpu.n = true;
        cpu.v = true;
        cpu.d = true;
        cpu.i = true;
        cpu.z = true;
        cpu.c = true;

        assert_eq!(0b1111_1111, cpu.get_status_byte(true))
    }

    #[test]
    fn test_fetch_next_byte() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.pc = 0xffff;
        bus.expect_read()
            .with(eq(0xffff))
            .times(1)
            .return_const(0xcc);

        let byte: u8 = cpu.fetch_byte(&bus);
        assert_eq!(0xcc, byte);
        assert_eq!(0x0, cpu.pc);
    }

    #[test]
    fn test_fetch_next_two_bytes_as_u16() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.pc = 0xffff;
        bus.expect_read()
            .with(eq(0xffff))
            .times(1)
            .return_const(0x40);
        bus.expect_read().with(eq(0x0)).times(1).return_const(0x20);

        let two_bytes = cpu.fetch_two_bytes_as_u16(&bus);
        assert_eq!(0x2040, two_bytes);
        assert_eq!(0x1, cpu.pc);
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR))
            .once()
            .return_const(0x40);

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR + 1))
            .once()
            .return_const(0x20);

        bus.expect_read().with(eq(0x2040)).once().return_const(0x69);

        bus.expect_read().with(eq(0x2041)).once().return_const(0xff);

        for _ in 0..8 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 2,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x69,
                    addressing_mode: AddrModeResult {
                        addr: None,
                        data: Some(0xff),
                        cycles: 0,
                        mode: addr::AddrMode::IMM
                    }
                }
            },
            cpu.current_instruction
        );

        assert_eq!(0x2042, cpu.pc);
        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_cpu_system_reset() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        cpu.current_instruction = CurrentInstruction {
            remaining_cycles: 1,
            instruction_type: InstructionType::Instruction {
                opcode: 0x0,
                addressing_mode: cpu.imp(),
            },
        };

        cpu.system_reset();

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 8,
                instruction_type: InstructionType::Reset
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_brk_instruction_fetches_extra_byte() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR))
            .once()
            .return_const(0x40);

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR + 1))
            .once()
            .return_const(0x20);

        bus.expect_read().with(eq(0x2040)).once().return_const(0x00);

        bus.expect_read().with(eq(0x2041)).once().return_const(0x00);

        for _ in 0..8 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x00,
                    addressing_mode: cpu.imp()
                }
            },
            cpu.current_instruction
        );

        assert_eq!(0x2042, cpu.pc);
    }

    #[test]
    fn test_nmi_request_triggerred() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        cpu.system_nmi();

        assert_eq!(true, cpu.pending_nmi);

        for _ in 0..8 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 8,
                instruction_type: InstructionType::NMI
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_nmi_request_ignored_on_nmi_startup() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        cpu.system_nmi();

        assert_eq!(true, cpu.pending_nmi);

        for _ in 0..8 {
            cpu.clock(&mut bus);
        }

        cpu.system_nmi();

        for _ in 0..8 {
            cpu.clock(&mut bus);
        }

        assert_eq!(false, cpu.pending_nmi);

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x0,
                    addressing_mode: cpu.imp()
                }
            },
            cpu.current_instruction
        );
    }
}
