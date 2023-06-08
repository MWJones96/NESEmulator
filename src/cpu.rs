use self::{addr::AddrModeResult, bus::CPUBus};

pub mod bus;

mod addr;
mod ops;

type Mnemonic = &'static str;
type AddrModeFn = fn(&mut CPU, &dyn CPUBus) -> AddrModeResult;
type CycleCountFn = fn(&CPU, &AddrModeResult) -> u8;
type ExecuteFn = fn(&mut CPU, &AddrModeResult, &mut dyn CPUBus);

#[derive(PartialEq, Debug, Clone)]
enum InstructionType {
    Jam,
    Reset,
    Nmi,
    Irq,
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
            InstructionType::Irq => "IRQ".to_owned(),
            InstructionType::Nmi => "NMI".to_owned(),
            InstructionType::Reset => "RESET".to_owned(),
        }
    }
}

impl CPU {
    const NMI_VECTOR: u16 = 0xfffa;
    const RESET_VECTOR: u16 = 0xfffc;
    const IRQ_VECTOR: u16 = 0xfffe;

    #[rustfmt::skip]
    const LOOKUP_TABLE: [(Mnemonic, AddrModeFn, CycleCountFn, ExecuteFn); 256] = [
        ("BRK", CPU::imm, CPU::brkc, CPU::brk), ("ORA", CPU::indx, CPU::orac, CPU::ora), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("SLO", CPU::indx, CPU::sloc, CPU::slo), ("NOP",  CPU::zp, CPU::nopc, CPU::nop), ("ORA",  CPU::zp, CPU::orac, CPU::ora), ("ASL",  CPU::zp, CPU::aslc, CPU::asl), ("SLO",  CPU::zp, CPU::sloc, CPU::slo), ("PHP", CPU::imp, CPU::phpc, CPU::php), ("ORA",  CPU::imm, CPU::orac, CPU::ora), ("ASL", CPU::acc, CPU::aslc, CPU::asl), ("ANC",  CPU::imm, CPU::ancc, CPU::anc), ("NOP",  CPU::abs, CPU::nopc, CPU::nop), ("ORA",  CPU::abs, CPU::orac, CPU::ora), ("ASL",  CPU::abs, CPU::aslc, CPU::asl), ("SLO",  CPU::abs, CPU::sloc, CPU::slo),
        ("BPL", CPU::rel, CPU::bplc, CPU::bpl), ("ORA", CPU::indy, CPU::orac, CPU::ora), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("SLO", CPU::indy, CPU::sloc, CPU::slo), ("NOP", CPU::zpx, CPU::nopc, CPU::nop), ("ORA", CPU::zpx, CPU::orac, CPU::ora), ("ASL", CPU::zpx, CPU::aslc, CPU::asl), ("SLO", CPU::zpx, CPU::sloc, CPU::slo), ("CLC", CPU::imp, CPU::clcc, CPU::clc), ("ORA", CPU::absy, CPU::orac, CPU::ora), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("SLO", CPU::absy, CPU::sloc, CPU::slo), ("NOP", CPU::absx, CPU::nopc, CPU::nop), ("ORA", CPU::absx, CPU::orac, CPU::ora), ("ASL", CPU::absx, CPU::aslc, CPU::asl), ("SLO", CPU::absx, CPU::sloc, CPU::slo),
        ("JSR", CPU::abs, CPU::jsrc, CPU::jsr), ("AND", CPU::indx, CPU::andc, CPU::and), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("RLA", CPU::indx, CPU::rlac, CPU::rla), ("BIT",  CPU::zp, CPU::bitc, CPU::bit), ("AND",  CPU::zp, CPU::andc, CPU::and), ("ROL",  CPU::zp, CPU::rolc, CPU::rol), ("RLA",  CPU::zp, CPU::rlac, CPU::rla), ("PLP", CPU::imp, CPU::plpc, CPU::plp), ("AND",  CPU::imm, CPU::andc, CPU::and), ("ROL", CPU::acc, CPU::rolc, CPU::rol), ("ANC",  CPU::imm, CPU::ancc, CPU::anc), ("BIT",  CPU::abs, CPU::bitc, CPU::bit), ("AND",  CPU::abs, CPU::andc, CPU::and), ("ROL",  CPU::abs, CPU::rolc, CPU::rol), ("RLA",  CPU::abs, CPU::rlac, CPU::rla),
        ("BMI", CPU::rel, CPU::bmic, CPU::bmi), ("AND", CPU::indy, CPU::andc, CPU::and), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("RLA", CPU::indy, CPU::rlac, CPU::rla), ("NOP", CPU::zpx, CPU::nopc, CPU::nop), ("AND", CPU::zpx, CPU::andc, CPU::and), ("ROL", CPU::zpx, CPU::rolc, CPU::rol), ("RLA", CPU::zpx, CPU::rlac, CPU::rla), ("SEC", CPU::imp, CPU::secc, CPU::sec), ("AND", CPU::absy, CPU::andc, CPU::and), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("RLA", CPU::absy, CPU::rlac, CPU::rla), ("NOP", CPU::absx, CPU::nopc, CPU::nop), ("AND", CPU::absx, CPU::andc, CPU::and), ("ROL", CPU::absx, CPU::rolc, CPU::rol), ("RLA", CPU::absx, CPU::rlac, CPU::rla),
        ("RTI", CPU::imp, CPU::rtic, CPU::rti), ("EOR", CPU::indx, CPU::eorc, CPU::eor), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("SRE", CPU::indx, CPU::srec, CPU::sre), ("NOP",  CPU::zp, CPU::nopc, CPU::nop), ("EOR",  CPU::zp, CPU::eorc, CPU::eor), ("LSR",  CPU::zp, CPU::lsrc, CPU::lsr), ("SRE",  CPU::zp, CPU::srec, CPU::sre), ("PHA", CPU::imp, CPU::phac, CPU::pha), ("EOR",  CPU::imm, CPU::eorc, CPU::eor), ("LSR", CPU::acc, CPU::lsrc, CPU::lsr), ("ASR",  CPU::imm, CPU::asrc, CPU::asr), ("JMP",  CPU::abs, CPU::jmpc, CPU::jmp), ("EOR",  CPU::abs, CPU::eorc, CPU::eor), ("LSR",  CPU::abs, CPU::lsrc, CPU::lsr), ("SRE",  CPU::abs, CPU::srec, CPU::sre),
        ("BVC", CPU::rel, CPU::bvcc, CPU::bvc), ("EOR", CPU::indy, CPU::eorc, CPU::eor), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("SRE", CPU::indy, CPU::srec, CPU::sre), ("NOP", CPU::zpx, CPU::nopc, CPU::nop), ("EOR", CPU::zpx, CPU::eorc, CPU::eor), ("LSR", CPU::zpx, CPU::lsrc, CPU::lsr), ("SRE", CPU::zpx, CPU::srec, CPU::sre), ("CLI", CPU::imp, CPU::clic, CPU::cli), ("EOR", CPU::absy, CPU::eorc, CPU::eor), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("SRE", CPU::absy, CPU::srec, CPU::sre), ("NOP", CPU::absx, CPU::nopc, CPU::nop), ("EOR", CPU::absx, CPU::eorc, CPU::eor), ("LSR", CPU::absx, CPU::lsrc, CPU::lsr), ("SRE", CPU::absx, CPU::srec, CPU::sre),
        ("RTS", CPU::imp, CPU::rtsc, CPU::rts), ("ADC", CPU::indx, CPU::adcc, CPU::adc), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("RRA", CPU::indx, CPU::rrac, CPU::rra), ("NOP",  CPU::zp, CPU::nopc, CPU::nop), ("ADC",  CPU::zp, CPU::adcc, CPU::adc), ("ROR",  CPU::zp, CPU::rorc, CPU::ror), ("RRA",  CPU::zp, CPU::rrac, CPU::rra), ("PLA", CPU::imp, CPU::plac, CPU::pla), ("ADC",  CPU::imm, CPU::adcc, CPU::adc), ("ROR", CPU::acc, CPU::rorc, CPU::ror), ("ARR",  CPU::imm, CPU::arrc, CPU::arr), ("JMP",  CPU::ind, CPU::jmpc, CPU::jmp), ("ADC",  CPU::abs, CPU::adcc, CPU::adc), ("ROR",  CPU::abs, CPU::rorc, CPU::ror), ("RRA",  CPU::abs, CPU::rrac, CPU::rra),
        ("BVS", CPU::rel, CPU::bvsc, CPU::bvs), ("ADC", CPU::indy, CPU::adcc, CPU::adc), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("RRA", CPU::indy, CPU::rrac, CPU::rra), ("NOP", CPU::zpx, CPU::nopc, CPU::nop), ("ADC", CPU::zpx, CPU::adcc, CPU::adc), ("ROR", CPU::zpx, CPU::rorc, CPU::ror), ("RRA", CPU::zpx, CPU::rrac, CPU::rra), ("SEI", CPU::imp, CPU::seic, CPU::sei), ("ADC", CPU::absy, CPU::adcc, CPU::adc), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("RRA", CPU::absy, CPU::rrac, CPU::rra), ("NOP", CPU::absx, CPU::nopc, CPU::nop), ("ADC", CPU::absx, CPU::adcc, CPU::adc), ("ROR", CPU::absx, CPU::rorc, CPU::ror), ("RRA", CPU::absx, CPU::rrac, CPU::rra),
        ("NOP", CPU::imm, CPU::nopc, CPU::nop), ("STA", CPU::indx, CPU::stac, CPU::sta), ("NOP", CPU::imm, CPU::nopc, CPU::nop),  ("SAX", CPU::indx, CPU::saxc, CPU::sax), ("STY",  CPU::zp, CPU::styc, CPU::sty), ("STA",  CPU::zp, CPU::stac, CPU::sta), ("STX",  CPU::zp, CPU::stxc, CPU::stx), ("SAX",  CPU::zp, CPU::saxc, CPU::sax), ("DEY", CPU::imp, CPU::deyc, CPU::dey), ("NOP",  CPU::imm, CPU::nopc, CPU::nop), ("TXA", CPU::imp, CPU::txac, CPU::txa), ("XAA",  CPU::imm, CPU::xaac, CPU::xaa), ("STY",  CPU::abs, CPU::styc, CPU::sty), ("STA",  CPU::abs, CPU::stac, CPU::sta), ("STX",  CPU::abs, CPU::stxc, CPU::stx), ("SAX",  CPU::abs, CPU::saxc, CPU::sax),
        ("BCC", CPU::rel, CPU::bccc, CPU::bcc), ("STA", CPU::indy, CPU::stac, CPU::sta), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("SHA", CPU::indy, CPU::shac, CPU::sha), ("STY", CPU::zpx, CPU::styc, CPU::sty), ("STA", CPU::zpx, CPU::stac, CPU::sta), ("STX", CPU::zpy, CPU::stxc, CPU::stx), ("SAX", CPU::zpy, CPU::saxc, CPU::sax), ("TYA", CPU::imp, CPU::tyac, CPU::tya), ("STA", CPU::absy, CPU::stac, CPU::sta), ("TXS", CPU::imp, CPU::txsc, CPU::txs), ("SHS", CPU::absy, CPU::shsc, CPU::shs), ("SHY", CPU::absx, CPU::shyc, CPU::shy), ("STA", CPU::absx, CPU::stac, CPU::sta), ("SHX", CPU::absy, CPU::shxc, CPU::shx), ("SHA", CPU::absy, CPU::shac, CPU::sha),
        ("LDY", CPU::imm, CPU::ldyc, CPU::ldy), ("LDA", CPU::indx, CPU::ldac, CPU::lda), ("LDX", CPU::imm, CPU::ldxc, CPU::ldx),  ("LAX", CPU::indx, CPU::laxc, CPU::lax), ("LDY",  CPU::zp, CPU::ldyc, CPU::ldy), ("LDA",  CPU::zp, CPU::ldac, CPU::lda), ("LDX",  CPU::zp, CPU::ldxc, CPU::ldx), ("LAX",  CPU::zp, CPU::laxc, CPU::lax), ("TAY", CPU::imp, CPU::tayc, CPU::tay), ("LDA",  CPU::imm, CPU::ldac, CPU::lda), ("TAX", CPU::imp, CPU::taxc, CPU::tax), ("LAX",  CPU::imm, CPU::laxc, CPU::lax), ("LDY",  CPU::abs, CPU::ldyc, CPU::ldy), ("LDA",  CPU::abs, CPU::ldac, CPU::lda), ("LDX",  CPU::abs, CPU::ldxc, CPU::ldx), ("LAX",  CPU::abs, CPU::laxc, CPU::lax),
        ("BCS", CPU::rel, CPU::bcsc, CPU::bcs), ("LDA", CPU::indy, CPU::ldac, CPU::lda), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("LAX", CPU::indy, CPU::laxc, CPU::lax), ("LDY", CPU::zpx, CPU::ldyc, CPU::ldy), ("LDA", CPU::zpx, CPU::ldac, CPU::lda), ("LDX", CPU::zpy, CPU::ldxc, CPU::ldx), ("LAX", CPU::zpy, CPU::laxc, CPU::lax), ("CLV", CPU::imp, CPU::clvc, CPU::clv), ("LDA", CPU::absy, CPU::ldac, CPU::lda), ("TSX", CPU::imp, CPU::tsxc, CPU::tsx), ("LAS", CPU::absy, CPU::lasc, CPU::las), ("LDY", CPU::absx, CPU::ldyc, CPU::ldy), ("LDA", CPU::absx, CPU::ldac, CPU::lda), ("LDX", CPU::absy, CPU::ldxc, CPU::ldx), ("LAX", CPU::absy, CPU::laxc, CPU::lax),
        ("CPY", CPU::imm, CPU::cpyc, CPU::cpy), ("CMP", CPU::indx, CPU::cmpc, CPU::cmp), ("NOP", CPU::imm, CPU::nopc, CPU::nop),  ("DCP", CPU::indx, CPU::dcpc, CPU::dcp), ("CPY",  CPU::zp, CPU::cpyc, CPU::cpy), ("CMP",  CPU::zp, CPU::cmpc, CPU::cmp), ("DEC",  CPU::zp, CPU::decc, CPU::dec), ("DCP",  CPU::zp, CPU::dcpc, CPU::dcp), ("INY", CPU::imp, CPU::inyc, CPU::iny), ("CMP",  CPU::imm, CPU::cmpc, CPU::cmp), ("DEX", CPU::imp, CPU::dexc, CPU::dex), ("SBX",  CPU::imm, CPU::sbxc, CPU::sbx), ("CPY",  CPU::abs, CPU::cpyc, CPU::cpy), ("CMP",  CPU::abs, CPU::cmpc, CPU::cmp), ("DEC",  CPU::abs, CPU::decc, CPU::dec), ("DCP",  CPU::abs, CPU::dcpc, CPU::dcp),
        ("BNE", CPU::rel, CPU::bnec, CPU::bne), ("CMP", CPU::indy, CPU::cmpc, CPU::cmp), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("DCP", CPU::indy, CPU::dcpc, CPU::dcp), ("NOP", CPU::zpx, CPU::nopc, CPU::nop), ("CMP", CPU::zpx, CPU::cmpc, CPU::cmp), ("DEC", CPU::zpx, CPU::decc, CPU::dec), ("DCP", CPU::zpx, CPU::dcpc, CPU::dcp), ("CLD", CPU::imp, CPU::cldc, CPU::cld), ("CMP", CPU::absy, CPU::cmpc, CPU::cmp), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("DCP", CPU::absy, CPU::dcpc, CPU::dcp), ("NOP", CPU::absx, CPU::nopc, CPU::nop), ("CMP", CPU::absx, CPU::cmpc, CPU::cmp), ("DEC", CPU::absx, CPU::decc, CPU::dec), ("DCP", CPU::absx, CPU::dcpc, CPU::dcp),
        ("CPX", CPU::imm, CPU::cpxc, CPU::cpx), ("SBC", CPU::indx, CPU::sbcc, CPU::sbc), ("NOP", CPU::imm, CPU::nopc, CPU::nop),  ("ISC", CPU::indx, CPU::iscc, CPU::isc), ("CPX",  CPU::zp, CPU::cpxc, CPU::cpx), ("SBC",  CPU::zp, CPU::sbcc, CPU::sbc), ("INC",  CPU::zp, CPU::incc, CPU::inc), ("ISC",  CPU::zp, CPU::iscc, CPU::isc), ("INX", CPU::imp, CPU::inxc, CPU::inx), ("SBC",  CPU::imm, CPU::sbcc, CPU::sbc), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("SBC",  CPU::imm, CPU::sbcc, CPU::sbc), ("CPX",  CPU::abs, CPU::cpxc, CPU::cpx), ("SBC",  CPU::abs, CPU::sbcc, CPU::sbc), ("INC",  CPU::abs, CPU::incc, CPU::inc), ("ISC",  CPU::abs, CPU::iscc, CPU::isc),
        ("BEQ", CPU::rel, CPU::beqc, CPU::beq), ("SBC", CPU::indy, CPU::sbcc, CPU::sbc), ("JAM", CPU::imp, CPU::jamc, CPU::_jam), ("ISC", CPU::indy, CPU::iscc, CPU::isc), ("NOP", CPU::zpx, CPU::nopc, CPU::nop), ("SBC", CPU::zpx, CPU::sbcc, CPU::sbc), ("INC", CPU::zpx, CPU::incc, CPU::inc), ("ISC", CPU::zpx, CPU::iscc, CPU::isc), ("SED", CPU::imp, CPU::sedc, CPU::sed), ("SBC", CPU::absy, CPU::sbcc, CPU::sbc), ("NOP", CPU::imp, CPU::nopc, CPU::nop), ("ISC", CPU::absy, CPU::iscc, CPU::isc), ("NOP", CPU::absx, CPU::nopc, CPU::nop), ("SBC", CPU::absx, CPU::sbcc, CPU::sbc), ("INC", CPU::absx, CPU::incc, CPU::inc), ("ISC", CPU::absx, CPU::iscc, CPU::isc),
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

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
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
            InstructionType::Nmi => {
                self.nmi(bus);
                self.current_instruction = self.fetch_next_instruction(bus);
            }
            InstructionType::Irq => {
                self.irq(bus);
                self.current_instruction = self.fetch_next_instruction(bus);
            }
            InstructionType::Instruction {
                opcode,
                addr_mode: addressing_mode,
            } => {
                let i_flag_before = self.i;

                let exec_fn = CPU::LOOKUP_TABLE[opcode as usize].3;
                exec_fn(self, &addressing_mode, bus);

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

    fn poll_for_interrupts_or_fetch_next_instruction(
        &mut self,
        bus: &mut dyn CPUBus,
        i_flag: bool,
    ) {
        if self.pending_nmi {
            self.current_instruction = CurrentInstruction {
                remaining_cycles: self.nmic(),
                instruction_type: InstructionType::Nmi,
            };
        } else if self.pending_irq && !i_flag {
            self.current_instruction = CurrentInstruction {
                remaining_cycles: self.irqc(),
                instruction_type: InstructionType::Irq,
            };
        } else {
            self.current_instruction = self.fetch_next_instruction(bus);
        }
    }

    fn fetch_next_instruction(&mut self, bus: &mut dyn CPUBus) -> CurrentInstruction {
        let opcode = self.fetch_byte(bus);
        let (_, addr_mode_fn, cycles_fn, _) = CPU::LOOKUP_TABLE[opcode as usize];
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

    fn get_status_byte(&self, brk: bool) -> u8 {
        (self.n as u8) << 7
            | (self.v as u8) << 6
            | 0x1 << 5
            | (brk as u8) << 4
            | (self.d as u8) << 3
            | (self.i as u8) << 2
            | (self.z as u8) << 1
            | (self.c as u8)
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
                        mode: addr::AddrModeType::Imm,
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
                instruction_type: InstructionType::Nmi
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
                instruction_type: InstructionType::Irq
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
                instruction_type: InstructionType::Irq
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
                instruction_type: InstructionType::Irq
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
                instruction_type: InstructionType::Irq
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
