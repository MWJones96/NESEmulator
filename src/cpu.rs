use self::{addr::AddrModeResult, bus::CPUBus};

pub mod bus;

mod addr;
mod ops;

type AddrFn = fn(&mut CPU, &dyn CPUBus) -> AddrModeResult;
type CyclesFn = fn(&CPU, &AddrModeResult) -> u8;
type ExecFn = fn(&mut CPU, &mut dyn CPUBus);

#[derive(PartialEq, Debug, Clone)]
enum InstructionType {
    Jam,
    Reset,
    NMI,
    IRQ,
    Instruction {
        opcode: u8,
        addr_mode: AddrModeResult,
    },
}

#[derive(PartialEq, Debug, Clone)]
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

    #[allow(arithmetic_overflow)]
    elapsed_cycles: u64,
}

impl ToString for CPU {
    fn to_string(&self) -> String {
        match &self.current_instruction.instruction_type {
            InstructionType::Instruction { opcode, addr_mode } => {
                format!(
                    "{:04X}  {:02X} {: <6} {} {: <27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
                    self.pc.wrapping_sub(addr_mode.bytes as u16),
                    *opcode,
                    addr_mode.operands,
                    CPU::LOOKUP_TABLE[*opcode as usize].0,
                    addr_mode.repr,
                    self.a,
                    self.x,
                    self.y,
                    self.get_status_byte(false),
                    self.sp,
                    self.elapsed_cycles
                )
            }
            InstructionType::Jam => "JAM".to_owned(),
            InstructionType::IRQ => "IRQ".to_owned(),
            InstructionType::NMI => "NMI".to_owned(),
            InstructionType::Reset => "RESET".to_owned(),
        }
    }
}

impl CPU {
    const NMI_VECTOR: u16 = 0xfffa;
    const RESET_VECTOR: u16 = 0xfffc;
    const IRQ_VECTOR: u16 = 0xfffe;

