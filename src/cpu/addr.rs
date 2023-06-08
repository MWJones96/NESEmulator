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
    Acc,
    Imm,
    Zp,
    Zpx,
    Zpy,
    Abs,
    Absx,
    Absy,
    Ind,
    Indx,
    Indy,
    Rel,
    Imp,
}

#[derive(Debug, PartialEq, Clone)]
pub(in crate::cpu) struct AddrModeResult {
    pub addr: Option<u16>,
    pub data: Option<u8>,
    pub cycles: u8,
    pub mode: AddrModeType,
    pub bytes: u8,
    pub operands: String,
    pub repr: String,
}
