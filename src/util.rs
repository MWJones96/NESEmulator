use std::fs;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug, PartialEq)]
pub struct INESHeader {
    prg_rom_banks: u8,
    chr_rom_banks: u8,
    mapper_num: u8,
    mirroring: Mirroring,
    battery: bool,
    trainer: bool,
    four_screen_vram: bool,
}

pub fn read_bytes(file_path: String) -> Vec<u8> {
    assert!(file_path.ends_with(".nes"));
    fs::read(file_path).expect("File not found!")
}

pub fn extract_header(bytes: &[u8; 16]) -> INESHeader {
    assert!(&bytes[0..4] == ['N' as u8, 'E' as u8, 'S' as u8, 0x1A]);

    INESHeader {
        prg_rom_banks: bytes[4],
        chr_rom_banks: bytes[5],
        mapper_num: (bytes[7] & 0xf0) | ((bytes[6] & 0xf0) >> 4),
        mirroring: if (bytes[6] & 0x1) == 0 {
            Mirroring::HORIZONTAL
        } else {
            Mirroring::VERTICAL
        },
        battery: (bytes[6] & 0x2) != 0,
        trainer: (bytes[6] & 0x4) != 0,
        four_screen_vram: (bytes[6] & 0x8) != 0,
    }
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_read_bytes_from_test_file() {
        let bytes = read_bytes("tests/roms/nestest.nes".to_owned());

        assert_eq!(24592, bytes.len());
        assert_eq!('N' as u8, bytes[0]);
        assert_eq!('E' as u8, bytes[1]);
        assert_eq!('S' as u8, bytes[2]);
        assert_eq!(0x1A, bytes[3]);
    }

    #[test]
    #[should_panic]
    fn test_extract_header_without_nes_prefix() {
        extract_header(&[0; 16]);
    }

    #[test]
    fn test_extract_header_from_test_rom() {
        let bytes = read_bytes("tests/roms/nestest.nes".to_owned());
        let header = extract_header(&bytes[0..16].try_into().unwrap());

        assert_eq!(
            INESHeader {
                prg_rom_banks: 1,
                chr_rom_banks: 1,
                mapper_num: 0,
                mirroring: Mirroring::HORIZONTAL,
                battery: false,
                trainer: false,
                four_screen_vram: false,
            },
            header
        );
    }
}
