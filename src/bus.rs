use mockall::automock;

pub mod cpu_bus;
pub mod ppu_bus;

#[automock]
pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}