    #[rustfmt::skip]
    const LOOKUP_TABLE: [(&str, AddrFn, CyclesFn); 256] = [
        ("BRK", CPU::imm, CPU::brkc), ("ORA", CPU::indx, CPU::orac), ("JAM", CPU::imp, CPU::jamc), ("SLO", CPU::indx, CPU::sloc), ("NOP",  CPU::zp, CPU::nopc), ("ORA",  CPU::zp, CPU::orac), ("ASL",  CPU::zp, CPU::aslc), ("SLO",  CPU::zp, CPU::sloc), ("PHP", CPU::imp, CPU::phpc), ("ORA",  CPU::imm, CPU::orac), ("ASL", CPU::acc, CPU::aslc), ("ANC",  CPU::imm, CPU::ancc), ("NOP",  CPU::abs, CPU::nopc), ("ORA",  CPU::abs, CPU::orac), ("ASL",  CPU::abs, CPU::aslc), ("SLO",  CPU::abs, CPU::sloc),
        ("BPL", CPU::rel, CPU::bplc), ("ORA", CPU::indy, CPU::orac), ("JAM", CPU::imp, CPU::jamc), ("SLO", CPU::indy, CPU::sloc), ("NOP", CPU::zpx, CPU::nopc), ("ORA", CPU::zpx, CPU::orac), ("ASL", CPU::zpx, CPU::aslc), ("SLO", CPU::zpx, CPU::sloc), ("CLC", CPU::imp, CPU::clcc), ("ORA", CPU::absy, CPU::orac), ("NOP", CPU::imp, CPU::nopc), ("SLO", CPU::absy, CPU::sloc), ("NOP", CPU::absx, CPU::nopc), ("ORA", CPU::absx, CPU::orac), ("ASL", CPU::absx, CPU::aslc), ("SLO", CPU::absx, CPU::sloc),
        ("JSR", CPU::abs, CPU::jsrc), ("AND", CPU::indx, CPU::andc), ("JAM", CPU::imp, CPU::jamc), ("RLA", CPU::indx, CPU::rlac), ("BIT",  CPU::zp, CPU::bitc), ("AND",  CPU::zp, CPU::andc), ("ROL",  CPU::zp, CPU::rolc), ("RLA",  CPU::zp, CPU::rlac), ("PLP", CPU::imp, CPU::plpc), ("AND",  CPU::imm, CPU::andc), ("ROL", CPU::acc, CPU::rolc), ("ANC",  CPU::imm, CPU::ancc), ("BIT",  CPU::abs, CPU::bitc), ("AND",  CPU::abs, CPU::andc), ("ROL",  CPU::abs, CPU::rolc), ("RLA",  CPU::abs, CPU::rlac),
        ("BMI", CPU::rel, CPU::bmic), ("AND", CPU::indy, CPU::andc), ("JAM", CPU::imp, CPU::jamc), ("RLA", CPU::indy, CPU::rlac), ("NOP", CPU::zpx, CPU::nopc), ("AND", CPU::zpx, CPU::andc), ("ROL", CPU::zpx, CPU::rolc), ("RLA", CPU::zpx, CPU::rlac), ("SEC", CPU::imp, CPU::secc), ("AND", CPU::absy, CPU::andc), ("NOP", CPU::imp, CPU::nopc), ("RLA", CPU::absy, CPU::rlac), ("NOP", CPU::absx, CPU::nopc), ("AND", CPU::absx, CPU::andc), ("ROL", CPU::absx, CPU::rolc), ("RLA", CPU::absx, CPU::rlac),
        ("RTI", CPU::imp, CPU::rtic), ("EOR", CPU::indx, CPU::eorc), ("JAM", CPU::imp, CPU::jamc), ("SRE", CPU::indx, CPU::srec), ("NOP",  CPU::zp, CPU::nopc), ("EOR",  CPU::zp, CPU::eorc), ("LSR",  CPU::zp, CPU::lsrc), ("SRE",  CPU::zp, CPU::srec), ("PHA", CPU::imp, CPU::phac), ("EOR",  CPU::imm, CPU::eorc), ("LSR", CPU::acc, CPU::lsrc), ("ASR",  CPU::imm, CPU::asrc), ("JMP",  CPU::abs, CPU::jmpc), ("EOR",  CPU::abs, CPU::eorc), ("LSR",  CPU::abs, CPU::lsrc), ("SRE",  CPU::abs, CPU::srec),
        ("BVC", CPU::rel, CPU::bvcc), ("EOR", CPU::indy, CPU::eorc), ("JAM", CPU::imp, CPU::jamc), ("SRE", CPU::indy, CPU::srec), ("NOP", CPU::zpx, CPU::nopc), ("EOR", CPU::zpx, CPU::eorc), ("LSR", CPU::zpx, CPU::lsrc), ("SRE", CPU::zpx, CPU::srec), ("CLI", CPU::imp, CPU::clic), ("EOR", CPU::absy, CPU::eorc), ("NOP", CPU::imp, CPU::nopc), ("SRE", CPU::absy, CPU::srec), ("NOP", CPU::absx, CPU::nopc), ("EOR", CPU::absx, CPU::eorc), ("LSR", CPU::absx, CPU::lsrc), ("SRE", CPU::absx, CPU::srec),
        ("RTS", CPU::imp, CPU::rtsc), ("ADC", CPU::indx, CPU::adcc), ("JAM", CPU::imp, CPU::jamc), ("RRA", CPU::indx, CPU::rrac), ("NOP",  CPU::zp, CPU::nopc), ("ADC",  CPU::zp, CPU::adcc), ("ROR",  CPU::zp, CPU::rorc), ("RRA",  CPU::zp, CPU::rrac), ("PLA", CPU::imp, CPU::plac), ("ADC",  CPU::imm, CPU::adcc), ("ROR", CPU::acc, CPU::rorc), ("ARR",  CPU::imm, CPU::arrc), ("JMP",  CPU::ind, CPU::jmpc), ("ADC",  CPU::abs, CPU::adcc), ("ROR",  CPU::abs, CPU::rorc), ("RRA",  CPU::abs, CPU::rrac),
        ("BVS", CPU::rel, CPU::bvsc), ("ADC", CPU::indy, CPU::adcc), ("JAM", CPU::imp, CPU::jamc), ("RRA", CPU::indy, CPU::rrac), ("NOP", CPU::zpx, CPU::nopc), ("ADC", CPU::zpx, CPU::adcc), ("ROR", CPU::zpx, CPU::rorc), ("RRA", CPU::zpx, CPU::rrac), ("SEI", CPU::imp, CPU::seic), ("ADC", CPU::absy, CPU::adcc), ("NOP", CPU::imp, CPU::nopc), ("RRA", CPU::absy, CPU::rrac), ("NOP", CPU::absx, CPU::nopc), ("ADC", CPU::absx, CPU::adcc), ("ROR", CPU::absx, CPU::rorc), ("RRA", CPU::absx, CPU::rrac),
        ("NOP", CPU::imm, CPU::nopc), ("STA", CPU::indx, CPU::stac), ("NOP", CPU::imm, CPU::nopc), ("SAX", CPU::indx, CPU::saxc), ("STY",  CPU::zp, CPU::styc), ("STA",  CPU::zp, CPU::stac), ("STX",  CPU::zp, CPU::stxc), ("SAX",  CPU::zp, CPU::saxc), ("DEY", CPU::imp, CPU::deyc), ("NOP",  CPU::imm, CPU::nopc), ("TXA", CPU::imp, CPU::txac), ("XAA",  CPU::imm, CPU::xaac), ("STY",  CPU::abs, CPU::styc), ("STA",  CPU::abs, CPU::stac), ("STX",  CPU::abs, CPU::stxc), ("SAX",  CPU::abs, CPU::saxc),
        ("BCC", CPU::rel, CPU::bccc), ("STA", CPU::indy, CPU::stac), ("JAM", CPU::imp, CPU::jamc), ("SHA", CPU::indy, CPU::shac), ("STY", CPU::zpx, CPU::styc), ("STA", CPU::zpx, CPU::stac), ("STX", CPU::zpy, CPU::stxc), ("SAX", CPU::zpy, CPU::saxc), ("TYA", CPU::imp, CPU::tyac), ("STA", CPU::absy, CPU::stac), ("TXS", CPU::imp, CPU::txsc), ("SHS", CPU::absy, CPU::shsc), ("SHY", CPU::absx, CPU::shyc), ("STA", CPU::absx, CPU::stac), ("SHX", CPU::absy, CPU::shxc), ("SHA", CPU::absy, CPU::shac),
        ("LDY", CPU::imm, CPU::ldyc), ("LDA", CPU::indx, CPU::ldac), ("LDX", CPU::imm, CPU::ldxc), ("LAX", CPU::indx, CPU::laxc), ("LDY",  CPU::zp, CPU::ldyc), ("LDA",  CPU::zp, CPU::ldac), ("LDX",  CPU::zp, CPU::ldxc), ("LAX",  CPU::zp, CPU::laxc), ("TAY", CPU::imp, CPU::tayc), ("LDA",  CPU::imm, CPU::ldac), ("TAX", CPU::imp, CPU::taxc), ("LAX",  CPU::imm, CPU::laxc), ("LDY",  CPU::abs, CPU::ldyc), ("LDA",  CPU::abs, CPU::ldac), ("LDX",  CPU::abs, CPU::ldxc), ("LAX",  CPU::abs, CPU::laxc),
        ("BCS", CPU::rel, CPU::bcsc), ("LDA", CPU::indy, CPU::ldac), ("JAM", CPU::imp, CPU::jamc), ("LAX", CPU::indy, CPU::laxc), ("LDY", CPU::zpx, CPU::ldyc), ("LDA", CPU::zpx, CPU::ldac), ("LDX", CPU::zpy, CPU::ldxc), ("LAX", CPU::zpy, CPU::laxc), ("CLV", CPU::imp, CPU::clvc), ("LDA", CPU::absy, CPU::ldac), ("TSX", CPU::imp, CPU::tsxc), ("LAS", CPU::absy, CPU::lasc), ("LDY", CPU::absx, CPU::ldyc), ("LDA", CPU::absx, CPU::ldac), ("LDX", CPU::absy, CPU::ldxc), ("LAX", CPU::absy, CPU::laxc),
        ("CPY", CPU::imm, CPU::cpyc), ("CMP", CPU::indx, CPU::cmpc), ("NOP", CPU::imm, CPU::nopc), ("DCP", CPU::indx, CPU::dcpc), ("CPY",  CPU::zp, CPU::cpyc), ("CMP",  CPU::zp, CPU::cmpc), ("DEC",  CPU::zp, CPU::decc), ("DCP",  CPU::zp, CPU::dcpc), ("INY", CPU::imp, CPU::inyc), ("CMP",  CPU::imm, CPU::cmpc), ("DEX", CPU::imp, CPU::dexc), ("SBX",  CPU::imm, CPU::sbxc), ("CPY",  CPU::abs, CPU::cpyc), ("CMP",  CPU::abs, CPU::cmpc), ("DEC",  CPU::abs, CPU::decc), ("DCP",  CPU::abs, CPU::dcpc),
        ("BNE", CPU::rel, CPU::bnec), ("CMP", CPU::indy, CPU::cmpc), ("JAM", CPU::imp, CPU::jamc), ("DCP", CPU::indy, CPU::dcpc), ("NOP", CPU::zpx, CPU::nopc), ("CMP", CPU::zpx, CPU::cmpc), ("DEC", CPU::zpx, CPU::decc), ("DCP", CPU::zpx, CPU::dcpc), ("CLD", CPU::imp, CPU::cldc), ("CMP", CPU::absy, CPU::cmpc), ("NOP", CPU::imp, CPU::nopc), ("DCP", CPU::absy, CPU::dcpc), ("NOP", CPU::absx, CPU::nopc), ("CMP", CPU::absx, CPU::cmpc), ("DEC", CPU::absx, CPU::decc), ("DCP", CPU::absx, CPU::dcpc),
        ("CPX", CPU::imm, CPU::cpxc), ("SBC", CPU::indx, CPU::sbcc), ("NOP", CPU::imm, CPU::nopc), ("ISC", CPU::indx, CPU::iscc), ("CPX",  CPU::zp, CPU::cpxc), ("SBC",  CPU::zp, CPU::sbcc), ("INC",  CPU::zp, CPU::incc), ("ISC",  CPU::zp, CPU::iscc), ("INX", CPU::imp, CPU::inxc), ("SBC",  CPU::imm, CPU::sbcc), ("NOP", CPU::imp, CPU::nopc), ("SBC",  CPU::imm, CPU::sbcc), ("CPX",  CPU::abs, CPU::cpxc), ("SBC",  CPU::abs, CPU::sbcc), ("INC",  CPU::abs, CPU::incc), ("ISC",  CPU::abs, CPU::iscc),
        ("BEQ", CPU::rel, CPU::beqc), ("SBC", CPU::indy, CPU::sbcc), ("JAM", CPU::imp, CPU::jamc), ("ISC", CPU::indy, CPU::iscc), ("NOP", CPU::zpx, CPU::nopc), ("SBC", CPU::zpx, CPU::sbcc), ("INC", CPU::zpx, CPU::incc), ("ISC", CPU::zpx, CPU::iscc), ("SED", CPU::imp, CPU::sedc), ("SBC", CPU::absy, CPU::sbcc), ("NOP", CPU::imp, CPU::nopc), ("ISC", CPU::absy, CPU::iscc), ("NOP", CPU::absx, CPU::nopc), ("SBC", CPU::absx, CPU::sbcc), ("INC", CPU::absx, CPU::incc), ("ISC", CPU::absx, CPU::iscc),
    ];

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
            i: true,  //Bit 2 (IRQs disabled on power-on)
            z: false, //Bit 1
            c: false, //Bit 0

