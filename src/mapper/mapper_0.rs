use super::Mapper;

pub struct Mapper0;

impl Mapper0 {
    pub fn new() -> Mapper0 {
        Mapper0 {}
    }
}

impl Mapper for Mapper0 {
    fn read_prg(&self, addr: u16, prg_banks: u8) -> u16 {
        assert!((0x8000..=0xffff).contains(&addr));
        assert!((1..=2).contains(&prg_banks));
        let offset = addr - 0x8000;

        if prg_banks == 1 {
            offset & 0x3fff
        } else {
            offset & 0x7fff
        }
    }

    fn write_prg(&self, addr: u16, _data: u8, prg_banks: u8) {
        assert!((0x8000..=0xffff).contains(&addr));
        assert!((1..=2).contains(&prg_banks));
    }

    fn read_chr(&self, addr: u16, chr_banks: u8) -> u16 {
        assert!((0x0..=0x1fff).contains(&addr));
        assert!(chr_banks == 1);

        addr
    }

    fn write_chr(&self, addr: u16, data: u8, chr_banks: u8) {
        assert!((0x0..=0x1fff).contains(&addr));
        assert!(chr_banks == 1);
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
        assert_eq!(0x2, Mapper::read_prg(&mapper, 0x8002, 1));
        assert_eq!(0x2, Mapper::read_prg(&mapper, 0xC002, 1));
    }

    #[test]
    fn test_mapper0_two_prg_rom_banks() {
        let mut prg_rom: [u8; 2 * 16384] = [0; 2 * 16384];

        prg_rom[0] = 0xff;
        prg_rom[0x4000] = 0xee;

        let mapper = Mapper0::new();

        //Memory is not mirrored
        assert_eq!(0x0, Mapper::read_prg(&mapper, 0x8000, 2));
        assert_eq!(0x4000, Mapper::read_prg(&mapper, 0xC000, 2));
    }

    #[test]
    #[should_panic]
    fn test_mapper0_chr_read_out_of_range() {
        let mapper = Mapper0::new();
        mapper.read_chr(0x2000, 1);
    }

    #[test]
    #[should_panic]
    fn test_mapper0_chr_read_too_many_banks() {
        let mapper = Mapper0::new();
        mapper.read_chr(0x2000, 2);
    }

    #[test]
    #[should_panic]
    fn test_mapper0_chr_write_out_of_range() {
        let mapper = Mapper0::new();
        mapper.write_chr(0x2000, 0x0, 1);
    }

    #[test]
    #[should_panic]
    fn test_mapper0_chr_write_too_many_banks() {
        let mapper = Mapper0::new();
        mapper.write_chr(0x2000, 0x0, 2);
    }

    #[test]
    fn test_mapper0_chr_read() {
        let mapper = Mapper0::new();
        assert_eq!(0x1234, mapper.read_chr(0x1234, 1));
    }
}
