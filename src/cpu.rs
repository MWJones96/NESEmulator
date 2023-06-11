use mockall::automock;

use crate::bus::Bus;

use self::addr::AddrModeResult;

mod addr;
mod ops;

type Mnemonic = &'static str;
type AddrModeFn = fn(&mut NESCPU, &dyn Bus) -> AddrModeResult;
type CycleCountFn = fn(&NESCPU, &AddrModeResult) -> u8;
type ExecuteFn = fn(&mut NESCPU, &AddrModeResult, &mut dyn Bus);

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

#[automock]
pub trait CPU {
    fn clock(&mut self, bus: &mut dyn Bus);
    fn cpu_reset(&mut self);
    fn cpu_irq(&mut self, interrupt: bool);
    fn cpu_nmi(&mut self);
    fn cycles_remaining(&self) -> u8;
}

pub struct NESCPU {
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

impl ToString for NESCPU {
    fn to_string(&self) -> String {
        match &self.current_instruction.instruction_type {
            InstructionType::Instruction { opcode, addr_mode } => {
                format!(
                    "{:04X}  {:02X} {: <6} {} {: <27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
                    self.pc.wrapping_sub(addr_mode.bytes as u16),
                    *opcode,
                    addr_mode.operands,
                    NESCPU::LOOKUP_TABLE[*opcode as usize].0,
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

impl NESCPU {
    const NMI_VECTOR: u16 = 0xfffa;
    const RESET_VECTOR: u16 = 0xfffc;
    const IRQ_VECTOR: u16 = 0xfffe;

    #[rustfmt::skip]
    const LOOKUP_TABLE: [(Mnemonic, AddrModeFn, CycleCountFn, ExecuteFn); 256] = [
        ("BRK", NESCPU::imm, NESCPU::brkc, NESCPU::brk), ("ORA", NESCPU::indx, NESCPU::orac, NESCPU::ora), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("SLO", NESCPU::indx, NESCPU::sloc, NESCPU::slo), ("NOP",  NESCPU::zp, NESCPU::nopc, NESCPU::nop), ("ORA",  NESCPU::zp, NESCPU::orac, NESCPU::ora), ("ASL",  NESCPU::zp, NESCPU::aslc, NESCPU::asl), ("SLO",  NESCPU::zp, NESCPU::sloc, NESCPU::slo), ("PHP", NESCPU::imp, NESCPU::phpc, NESCPU::php), ("ORA",  NESCPU::imm, NESCPU::orac, NESCPU::ora), ("ASL", NESCPU::acc, NESCPU::aslc, NESCPU::asl), ("ANC",  NESCPU::imm, NESCPU::ancc, NESCPU::anc), ("NOP",  NESCPU::abs, NESCPU::nopc, NESCPU::nop), ("ORA",  NESCPU::abs, NESCPU::orac, NESCPU::ora), ("ASL",  NESCPU::abs, NESCPU::aslc, NESCPU::asl), ("SLO",  NESCPU::abs, NESCPU::sloc, NESCPU::slo),
        ("BPL", NESCPU::rel, NESCPU::bplc, NESCPU::bpl), ("ORA", NESCPU::indy, NESCPU::orac, NESCPU::ora), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("SLO", NESCPU::indy, NESCPU::sloc, NESCPU::slo), ("NOP", NESCPU::zpx, NESCPU::nopc, NESCPU::nop), ("ORA", NESCPU::zpx, NESCPU::orac, NESCPU::ora), ("ASL", NESCPU::zpx, NESCPU::aslc, NESCPU::asl), ("SLO", NESCPU::zpx, NESCPU::sloc, NESCPU::slo), ("CLC", NESCPU::imp, NESCPU::clcc, NESCPU::clc), ("ORA", NESCPU::absy, NESCPU::orac, NESCPU::ora), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("SLO", NESCPU::absy, NESCPU::sloc, NESCPU::slo), ("NOP", NESCPU::absx, NESCPU::nopc, NESCPU::nop), ("ORA", NESCPU::absx, NESCPU::orac, NESCPU::ora), ("ASL", NESCPU::absx, NESCPU::aslc, NESCPU::asl), ("SLO", NESCPU::absx, NESCPU::sloc, NESCPU::slo),
        ("JSR", NESCPU::abs, NESCPU::jsrc, NESCPU::jsr), ("AND", NESCPU::indx, NESCPU::andc, NESCPU::and), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("RLA", NESCPU::indx, NESCPU::rlac, NESCPU::rla), ("BIT",  NESCPU::zp, NESCPU::bitc, NESCPU::bit), ("AND",  NESCPU::zp, NESCPU::andc, NESCPU::and), ("ROL",  NESCPU::zp, NESCPU::rolc, NESCPU::rol), ("RLA",  NESCPU::zp, NESCPU::rlac, NESCPU::rla), ("PLP", NESCPU::imp, NESCPU::plpc, NESCPU::plp), ("AND",  NESCPU::imm, NESCPU::andc, NESCPU::and), ("ROL", NESCPU::acc, NESCPU::rolc, NESCPU::rol), ("ANC",  NESCPU::imm, NESCPU::ancc, NESCPU::anc), ("BIT",  NESCPU::abs, NESCPU::bitc, NESCPU::bit), ("AND",  NESCPU::abs, NESCPU::andc, NESCPU::and), ("ROL",  NESCPU::abs, NESCPU::rolc, NESCPU::rol), ("RLA",  NESCPU::abs, NESCPU::rlac, NESCPU::rla),
        ("BMI", NESCPU::rel, NESCPU::bmic, NESCPU::bmi), ("AND", NESCPU::indy, NESCPU::andc, NESCPU::and), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("RLA", NESCPU::indy, NESCPU::rlac, NESCPU::rla), ("NOP", NESCPU::zpx, NESCPU::nopc, NESCPU::nop), ("AND", NESCPU::zpx, NESCPU::andc, NESCPU::and), ("ROL", NESCPU::zpx, NESCPU::rolc, NESCPU::rol), ("RLA", NESCPU::zpx, NESCPU::rlac, NESCPU::rla), ("SEC", NESCPU::imp, NESCPU::secc, NESCPU::sec), ("AND", NESCPU::absy, NESCPU::andc, NESCPU::and), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("RLA", NESCPU::absy, NESCPU::rlac, NESCPU::rla), ("NOP", NESCPU::absx, NESCPU::nopc, NESCPU::nop), ("AND", NESCPU::absx, NESCPU::andc, NESCPU::and), ("ROL", NESCPU::absx, NESCPU::rolc, NESCPU::rol), ("RLA", NESCPU::absx, NESCPU::rlac, NESCPU::rla),
        ("RTI", NESCPU::imp, NESCPU::rtic, NESCPU::rti), ("EOR", NESCPU::indx, NESCPU::eorc, NESCPU::eor), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("SRE", NESCPU::indx, NESCPU::srec, NESCPU::sre), ("NOP",  NESCPU::zp, NESCPU::nopc, NESCPU::nop), ("EOR",  NESCPU::zp, NESCPU::eorc, NESCPU::eor), ("LSR",  NESCPU::zp, NESCPU::lsrc, NESCPU::lsr), ("SRE",  NESCPU::zp, NESCPU::srec, NESCPU::sre), ("PHA", NESCPU::imp, NESCPU::phac, NESCPU::pha), ("EOR",  NESCPU::imm, NESCPU::eorc, NESCPU::eor), ("LSR", NESCPU::acc, NESCPU::lsrc, NESCPU::lsr), ("ASR",  NESCPU::imm, NESCPU::asrc, NESCPU::asr), ("JMP",  NESCPU::abs, NESCPU::jmpc, NESCPU::jmp), ("EOR",  NESCPU::abs, NESCPU::eorc, NESCPU::eor), ("LSR",  NESCPU::abs, NESCPU::lsrc, NESCPU::lsr), ("SRE",  NESCPU::abs, NESCPU::srec, NESCPU::sre),
        ("BVC", NESCPU::rel, NESCPU::bvcc, NESCPU::bvc), ("EOR", NESCPU::indy, NESCPU::eorc, NESCPU::eor), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("SRE", NESCPU::indy, NESCPU::srec, NESCPU::sre), ("NOP", NESCPU::zpx, NESCPU::nopc, NESCPU::nop), ("EOR", NESCPU::zpx, NESCPU::eorc, NESCPU::eor), ("LSR", NESCPU::zpx, NESCPU::lsrc, NESCPU::lsr), ("SRE", NESCPU::zpx, NESCPU::srec, NESCPU::sre), ("CLI", NESCPU::imp, NESCPU::clic, NESCPU::cli), ("EOR", NESCPU::absy, NESCPU::eorc, NESCPU::eor), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("SRE", NESCPU::absy, NESCPU::srec, NESCPU::sre), ("NOP", NESCPU::absx, NESCPU::nopc, NESCPU::nop), ("EOR", NESCPU::absx, NESCPU::eorc, NESCPU::eor), ("LSR", NESCPU::absx, NESCPU::lsrc, NESCPU::lsr), ("SRE", NESCPU::absx, NESCPU::srec, NESCPU::sre),
        ("RTS", NESCPU::imp, NESCPU::rtsc, NESCPU::rts), ("ADC", NESCPU::indx, NESCPU::adcc, NESCPU::adc), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("RRA", NESCPU::indx, NESCPU::rrac, NESCPU::rra), ("NOP",  NESCPU::zp, NESCPU::nopc, NESCPU::nop), ("ADC",  NESCPU::zp, NESCPU::adcc, NESCPU::adc), ("ROR",  NESCPU::zp, NESCPU::rorc, NESCPU::ror), ("RRA",  NESCPU::zp, NESCPU::rrac, NESCPU::rra), ("PLA", NESCPU::imp, NESCPU::plac, NESCPU::pla), ("ADC",  NESCPU::imm, NESCPU::adcc, NESCPU::adc), ("ROR", NESCPU::acc, NESCPU::rorc, NESCPU::ror), ("ARR",  NESCPU::imm, NESCPU::arrc, NESCPU::arr), ("JMP",  NESCPU::ind, NESCPU::jmpc, NESCPU::jmp), ("ADC",  NESCPU::abs, NESCPU::adcc, NESCPU::adc), ("ROR",  NESCPU::abs, NESCPU::rorc, NESCPU::ror), ("RRA",  NESCPU::abs, NESCPU::rrac, NESCPU::rra),
        ("BVS", NESCPU::rel, NESCPU::bvsc, NESCPU::bvs), ("ADC", NESCPU::indy, NESCPU::adcc, NESCPU::adc), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("RRA", NESCPU::indy, NESCPU::rrac, NESCPU::rra), ("NOP", NESCPU::zpx, NESCPU::nopc, NESCPU::nop), ("ADC", NESCPU::zpx, NESCPU::adcc, NESCPU::adc), ("ROR", NESCPU::zpx, NESCPU::rorc, NESCPU::ror), ("RRA", NESCPU::zpx, NESCPU::rrac, NESCPU::rra), ("SEI", NESCPU::imp, NESCPU::seic, NESCPU::sei), ("ADC", NESCPU::absy, NESCPU::adcc, NESCPU::adc), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("RRA", NESCPU::absy, NESCPU::rrac, NESCPU::rra), ("NOP", NESCPU::absx, NESCPU::nopc, NESCPU::nop), ("ADC", NESCPU::absx, NESCPU::adcc, NESCPU::adc), ("ROR", NESCPU::absx, NESCPU::rorc, NESCPU::ror), ("RRA", NESCPU::absx, NESCPU::rrac, NESCPU::rra),
        ("NOP", NESCPU::imm, NESCPU::nopc, NESCPU::nop), ("STA", NESCPU::indx, NESCPU::stac, NESCPU::sta), ("NOP", NESCPU::imm, NESCPU::nopc, NESCPU::nop),  ("SAX", NESCPU::indx, NESCPU::saxc, NESCPU::sax), ("STY",  NESCPU::zp, NESCPU::styc, NESCPU::sty), ("STA",  NESCPU::zp, NESCPU::stac, NESCPU::sta), ("STX",  NESCPU::zp, NESCPU::stxc, NESCPU::stx), ("SAX",  NESCPU::zp, NESCPU::saxc, NESCPU::sax), ("DEY", NESCPU::imp, NESCPU::deyc, NESCPU::dey), ("NOP",  NESCPU::imm, NESCPU::nopc, NESCPU::nop), ("TXA", NESCPU::imp, NESCPU::txac, NESCPU::txa), ("XAA",  NESCPU::imm, NESCPU::xaac, NESCPU::xaa), ("STY",  NESCPU::abs, NESCPU::styc, NESCPU::sty), ("STA",  NESCPU::abs, NESCPU::stac, NESCPU::sta), ("STX",  NESCPU::abs, NESCPU::stxc, NESCPU::stx), ("SAX",  NESCPU::abs, NESCPU::saxc, NESCPU::sax),
        ("BCC", NESCPU::rel, NESCPU::bccc, NESCPU::bcc), ("STA", NESCPU::indy, NESCPU::stac, NESCPU::sta), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("SHA", NESCPU::indy, NESCPU::shac, NESCPU::sha), ("STY", NESCPU::zpx, NESCPU::styc, NESCPU::sty), ("STA", NESCPU::zpx, NESCPU::stac, NESCPU::sta), ("STX", NESCPU::zpy, NESCPU::stxc, NESCPU::stx), ("SAX", NESCPU::zpy, NESCPU::saxc, NESCPU::sax), ("TYA", NESCPU::imp, NESCPU::tyac, NESCPU::tya), ("STA", NESCPU::absy, NESCPU::stac, NESCPU::sta), ("TXS", NESCPU::imp, NESCPU::txsc, NESCPU::txs), ("SHS", NESCPU::absy, NESCPU::shsc, NESCPU::shs), ("SHY", NESCPU::absx, NESCPU::shyc, NESCPU::shy), ("STA", NESCPU::absx, NESCPU::stac, NESCPU::sta), ("SHX", NESCPU::absy, NESCPU::shxc, NESCPU::shx), ("SHA", NESCPU::absy, NESCPU::shac, NESCPU::sha),
        ("LDY", NESCPU::imm, NESCPU::ldyc, NESCPU::ldy), ("LDA", NESCPU::indx, NESCPU::ldac, NESCPU::lda), ("LDX", NESCPU::imm, NESCPU::ldxc, NESCPU::ldx),  ("LAX", NESCPU::indx, NESCPU::laxc, NESCPU::lax), ("LDY",  NESCPU::zp, NESCPU::ldyc, NESCPU::ldy), ("LDA",  NESCPU::zp, NESCPU::ldac, NESCPU::lda), ("LDX",  NESCPU::zp, NESCPU::ldxc, NESCPU::ldx), ("LAX",  NESCPU::zp, NESCPU::laxc, NESCPU::lax), ("TAY", NESCPU::imp, NESCPU::tayc, NESCPU::tay), ("LDA",  NESCPU::imm, NESCPU::ldac, NESCPU::lda), ("TAX", NESCPU::imp, NESCPU::taxc, NESCPU::tax), ("LAX",  NESCPU::imm, NESCPU::laxc, NESCPU::lax), ("LDY",  NESCPU::abs, NESCPU::ldyc, NESCPU::ldy), ("LDA",  NESCPU::abs, NESCPU::ldac, NESCPU::lda), ("LDX",  NESCPU::abs, NESCPU::ldxc, NESCPU::ldx), ("LAX",  NESCPU::abs, NESCPU::laxc, NESCPU::lax),
        ("BCS", NESCPU::rel, NESCPU::bcsc, NESCPU::bcs), ("LDA", NESCPU::indy, NESCPU::ldac, NESCPU::lda), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("LAX", NESCPU::indy, NESCPU::laxc, NESCPU::lax), ("LDY", NESCPU::zpx, NESCPU::ldyc, NESCPU::ldy), ("LDA", NESCPU::zpx, NESCPU::ldac, NESCPU::lda), ("LDX", NESCPU::zpy, NESCPU::ldxc, NESCPU::ldx), ("LAX", NESCPU::zpy, NESCPU::laxc, NESCPU::lax), ("CLV", NESCPU::imp, NESCPU::clvc, NESCPU::clv), ("LDA", NESCPU::absy, NESCPU::ldac, NESCPU::lda), ("TSX", NESCPU::imp, NESCPU::tsxc, NESCPU::tsx), ("LAS", NESCPU::absy, NESCPU::lasc, NESCPU::las), ("LDY", NESCPU::absx, NESCPU::ldyc, NESCPU::ldy), ("LDA", NESCPU::absx, NESCPU::ldac, NESCPU::lda), ("LDX", NESCPU::absy, NESCPU::ldxc, NESCPU::ldx), ("LAX", NESCPU::absy, NESCPU::laxc, NESCPU::lax),
        ("CPY", NESCPU::imm, NESCPU::cpyc, NESCPU::cpy), ("CMP", NESCPU::indx, NESCPU::cmpc, NESCPU::cmp), ("NOP", NESCPU::imm, NESCPU::nopc, NESCPU::nop),  ("DCP", NESCPU::indx, NESCPU::dcpc, NESCPU::dcp), ("CPY",  NESCPU::zp, NESCPU::cpyc, NESCPU::cpy), ("CMP",  NESCPU::zp, NESCPU::cmpc, NESCPU::cmp), ("DEC",  NESCPU::zp, NESCPU::decc, NESCPU::dec), ("DCP",  NESCPU::zp, NESCPU::dcpc, NESCPU::dcp), ("INY", NESCPU::imp, NESCPU::inyc, NESCPU::iny), ("CMP",  NESCPU::imm, NESCPU::cmpc, NESCPU::cmp), ("DEX", NESCPU::imp, NESCPU::dexc, NESCPU::dex), ("SBX",  NESCPU::imm, NESCPU::sbxc, NESCPU::sbx), ("CPY",  NESCPU::abs, NESCPU::cpyc, NESCPU::cpy), ("CMP",  NESCPU::abs, NESCPU::cmpc, NESCPU::cmp), ("DEC",  NESCPU::abs, NESCPU::decc, NESCPU::dec), ("DCP",  NESCPU::abs, NESCPU::dcpc, NESCPU::dcp),
        ("BNE", NESCPU::rel, NESCPU::bnec, NESCPU::bne), ("CMP", NESCPU::indy, NESCPU::cmpc, NESCPU::cmp), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("DCP", NESCPU::indy, NESCPU::dcpc, NESCPU::dcp), ("NOP", NESCPU::zpx, NESCPU::nopc, NESCPU::nop), ("CMP", NESCPU::zpx, NESCPU::cmpc, NESCPU::cmp), ("DEC", NESCPU::zpx, NESCPU::decc, NESCPU::dec), ("DCP", NESCPU::zpx, NESCPU::dcpc, NESCPU::dcp), ("CLD", NESCPU::imp, NESCPU::cldc, NESCPU::cld), ("CMP", NESCPU::absy, NESCPU::cmpc, NESCPU::cmp), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("DCP", NESCPU::absy, NESCPU::dcpc, NESCPU::dcp), ("NOP", NESCPU::absx, NESCPU::nopc, NESCPU::nop), ("CMP", NESCPU::absx, NESCPU::cmpc, NESCPU::cmp), ("DEC", NESCPU::absx, NESCPU::decc, NESCPU::dec), ("DCP", NESCPU::absx, NESCPU::dcpc, NESCPU::dcp),
        ("CPX", NESCPU::imm, NESCPU::cpxc, NESCPU::cpx), ("SBC", NESCPU::indx, NESCPU::sbcc, NESCPU::sbc), ("NOP", NESCPU::imm, NESCPU::nopc, NESCPU::nop),  ("ISC", NESCPU::indx, NESCPU::iscc, NESCPU::isc), ("CPX",  NESCPU::zp, NESCPU::cpxc, NESCPU::cpx), ("SBC",  NESCPU::zp, NESCPU::sbcc, NESCPU::sbc), ("INC",  NESCPU::zp, NESCPU::incc, NESCPU::inc), ("ISC",  NESCPU::zp, NESCPU::iscc, NESCPU::isc), ("INX", NESCPU::imp, NESCPU::inxc, NESCPU::inx), ("SBC",  NESCPU::imm, NESCPU::sbcc, NESCPU::sbc), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("SBC",  NESCPU::imm, NESCPU::sbcc, NESCPU::sbc), ("CPX",  NESCPU::abs, NESCPU::cpxc, NESCPU::cpx), ("SBC",  NESCPU::abs, NESCPU::sbcc, NESCPU::sbc), ("INC",  NESCPU::abs, NESCPU::incc, NESCPU::inc), ("ISC",  NESCPU::abs, NESCPU::iscc, NESCPU::isc),
        ("BEQ", NESCPU::rel, NESCPU::beqc, NESCPU::beq), ("SBC", NESCPU::indy, NESCPU::sbcc, NESCPU::sbc), ("JAM", NESCPU::imp, NESCPU::jamc, NESCPU::_jam), ("ISC", NESCPU::indy, NESCPU::iscc, NESCPU::isc), ("NOP", NESCPU::zpx, NESCPU::nopc, NESCPU::nop), ("SBC", NESCPU::zpx, NESCPU::sbcc, NESCPU::sbc), ("INC", NESCPU::zpx, NESCPU::incc, NESCPU::inc), ("ISC", NESCPU::zpx, NESCPU::iscc, NESCPU::isc), ("SED", NESCPU::imp, NESCPU::sedc, NESCPU::sed), ("SBC", NESCPU::absy, NESCPU::sbcc, NESCPU::sbc), ("NOP", NESCPU::imp, NESCPU::nopc, NESCPU::nop), ("ISC", NESCPU::absy, NESCPU::iscc, NESCPU::isc), ("NOP", NESCPU::absx, NESCPU::nopc, NESCPU::nop), ("SBC", NESCPU::absx, NESCPU::sbcc, NESCPU::sbc), ("INC", NESCPU::absx, NESCPU::incc, NESCPU::inc), ("ISC", NESCPU::absx, NESCPU::iscc, NESCPU::isc),
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
}

impl CPU for NESCPU {
    fn cpu_nmi(&mut self) {
        //Edge-detected
        self.pending_nmi = true;
    }

    fn cpu_irq(&mut self, interrupt: bool) {
        //Level-detected
        self.pending_irq = interrupt;
    }

    fn cpu_reset(&mut self) {
        self.current_instruction = CurrentInstruction {
            remaining_cycles: self.resetc(),
            instruction_type: InstructionType::Reset,
        }
    }

    fn clock(&mut self, bus: &mut dyn Bus) {
        self.elapsed_cycles += 1;
        self.current_instruction.remaining_cycles -= 1;

        if self.current_instruction.remaining_cycles == 0 {
            self.execute_operation(bus);
        }
    }

    fn cycles_remaining(&self) -> u8 {
        self.current_instruction.remaining_cycles
    }
}

impl Default for NESCPU {
    fn default() -> Self {
        Self::new()
    }
}

impl NESCPU {
    fn execute_operation(&mut self, bus: &mut dyn Bus) {
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

                let exec_fn = NESCPU::LOOKUP_TABLE[opcode as usize].3;
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

    fn poll_for_interrupts_or_fetch_next_instruction(&mut self, bus: &mut dyn Bus, i_flag: bool) {
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

    fn fetch_next_instruction(&mut self, bus: &mut dyn Bus) -> CurrentInstruction {
        let opcode = self.fetch_byte(bus);
        let (_, addr_mode_fn, cycles_fn, _) = NESCPU::LOOKUP_TABLE[opcode as usize];
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

    fn fetch_byte(&mut self, bus: &dyn Bus) -> u8 {
        let data = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        data
    }

    fn fetch_two_bytes_as_u16(&mut self, bus: &dyn Bus) -> u16 {
        let low_byte: u16 = bus.read(self.pc.wrapping_add(0)) as u16;
        let high_byte: u16 = bus.read(self.pc.wrapping_add(1)) as u16;
        self.pc = self.pc.wrapping_add(2);

        high_byte << 8 | low_byte
    }
}

#[cfg(test)]
mod cpu_tests {
    use mockall::predicate::eq;

    use crate::{
        bus::MockBus,
        cpu::{
            addr::{AddrModeResult, AddrModeType},
            CurrentInstruction, InstructionType, CPU, NESCPU,
        },
    };

    // Note this useful idiom: importing names from outer (for mod tests) scope.

    #[test]
    fn test_cpu_initial_state() {
        let cpu = NESCPU::new();

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
        let mut cpu = NESCPU::new();
        cpu.i = false;
        assert_eq!(0b0010_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;
        cpu.n = true;
        assert_eq!(0b1010_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_overflow_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;
        cpu.v = true;
        assert_eq!(0b0110_0000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_break_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;
        assert_eq!(0b0011_0000, cpu.get_status_byte(true))
    }

    #[test]
    fn test_get_status_byte_decimal_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;
        cpu.d = true;
        assert_eq!(0b0010_1000, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_interrupt_flag() {
        let cpu = NESCPU::new();
        assert_eq!(0b0010_0100, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;
        cpu.z = true;
        assert_eq!(0b0010_0010, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_carry_flag() {
        let mut cpu = NESCPU::new();
        cpu.i = false;
        cpu.c = true;
        assert_eq!(0b0010_0001, cpu.get_status_byte(false))
    }

    #[test]
    fn test_get_status_byte_all_flags() {
        let mut cpu = NESCPU::new();
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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .once()
            .return_const(0x40);

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
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
                        mode: AddrModeType::Imm,
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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        cpu.current_instruction = CurrentInstruction {
            remaining_cycles: 1,
            instruction_type: InstructionType::Instruction {
                opcode: 0x0,
                addr_mode: cpu._imp(),
            },
        };

        cpu.cpu_reset();

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).return_const(0x69);
        bus.expect_read().with(eq(0x2041)).return_const(0x69);
        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        cpu.cpu_nmi();

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).return_const(0x69);
        bus.expect_read().with(eq(0x2041)).return_const(0x69);

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        cpu.i = false;
        cpu.cpu_irq(true);
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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).return_const(0x69);
        bus.expect_read().with(eq(0x2041)).return_const(0x69);
        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        for _ in 0..7 {
            cpu.clock(&mut bus);
        }

        cpu.i = true;
        cpu.cpu_irq(true);

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .once()
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .once()
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).once().return_const(0x58); //CLI
        bus.expect_read().with(eq(0x2041)).once().return_const(0x58);

        bus.expect_read().return_const(0x0);

        cpu.cpu_irq(true);

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .once()
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .once()
            .return_const(0x20);
        bus.expect_read().with(eq(0x2040)).once().return_const(0x78); //SEI

        bus.expect_read().return_const(0x0);

        cpu.cpu_irq(true);

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .once()
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
            .once()
            .return_const(0x20);

        bus.expect_read().with(eq(0x2040)).once().return_const(0x28); //PLP
        bus.expect_read()
            .with(eq(0x1fe))
            .once()
            .return_const(0b1111_1111);

        bus.expect_read().return_const(0x0);

        cpu.cpu_irq(true);

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
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR))
            .once()
            .return_const(0x40);
        bus.expect_read()
            .with(eq(NESCPU::RESET_VECTOR + 1))
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

        cpu.cpu_nmi();

        for _ in 0..10_000 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            InstructionType::Jam,
            cpu.current_instruction.instruction_type
        );

        cpu.cpu_irq(true);

        for _ in 0..10_000 {
            cpu.clock(&mut bus);
        }

        assert_eq!(
            InstructionType::Jam,
            cpu.current_instruction.instruction_type
        );

        cpu.cpu_reset();

        assert_eq!(
            CurrentInstruction {
                remaining_cycles: 7,
                instruction_type: InstructionType::Reset
            },
            cpu.current_instruction
        );
    }
}