            current_instruction: CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Reset,
            },

            pending_nmi: false,
            pending_irq: false,

            elapsed_cycles: 0,
        }
    }

    pub fn system_nmi(&mut self) {
        //Edge-detected
        self.pending_nmi = true;
    }

    pub fn system_irq(&mut self, interrupt: bool) {
        //Level-detected
        self.pending_irq = interrupt;
    }

    pub fn system_reset(&mut self) {
        self.current_instruction = CurrentInstruction {
            remaining_cycles: self.resetc(),
            instruction_type: InstructionType::Reset,
        }
    }

    pub fn clock(&mut self, bus: &mut dyn CPUBus) {
        self.elapsed_cycles += 1;
        self.current_instruction.remaining_cycles -= 1;

        if self.current_instruction.remaining_cycles == 0 {
            self.execute_operation(bus);
        }
    }

    pub fn cycles_remaining(&self) -> u8 {
        self.current_instruction.remaining_cycles
    }
}

impl CPU {
    #[inline]
    fn execute_operation(&mut self, bus: &mut dyn CPUBus) {
        let current_instruction = self.current_instruction.clone();
        match current_instruction.instruction_type {
            InstructionType::Jam => {
                self.current_instruction.remaining_cycles = 0xff;
            }
            InstructionType::Reset => {
                self.reset(bus);
                self.current_instruction = self.fetch_next_instruction(bus);
            }
            InstructionType::NMI => {
                self.nmi(bus);
                self.current_instruction = self.fetch_next_instruction(bus);
            }
            InstructionType::IRQ => {
                self.irq(bus);
                self.current_instruction = self.fetch_next_instruction(bus);
            }
            InstructionType::Instruction {
                opcode,
                addr_mode: addressing_mode,
            } => {
                let i_flag_before = self.i;
                self.execute(opcode, &addressing_mode, bus);
                let i_flag_after = self.i;

                let polled_i_flag = match opcode {
                    0x28 | 0x58 | 0x78 => i_flag_before,
                    _ => i_flag_after,
                };
                //We only poll for interrupts in non-interrupt routines
                //(i.e. regular instructions)
                self.poll_for_interrupts_or_fetch_next_instruction(bus, polled_i_flag);
            }
        }
    }

