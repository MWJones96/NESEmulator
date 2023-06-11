use self::mapper_0::Mapper0;
use mockall::automock;

mod mapper_0;

#[automock]
pub trait Mapper {
    fn read(&self, addr: u16, prg_banks: u8) -> u16;
    fn write(&self, addr: u16, data: u8, prg_banks: u8);
}

pub fn mapper_factory(mapper: u8) -> impl Mapper {
    match mapper {
        0 => Mapper0::new(),
        mapper => panic!("Mapper {mapper} has not been implemented"),
    }
}

#[cfg(test)]
mod mapper_tests {
    use super::*;

    #[test]
    fn test_mapper_factory_with_mapper0() {
        mapper_factory(0);
    }

    #[test]
    #[should_panic]
    fn test_mapper_factory_with_unimplemented_mapper() {
        mapper_factory(0xff);
    }
}
