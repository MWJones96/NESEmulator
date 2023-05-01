use mockall::automock;

#[automock]
pub trait CPUBus {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, data: u8);
}

#[automock]
pub trait PPUBus {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, data: u8);
}