    #[inline]
    fn poll_for_interrupts_or_fetch_next_instruction(
        &mut self,
        bus: &mut dyn CPUBus,
        i_flag: bool,
    ) {
        if self.pending_nmi {
            self.current_instruction = CurrentInstruction {
                remaining_cycles: self.nmic(),
                instruction_type: InstructionType::NMI,
            };
        } else if self.pending_irq && !i_flag {
            self.current_instruction = CurrentInstruction {
                remaining_cycles: self.irqc(),
                instruction_type: InstructionType::IRQ,
            };
        } else {
            self.current_instruction = self.fetch_next_instruction(bus);
        }
    }

    #[inline]
    fn fetch_next_instruction(&mut self, bus: &mut dyn CPUBus) -> CurrentInstruction {
        let opcode = self.fetch_byte(bus);
        let (_, addr_mode_fn, cycles_fn) = CPU::LOOKUP_TABLE[opcode as usize];
        let addr_mode = addr_mode_fn(self, bus);
        let cycles = cycles_fn(self, &addr_mode);
        if cycles == 0 {
            //Jam
            CurrentInstruction {
                remaining_cycles: 0xff,
                instruction_type: InstructionType::Jam,
            }
        } else {
            CurrentInstruction {
                remaining_cycles: cycles,
                instruction_type: InstructionType::Instruction { opcode, addr_mode },
            }
        }
    }

