use super::Mapper;

pub struct Mapper0 {}

impl Mapper0 {
    pub fn new() -> Mapper0 {
        Mapper0 {}
    }
}

impl Mapper for Mapper0 {
    fn read(&self, addr: u16, prg_banks: u8) -> u16 {
        assert!((0x8000..=0xffff).contains(&addr));
        let offset = addr - 0x8000;

        if prg_banks == 1 {
            offset & 0x3fff
        } else {
            offset & 0x7fff
        }
    }

    fn write(&self, _addr: u16, _data: u8, _prg_banks: u8) {
        assert!((0x8000..=0xffff).contains(&_addr));
    }
}

#[cfg(test)]
mod mapper0_tests {
    use super::*;

    #[test]
    fn test_mapper0_one_prg_rom_bank() {
        let mut prg_rom: [u8; 16384] = [0; 16384];

        prg_rom[0] = 0xff;

        let mapper = Mapper0::new();

        //Memory is mirrored
        assert_eq!(0x2, Mapper::read(&mapper, 0x8002, 1));
        assert_eq!(0x2, Mapper::read(&mapper, 0xC002, 1))
    }

    #[test]
    fn test_mapper0_two_prg_rom_banks() {
        let mut prg_rom: [u8; 2 * 16384] = [0; 2 * 16384];

        prg_rom[0] = 0xff;
        prg_rom[0x4000] = 0xee;

        let mapper = Mapper0::new();

        //Memory is not mirrored
        assert_eq!(0x0, Mapper::read(&mapper, 0x8000, 2));
        assert_eq!(0x4000, Mapper::read(&mapper, 0xC000, 2))
    }
}
