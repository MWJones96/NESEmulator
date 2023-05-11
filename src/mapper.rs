use self::mapper_0::Mapper0;

mod mapper_0;

pub trait Mapper: PRGRomMapper + CHRRomMapper {}

pub trait PRGRomMapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub trait CHRRomMapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub fn mapper_factory(mapper: u8) -> Box<dyn Mapper> {
    match mapper {
        0 => Box::new(Mapper0),
        mapper => panic!("Mapper {mapper} has not been implemented")
    }
}