    #[inline]
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

    #[inline]
    fn fetch_byte(&mut self, bus: &dyn CPUBus) -> u8 {
        let data = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        data
    }

    #[inline]
    fn fetch_two_bytes_as_u16(&mut self, bus: &dyn CPUBus) -> u16 {
        let low_byte: u16 = bus.read(self.pc.wrapping_add(0)) as u16;
        let high_byte: u16 = bus.read(self.pc.wrapping_add(1)) as u16;
        self.pc = self.pc.wrapping_add(2);

        high_byte << 8 | low_byte
    }
}

impl CPU {
    #[inline]
    fn execute(&mut self, opcode: u8, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        match opcode {
            0x00 => self.brk(mode, bus),
            0x08 => self.php(mode, bus),
            0x09 | 0x0D | 0x1D | 0x19 | 0x05 | 0x15 | 0x01 | 0x11 => self.ora(mode),
            0x0A | 0x0E | 0x1E | 0x06 | 0x16 => self.asl(mode, bus),
            0x0B | 0x2B => self.anc(mode),
            0x0F | 0x1F | 0x1B | 0x07 | 0x17 | 0x03 | 0x13 => self.slo(mode, bus),
            0x10 => self.bpl(mode),
            0x18 => self.clc(mode),
            0x20 => self.jsr(mode, bus),
            0x28 => self.plp(mode, bus),
            0x29 | 0x2D | 0x3D | 0x39 | 0x25 | 0x35 | 0x21 | 0x31 => self.and(mode),
            0x2A | 0x2E | 0x3E | 0x26 | 0x36 => self.rol(mode, bus),
            0x2C | 0x24 => self.bit(mode),
            0x2F | 0x3F | 0x3B | 0x27 | 0x37 | 0x23 | 0x33 => self.rla(mode, bus),
            0x30 => self.bmi(mode),
            0x38 => self.sec(mode),
            0x40 => self.rti(mode, bus),
            0x48 => self.pha(mode, bus),
            0x49 | 0x4D | 0x5D | 0x59 | 0x45 | 0x55 | 0x41 | 0x51 => self.eor(mode),
            0x4A | 0x4E | 0x5E | 0x46 | 0x56 => self.lsr(mode, bus),
            0x4B => self.asr(mode, bus),
            0x4C | 0x6C => self.jmp(mode),
            0x4F | 0x5F | 0x5B | 0x47 | 0x57 | 0x43 | 0x53 => self.sre(mode, bus),
            0x50 => self.bvc(mode),
            0x58 => self.cli(mode),
            0x60 => self.rts(mode, bus),
            0x68 => self.pla(mode, bus),
            0x69 | 0x6D | 0x7D | 0x79 | 0x65 | 0x75 | 0x61 | 0x71 => self.adc(mode),
            0x6A | 0x6E | 0x7E | 0x66 | 0x76 => self.ror(mode, bus),
            0x6B => self.arr(mode, bus),
            0x6F | 0x7F | 0x7B | 0x67 | 0x77 | 0x63 | 0x73 => self.rra(mode, bus),
            0x70 => self.bvs(mode),
            0x78 => self.sei(mode),
            0x88 => self.dey(mode),
            0x8C | 0x84 | 0x94 => self.sty(mode, bus),
            0x8A => self.txa(mode),
            0x8B => self.xaa(mode, bus),
            0x8D | 0x9D | 0x99 | 0x85 | 0x95 | 0x81 | 0x91 => self.sta(mode, bus),
            0x8E | 0x86 | 0x96 => self.stx(mode, bus),
            0x8F | 0x87 | 0x97 | 0x83 => self.sax(mode, bus),
            0x90 => self.bcc(mode),
            0x98 => self.tya(mode),
            0x9A => self.txs(mode),
            0x9B => self.shs(mode, bus),
            0x9C => self.shy(mode, bus),
            0x9E => self.shx(mode, bus),
            0x9F | 0x93 => self.sha(mode, bus),
            0xA2 | 0xAE | 0xBE | 0xA6 | 0xB6 => self.ldx(mode),
            0xA8 => self.tay(mode),
            0xA9 | 0xAD | 0xBD | 0xB9 | 0xA5 | 0xB5 | 0xA1 | 0xB1 => self.lda(mode),
            0xA0 | 0xAC | 0xBC | 0xA4 | 0xB4 => self.ldy(mode),
            0xAA => self.tax(mode),
            0xAB | 0xAF | 0xBF | 0xA7 | 0xB7 | 0xA3 | 0xB3 => self.lax(mode),
            0xB0 => self.bcs(mode),
            0xB8 => self.clv(mode),
            0xBA => self.tsx(mode),
            0xBB => self.las(mode),
            0xC0 | 0xCC | 0xC4 => self.cpy(mode),
            0xC8 => self.iny(mode),
            0xC9 | 0xCD | 0xDD | 0xD9 | 0xC5 | 0xD5 | 0xC1 | 0xD1 => self.cmp(mode),
            0xCA => self.dex(mode),
            0xCB => self.sbx(mode),
            0xCE | 0xDE | 0xC6 | 0xD6 => self.dec(mode, bus),
            0xCF | 0xDF | 0xDB | 0xC7 | 0xD7 | 0xC3 | 0xD3 => self.dcp(mode, bus),
            0xD0 => self.bne(mode),
            0xD8 => self.cld(mode),
            0xE0 | 0xEC | 0xE4 => self.cpx(mode),
            0xE8 => self.inx(mode),
            0xE9 | 0xEB | 0xED | 0xFD | 0xF9 | 0xE5 | 0xF5 | 0xE1 | 0xF1 => self.sbc(mode),
            0xEA | 0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA | 0x80 | 0x82 | 0x89 | 0xC2 | 0xE2
            | 0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC | 0x04 | 0x44 | 0x64 | 0x14 | 0x34
            | 0x54 | 0x74 | 0xD4 | 0xF4 => self.nop(mode),
            0xEE | 0xFE | 0xE6 | 0xF6 => self.inc(mode, bus),
            0xEF | 0xFF | 0xFB | 0xE7 | 0xF7 | 0xE3 | 0xF3 => self.isc(mode, bus),
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
        assert_eq!(true, cpu.i); //Bit 2
        assert_eq!(false, cpu.z); //Bit 1
        assert_eq!(false, cpu.c); //Bit 0

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Reset
            },
            cpu.current_instruction
        );

