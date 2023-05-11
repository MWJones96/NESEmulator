mod mapper_0;

pub trait PRGRomMapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub trait CHRRomMapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub trait Mapper: PRGRomMapper + CHRRomMapper {

}

pub fn mapper_factory(mapper: u8) -> Box<dyn Mapper> {
    match mapper {
        mapper => panic!("Mapper {mapper} has not been implemented")
    }
}