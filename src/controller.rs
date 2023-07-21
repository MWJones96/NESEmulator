use mockall::automock;
use std::cell::RefCell;

#[automock]
pub trait Controller {
    fn read(&self) -> u8;
    fn write(&mut self, data: u8);
}

pub struct NESController {
    buffer: RefCell<u8>,
    polling: bool,

    pub a_latch: bool,
    pub b_latch: bool,

    pub select_latch: bool,
    pub start_latch: bool,

    pub up_latch: bool,
    pub down_latch: bool,
    pub left_latch: bool,
    pub right_latch: bool,
}

impl NESController {
    pub fn new() -> Self {
        NESController {
            buffer: RefCell::new(0x0),
            polling: false,

            a_latch: false,
            b_latch: false,

            select_latch: false,
            start_latch: false,

            up_latch: false,
            down_latch: false,
            left_latch: false,
            right_latch: false,
        }
    }
}

impl Default for NESController {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller for NESController {
    fn read(&self) -> u8 {
        let buffer_data = *self.buffer.borrow();
        *self.buffer.borrow_mut() = buffer_data >> 1;

        buffer_data & 0x1
    }

    fn write(&mut self, data: u8) {
        if !self.polling && (data & 0x1) != 0 {
            self.polling = true;
        } else if self.polling && (data & 0x1) == 0 {
            self.polling = false;

            let buffer_data = (self.right_latch as u8) << 7
                | (self.left_latch as u8) << 6
                | (self.down_latch as u8) << 5
                | (self.up_latch as u8) << 4
                | (self.start_latch as u8) << 3
                | (self.select_latch as u8) << 2
                | (self.b_latch as u8) << 1
                | self.a_latch as u8;

            *self.buffer.borrow_mut() = buffer_data;
        }
    }
}

#[cfg(test)]
mod controller_tests {
    use super::*;

    #[test]
    fn test_controller_write() {
        let mut controller = NESController::new();
        controller.a_latch = true;
        controller.select_latch = true;
        controller.up_latch = true;
        controller.left_latch = true;

        controller.write(0x1);
        assert_eq!(true, controller.polling);

        controller.write(0x1);
        assert_eq!(true, controller.polling);

        controller.write(0x0);
        assert_eq!(false, controller.polling);

        assert_eq!(0b0101_0101, *controller.buffer.borrow());
    }

    #[test]
    fn test_controller_read() {
        let controller = NESController::new();
        *controller.buffer.borrow_mut() = 0b0101_0101;

        assert_eq!(0x1, controller.read());
        assert_eq!(0x0, controller.read());
        assert_eq!(0x1, controller.read());
        assert_eq!(0x0, controller.read());
        assert_eq!(0x1, controller.read());
        assert_eq!(0x0, controller.read());
        assert_eq!(0x1, controller.read());
        assert_eq!(0x0, controller.read());

        assert_eq!(0x0, *controller.buffer.borrow());
    }
}
