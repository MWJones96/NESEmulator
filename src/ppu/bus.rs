use mockall::automock;

#[automock]
pub trait PPUBus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}
