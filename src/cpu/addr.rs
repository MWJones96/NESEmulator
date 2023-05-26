mod abs;
mod absx;
mod absy;
mod acc;
mod imm;
mod imp;
mod ind;
mod indx;
mod indy;
mod rel;
mod zp;
mod zpx;
mod zpy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::cpu) enum AddrModeType {
    ACC,
    IMM,
    ZP,
    ZPX,
    ZPY,
    ABS,
    ABSX,
    ABSY,
    IND,
    INDX,
    INDY,
    REL,
    IMP,
}

pub(in crate::cpu) trait AddrMode {
    fn addr_mode_cycles(&self) -> u8;
    fn num_bytes(&self) -> u8;
    fn operands(&self) -> Vec<u8>;
    fn mode(&self) -> AddrModeType;

    fn get_operands_string(&self) -> String;
    fn get_addr_mode_string(&self) -> String;
}

pub(in crate::cpu) trait DataAccessAddrMode: AddrMode {
    fn data(&self) -> u8;
}

pub(in crate::cpu) trait AddrAccessAddrMode: AddrMode {
    fn addr(&self) -> u16;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(in crate::cpu) struct AddrModeResult {
    pub addr: Option<u16>,
    pub data: Option<u8>,
    pub cycles: u8,
    pub mode: AddrModeType,
    pub bytes: u8,
}
