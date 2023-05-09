use std::fs;

pub fn read_bytes(file_path: String) -> Vec<u8> {
    fs::read(file_path).expect("File not found!")
}

#[cfg(test)]
mod file_reader_tests {
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
}