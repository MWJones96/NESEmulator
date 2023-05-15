use std::rc::Rc;
use mockall::automock;
use self::mapper_0::Mapper0;

mod mapper_0;

pub trait Mapper: PRGRomMapper + CHRRomMapper {}

#[automock]
pub trait PRGRomMapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

#[automock]
pub trait CHRRomMapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub fn mapper_factory<'a>(mapper: u8, prg_rom: &'a [u8], chr_rom: &'a [u8]) -> Rc<impl Mapper + 'a> {
    match mapper {
        0 => Rc::new(Mapper0::new(prg_rom, chr_rom)),
        mapper => panic!("Mapper {mapper} has not been implemented"),
    }
}

#[cfg(test)]
mod mapper_tests {
    use super::*;

    #[test]
    fn test_mapper_factory_with_mapper0() {
        mapper_factory(0, &[], &[]);
    }

    #[test]
    #[should_panic]
    fn test_mapper_factory_with_unimplemented_mapper() {
        mapper_factory(0xff, &[], &[]);
    }
}
