pub struct PPU {
    screen: [[u8; 256]; 240],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            //Black screen
            screen: [[0x0; 256]; 240],
        }
    }

    pub fn get_screen(&self) -> &[[u8; 256]; 240] {
        &self.screen
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod ppu_tests {
    use super::*;

    #[test]
    fn test_get_screen() {
        let ppu = PPU::new();
        let expected = &[[0x0u8; 256]; 240];

        assert_eq!(expected, ppu.get_screen());
    }
}
