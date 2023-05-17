use super::{Mapper, PRGRomMapper};

pub struct Mapper0<'a> {
    prg_rom: &'a [u8],
}

impl<'a> Mapper0<'a> {
    pub fn new(prg_rom: &'a [u8]) -> Mapper0<'a> {
        assert!(prg_rom.len() % 16384 == 0);

        Self { prg_rom }
    }
}

impl Mapper for Mapper0<'_> {}

impl PRGRomMapper for Mapper0<'_> {
    fn read(&self, addr: u16) -> u8 {
        assert!((0x8000..=0xffff).contains(&addr));
        let offset = addr - 0x8000;
        let num_banks = (self.prg_rom.len() / 16384) as u8;

        let prg_rom_addr = if num_banks == 1 {
            (offset & 0x3fff) as usize
        } else {
            (offset & 0x7fff) as usize
        };

        self.prg_rom[prg_rom_addr]
    }

    fn write(&mut self, _addr: u16, _data: u8) {}
}

#[cfg(test)]
mod mapper0_tests {
    use super::*;

    #[test]
    fn test_mapper0_one_prg_rom_bank() {
        let mut prg_rom: [u8; 16384] = [0; 16384];

        prg_rom[0] = 0xff;

        let mapper = Mapper0::new(&prg_rom);

        //Memory is mirrored
        assert_eq!(0xff, PRGRomMapper::read(&mapper, 0x8000));
        assert_eq!(0xff, PRGRomMapper::read(&mapper, 0xC000))
    }

    #[test]
    fn test_mapper0_two_prg_rom_banks() {
        let mut prg_rom: [u8; 2 * 16384] = [0; 2 * 16384];

        prg_rom[0] = 0xff;
        prg_rom[0x4000] = 0xee;

        let mapper = Mapper0::new(&prg_rom);

        //Memory is not mirrored
        assert_eq!(0xff, PRGRomMapper::read(&mapper, 0x8000));
        assert_eq!(0xee, PRGRomMapper::read(&mapper, 0xC000))
    }
}
