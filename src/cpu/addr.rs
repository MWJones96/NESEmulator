use super::{CPU, bus::Bus};

mod imm;
mod zp;
mod zpx;
mod abs;
mod absx;
mod absy;

impl CPU {
    fn abs_helper(&self, addr: u16, register: u8, bus: &dyn Bus) -> (u8, u8) {
        let page_before: u8 = (addr >> 8) as u8;
        let resolved_addr = addr.wrapping_add(register as u16);
        let page_after: u8 = (resolved_addr >> 8) as u8;

        (2 + ((page_before != page_after) as u8), bus.read(resolved_addr))
    }
}