use super::{CHRRomMapper, PRGRomMapper};

pub struct Mapper0;

impl PRGRomMapper for Mapper0 {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}

impl CHRRomMapper for Mapper0 {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}