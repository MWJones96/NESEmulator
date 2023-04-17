use super::{CPU, bus::Bus};

mod acc;
mod imm;
mod zp;
mod zpx;
mod abs;
mod absx;
mod absy;
mod indx;
mod indy;
mod rel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::cpu) enum AddrMode {
    ACC,
    IMM,
    ZP,
    ZPX,
    ABS,
    ABSX,
    ABSY,
    INDX,
    INDY,
    REL
}

#[derive(Debug, PartialEq)]
pub(in crate::cpu) struct AddrModeResult {
    pub data: u8,
    pub cycles: u8,
    pub mode: AddrMode,
    //Address where the data came from (does not apply for ACC and IMM modes)
    pub addr: Option<u16>,
}

impl CPU {
    fn abs_helper(&self, addr: u16, register: u8, mode: AddrMode, bus: &dyn Bus) 
        -> AddrModeResult {
        let page_before: u8 = (addr >> 8) as u8;
        let resolved_addr = addr.wrapping_add(register as u16);
        let page_after: u8 = (resolved_addr >> 8) as u8;

        AddrModeResult {
            data: bus.read(resolved_addr),
            cycles: 2 + ((page_before != page_after) as u8),
            mode: mode,
            addr: Some(resolved_addr)
        }
    }
}