use super::{CHRRomMapper, PRGRomMapper, Mapper};

pub struct Mapper0<'a> {
    prg_rom: &'a [u8],
    chr_rom: &'a [u8],
}

impl<'a> Mapper0<'a> {
    pub fn new(prg_rom: &'a [u8], chr_rom: &'a [u8]) -> Mapper0<'a> {
        assert!(prg_rom.len() % 16384 == 0);
        assert!(chr_rom.len() % 8192 == 0);

        Self {
            prg_rom,
            chr_rom,
        }
    }
}

impl Mapper for Mapper0<'_> {}

impl PRGRomMapper for Mapper0<'_> {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}

impl CHRRomMapper for Mapper0<'_> {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}