        assert_eq!(false, cpu.pending_nmi);
        assert_eq!(false, cpu.pending_irq);
    }

    #[test]
    fn test_get_status_byte_no_flags() {
        let mut cpu = CPU::new();
        cpu.i = false;
        assert_eq!(0b0010_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_negative_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;
        cpu.n = true;
        assert_eq!(0b1010_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;
        cpu.v = true;
        assert_eq!(0b0110_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_break_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;
        assert_eq!(0b0011_0000, cpu.get_status_byte(true))
    }

    #[test]
    fn test_get_status_byte_decimal_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;
        cpu.d = true;
        assert_eq!(0b0010_1000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_interrupt_flag() {
        let cpu = CPU::new();
        assert_eq!(0b0010_0100, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_zero_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;
        cpu.z = true;
        assert_eq!(0b0010_0010, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_carry_flag() {
        let mut cpu = CPU::new();
        cpu.i = false;
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

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 2,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x69,
                    addr_mode: AddrModeResult {
                        addr: None,
                        data: Some(0xff),
                        cycles: 0,
                        mode: addr::AddrModeType::IMM,
                        bytes: 2,
                        operands: "FF".to_owned(),
                        repr: "#$FF".to_owned()
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
                addr_mode: cpu._imp(),
            },
        };

        cpu.system_reset();

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Reset
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_nmi_request_triggered() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR))
            .return_const(0x40);
        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR + 1))
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).return_const(0x69);
        bus.expect_read().with(eq(0x2041)).return_const(0x69);
        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        cpu.system_nmi();

        assert_eq!(true, cpu.pending_nmi);

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 2,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x69,
                    addr_mode: cpu._imm(0x69)
                }
            },
            cpu.current_instruction
        );
        assert_eq!(0x2042, cpu.pc);

        for _ in 0..2 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::NMI
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_irq_request_triggered() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR))
            .return_const(0x40);
        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR + 1))
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).return_const(0x69);
        bus.expect_read().with(eq(0x2041)).return_const(0x69);

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        cpu.i = false;
        cpu.system_irq(true);
        for _ in 0..2 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::IRQ
            },
            cpu.current_instruction
        )
    }

    #[test]
    fn test_irq_request_ignored_on_flag_set() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR))
            .return_const(0x40);
        bus.expect_read()
            .with(eq(CPU::RESET_VECTOR + 1))
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).return_const(0x69);
        bus.expect_read().with(eq(0x2041)).return_const(0x69);
        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        cpu.i = true;
        cpu.system_irq(true);

        for _ in 0..2 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x0,
                    addr_mode: cpu._imm(0x0)
                }
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_cpu_cli_delays_interrupt() {
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
        bus.expect_read().with(eq(0x2040)).once().return_const(0x58); //CLI
        bus.expect_read().with(eq(0x2041)).once().return_const(0x58);

        bus.expect_read().return_const(0x0);

        cpu.system_irq(true);

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 2,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x58,
                    addr_mode: cpu._imp()
                }
            },
            cpu.current_instruction
        );

        for _ in 0..2 {
            cpu.clock(&mut bus);
        }

        //Interrupt delayed until end of next instruction
        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 2,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x58,
                    addr_mode: cpu._imp()
                }
            },
            cpu.current_instruction
        );

        for _ in 0..2 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::IRQ
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_cpu_sei_triggers_interrupt() {
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
        bus.expect_read().with(eq(0x2040)).once().return_const(0x78); //SEI

        bus.expect_read().return_const(0x0);

        cpu.system_irq(true);

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        cpu.i = false;

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 2,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x78,
                    addr_mode: cpu._imp()
                }
            },
            cpu.current_instruction
        );

        for _ in 0..2 {
            cpu.clock(&mut bus);
        }

        //Interrupt triggered immediately (despite I flag set)
        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::IRQ
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_cpu_plp_triggers_interrupt() {
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

        bus.expect_read().with(eq(0x2040)).once().return_const(0x28); //PLP
        bus.expect_read()
            .with(eq(0x1fe))
            .once()
            .return_const(0b1111_1111);

        bus.expect_read().return_const(0x0);

        cpu.system_irq(true);

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        cpu.i = false;

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 4,
                instruction_type: InstructionType::Instruction {
                    opcode: 0x28,
                    addr_mode: cpu._imp()
                }
            },
            cpu.current_instruction
        );

        for _ in 0..4 {
            cpu.clock(&mut bus);
        }

        //Interrupt triggered immediately (despite I flag set)
        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::IRQ
            },
            cpu.current_instruction
        );
    }

    #[test]
    fn test_jam() {
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

        bus.expect_read().with(eq(0x2040)).once().return_const(0x2);
        bus.expect_read().return_const(0x0);

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 0xff,
                instruction_type: InstructionType::Jam
            },
            cpu.current_instruction
        );

        for _ in 0..10_000 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            InstructionType::Jam,
            cpu.current_instruction.instruction_type
        );

        cpu.system_nmi();

        for _ in 0..10_000 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            InstructionType::Jam,
            cpu.current_instruction.instruction_type
        );

        cpu.system_irq(true);

        for _ in 0..10_000 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            InstructionType::Jam,
            cpu.current_instruction.instruction_type
        );

        cpu.system_reset();

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Reset
            },
            cpu.current_instruction
        );
    }
}
