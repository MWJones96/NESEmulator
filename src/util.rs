use std::fs;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug, PartialEq)]
pub struct INESHeader {
    pub prg_rom_banks: u8,
    pub chr_rom_banks: u8,
    pub mapper_num: u8,
    pub mirroring: Mirroring,
    pub battery: bool,
    pub trainer: bool,
    pub four_screen_vram: bool,
}

pub fn read_bytes_from_file(file_path: String) -> Vec<u8> {
    assert!(file_path.ends_with(".nes"));

    let bytes = fs::read(file_path).expect("File not found!");

    assert!(!bytes.is_empty());
    assert!(bytes[0..4] == [b'N', b'E', b'S', 0x1A]);

    bytes
}

pub fn extract_header(bytes: &[u8]) -> INESHeader {
    assert!(bytes[0..4] == [b'N', b'E', b'S', 0x1A]);

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

pub fn extract_prg_rom<'a>(header: &INESHeader, bytes: &'a [u8]) -> &'a [u8] {
    assert!(bytes[0..4] == [b'N', b'E', b'S', 0x1A]);

    let start = 16 + (header.trainer as usize) * 512;
    let end = start + (header.prg_rom_banks as usize) * 16384;

    &bytes[start..end]
}

pub fn extract_chr_rom<'a>(header: &INESHeader, bytes: &'a [u8]) -> &'a [u8] {
    assert!(bytes[0..4] == [b'N', b'E', b'S', 0x1A]);

    let start = 16 + (header.trainer as usize) * 512 + (header.prg_rom_banks as usize) * 16384;
    let end = start + (header.chr_rom_banks as usize) * 8192;

    &bytes[start..end]
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_read_bytes_from_test_file() {
        let bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());

        assert_eq!(24592, bytes.len());
        assert_eq!(['N' as u8, 'E' as u8, 'S' as u8, 0x1A], &bytes[0..4]);
        assert_eq!(0x1A, bytes[3]);
    }

    #[test]
    #[should_panic]
    fn test_extract_header_without_nes_prefix() {
        extract_header(&[0; 16]);
    }

    #[test]
    fn test_extract_header_from_test_rom() {
        let bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
        let header = extract_header(&bytes);

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

    #[test]
    fn test_extract_prg_rom_no_trainer() {
        let mut bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
        bytes[16] = 0xff;
        bytes[16 + 16383] = 0xff;

        let header = extract_header(&bytes);
        let prg_rom = extract_prg_rom(&header, &bytes);

        assert_eq!(16384, prg_rom.len());
        assert_eq!(0xff, prg_rom[0]);
        assert_eq!(0xff, prg_rom[16383]);
    }

    #[test]
    fn test_extract_prg_rom_with_two_banks_no_trainer() {
        let header = INESHeader {
            prg_rom_banks: 2,
            chr_rom_banks: 1,
            mapper_num: 0,
            mirroring: Mirroring::HORIZONTAL,
            battery: false,
            trainer: false,
            four_screen_vram: false,
        };

        let mut bytes: [u8; 32784] = [0; 16 + 2 * 16384];
        bytes[0] = 'N' as u8;
        bytes[1] = 'E' as u8;
        bytes[2] = 'S' as u8;
        bytes[3] = 0x1a;

        bytes[16] = 0xff;
        bytes[32783] = 0xff;

        let prg_rom = extract_prg_rom(&header, &bytes);
        assert_eq!(2 * 16384, prg_rom.len());
        assert_eq!(0xff, prg_rom[0]);
        assert_eq!(0xff, prg_rom[2 * 16384 - 1]);
    }

    #[test]
    fn test_extract_prg_with_trainer() {
        let header = INESHeader {
            prg_rom_banks: 1,
            chr_rom_banks: 1,
            mapper_num: 0,
            mirroring: Mirroring::HORIZONTAL,
            battery: false,
            trainer: true,
            four_screen_vram: false,
        };

        let mut bytes: [u8; 16 + 512 + 16384] = [0; 16 + 512 + 16384];
        bytes[0] = 'N' as u8;
        bytes[1] = 'E' as u8;
        bytes[2] = 'S' as u8;
        bytes[3] = 0x1a;

        bytes[16 + 512] = 0xff;
        bytes[16 + 512 + 16383] = 0xff;

        let prg_rom = extract_prg_rom(&header, &bytes);
        assert_eq!(16384, prg_rom.len());
        assert_eq!(0xff, prg_rom[0]);
        assert_eq!(0xff, prg_rom[16383]);
    }

    #[test]
    #[should_panic]
    fn test_extract_chr_rom_fails_on_bad_header() {
        let mut bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
        let header = extract_header(&bytes);

        bytes[0] = 0;
        extract_chr_rom(&header, &bytes);
    }

    #[test]
    fn test_extract_chr_rom_with_no_trainer() {
        let mut bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
        let header = extract_header(&bytes);

        bytes[16 + 16384] = 0xff;
        bytes[16 + 16384 + 8191] = 0xff;

        let chr_rom = extract_chr_rom(&header, &bytes);
        assert_eq!(0xff, chr_rom[0]);
        assert_eq!(0xff, chr_rom[8191]);
    }

    #[test]
    fn test_extract_rom_with_trainer() {
        let header = INESHeader {
            prg_rom_banks: 2,
            chr_rom_banks: 2,
            mapper_num: 0,
            mirroring: Mirroring::HORIZONTAL,
            battery: false,
            trainer: true,
            four_screen_vram: false,
        };

        let mut bytes: [u8; 16 + 512 + 2 * 16384 + 2 * 8192] = [0; 16 + 512 + 2 * 16384 + 2 * 8192];
        bytes[0] = 'N' as u8;
        bytes[1] = 'E' as u8;
        bytes[2] = 'S' as u8;
        bytes[3] = 0x1a;

        bytes[16 + 512 + 2 * 16384] = 0xff;
        bytes[16 + 512 + 2 * 16384 + 2 * 8192 - 1] = 0xff;

        let chr_rom = extract_chr_rom(&header, &bytes);
        assert_eq!(2 * 8192, chr_rom.len());
        assert_eq!(0xff, chr_rom[0]);
        assert_eq!(0xff, chr_rom[2 * 8192 - 1]);
    }
}
