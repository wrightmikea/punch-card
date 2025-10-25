// EBCDIC Encoding Module
//
// Maps Hollerith punch patterns to EBCDIC character codes (80-column format)

use crate::hollerith::HollerithCode;

/// Convert a Hollerith pattern to an EBCDIC byte
///
/// Standard EBCDIC encoding for punch cards:
/// - Digits 0-9: 0xF0-0xF9
/// - Letters A-I: 0xC1-0xC9
/// - Letters J-R: 0xD1-0xD9
/// - Letters S-Z: 0xE2-0xE9
/// - Space: 0x40
pub fn hollerith_to_ebcdic(code: &HollerithCode) -> u8 {
    // Check for common patterns
    let rows = &code.rows;

    // Space (no punches)
    if rows.is_empty() {
        return 0x40;
    }

    // Single digit punches (0-9)
    if rows.len() == 1 {
        match rows[0] {
            0 => return 0xF0,  // '0'
            1 => return 0xF1,  // '1'
            2 => return 0xF2,  // '2'
            3 => return 0xF3,  // '3'
            4 => return 0xF4,  // '4'
            5 => return 0xF5,  // '5'
            6 => return 0xF6,  // '6'
            7 => return 0xF7,  // '7'
            8 => return 0xF8,  // '8'
            9 => return 0xF9,  // '9'
            12 => return 0x4C, // '&' ampersand
            11 => return 0x60, // '-' hyphen
            _ => return 0x40,  // default to space
        }
    }

    // Two punches (letters or special characters)
    // Note: rows are sorted, so we need to match in sorted order
    if rows.len() == 2 {
        match (rows[0], rows[1]) {
            // Letters A-I (1-9 + 12) - sorted order
            (1, 12) => return 0xC1, // 'A'
            (2, 12) => return 0xC2, // 'B'
            (3, 12) => return 0xC3, // 'C'
            (4, 12) => return 0xC4, // 'D'
            (5, 12) => return 0xC5, // 'E'
            (6, 12) => return 0xC6, // 'F'
            (7, 12) => return 0xC7, // 'G'
            (8, 12) => return 0xC8, // 'H'
            (9, 12) => return 0xC9, // 'I'

            // Letters J-R (1-9 + 11) - sorted order
            (1, 11) => return 0xD1, // 'J'
            (2, 11) => return 0xD2, // 'K'
            (3, 11) => return 0xD3, // 'L'
            (4, 11) => return 0xD4, // 'M'
            (5, 11) => return 0xD5, // 'N'
            (6, 11) => return 0xD6, // 'O'
            (7, 11) => return 0xD7, // 'P'
            (8, 11) => return 0xD8, // 'Q'
            (9, 11) => return 0xD9, // 'R'

            // Letters S-Z (0 + 2-9) - sorted order
            (0, 2) => return 0xE2, // 'S'
            (0, 3) => return 0xE3, // 'T'
            (0, 4) => return 0xE4, // 'U'
            (0, 5) => return 0xE5, // 'V'
            (0, 6) => return 0xE6, // 'W'
            (0, 7) => return 0xE7, // 'X'
            (0, 8) => return 0xE8, // 'Y'
            (0, 9) => return 0xE9, // 'Z'

            // Special characters
            (0, 1) => return 0x61, // '/' slash

            _ => return 0x40, // default to space
        }
    }

    // Three or more punches (special characters with overpunch)
    // For now, default to space for unsupported patterns
    0x40
}

/// Convert an EBCDIC byte to a Hollerith pattern
///
/// This is the inverse of hollerith_to_ebcdic
pub fn ebcdic_to_hollerith(byte: u8) -> HollerithCode {
    let rows = match byte {
        // Space
        0x40 => vec![],

        // Digits 0-9 (0xF0-0xF9)
        0xF0 => vec![0],
        0xF1 => vec![1],
        0xF2 => vec![2],
        0xF3 => vec![3],
        0xF4 => vec![4],
        0xF5 => vec![5],
        0xF6 => vec![6],
        0xF7 => vec![7],
        0xF8 => vec![8],
        0xF9 => vec![9],

        // Letters A-I (0xC1-0xC9)
        0xC1 => vec![12, 1], // 'A'
        0xC2 => vec![12, 2], // 'B'
        0xC3 => vec![12, 3], // 'C'
        0xC4 => vec![12, 4], // 'D'
        0xC5 => vec![12, 5], // 'E'
        0xC6 => vec![12, 6], // 'F'
        0xC7 => vec![12, 7], // 'G'
        0xC8 => vec![12, 8], // 'H'
        0xC9 => vec![12, 9], // 'I'

        // Letters J-R (0xD1-0xD9)
        0xD1 => vec![11, 1], // 'J'
        0xD2 => vec![11, 2], // 'K'
        0xD3 => vec![11, 3], // 'L'
        0xD4 => vec![11, 4], // 'M'
        0xD5 => vec![11, 5], // 'N'
        0xD6 => vec![11, 6], // 'O'
        0xD7 => vec![11, 7], // 'P'
        0xD8 => vec![11, 8], // 'Q'
        0xD9 => vec![11, 9], // 'R'

        // Letters S-Z (0xE2-0xE9)
        0xE2 => vec![0, 2], // 'S'
        0xE3 => vec![0, 3], // 'T'
        0xE4 => vec![0, 4], // 'U'
        0xE5 => vec![0, 5], // 'V'
        0xE6 => vec![0, 6], // 'W'
        0xE7 => vec![0, 7], // 'X'
        0xE8 => vec![0, 8], // 'Y'
        0xE9 => vec![0, 9], // 'Z'

        // Special characters
        0x4C => vec![12],   // '&' ampersand
        0x60 => vec![11],   // '-' hyphen
        0x61 => vec![0, 1], // '/' slash

        // Default to space for unknown codes
        _ => vec![],
    };

    HollerithCode::new(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebcdic_space() {
        let code = HollerithCode::empty();
        assert_eq!(hollerith_to_ebcdic(&code), 0x40);

        let decoded = ebcdic_to_hollerith(0x40);
        assert_eq!(decoded.rows.len(), 0);
    }

    #[test]
    fn test_ebcdic_digits() {
        // Test '0' (0xF0 -> row 0)
        let code_0 = HollerithCode::new(vec![0]);
        assert_eq!(hollerith_to_ebcdic(&code_0), 0xF0);

        // Test '5' (0xF5 -> row 5)
        let code_5 = HollerithCode::new(vec![5]);
        assert_eq!(hollerith_to_ebcdic(&code_5), 0xF5);

        // Test '9' (0xF9 -> row 9)
        let code_9 = HollerithCode::new(vec![9]);
        assert_eq!(hollerith_to_ebcdic(&code_9), 0xF9);
    }

    #[test]
    fn test_ebcdic_letters_a_i() {
        // Test 'A' (0xC1 -> rows 12+1)
        let code_a = HollerithCode::new(vec![12, 1]);
        assert_eq!(hollerith_to_ebcdic(&code_a), 0xC1);

        // Test 'E' (0xC5 -> rows 12+5)
        let code_e = HollerithCode::new(vec![12, 5]);
        assert_eq!(hollerith_to_ebcdic(&code_e), 0xC5);
    }

    #[test]
    fn test_ebcdic_letters_j_r() {
        // Test 'J' (0xD1 -> rows 11+1)
        let code_j = HollerithCode::new(vec![11, 1]);
        assert_eq!(hollerith_to_ebcdic(&code_j), 0xD1);
    }

    #[test]
    fn test_ebcdic_letters_s_z() {
        // Test 'S' (0xE2 -> rows 0+2)
        let code_s = HollerithCode::new(vec![0, 2]);
        assert_eq!(hollerith_to_ebcdic(&code_s), 0xE2);

        // Test 'Z' (0xE9 -> rows 0+9)
        let code_z = HollerithCode::new(vec![0, 9]);
        assert_eq!(hollerith_to_ebcdic(&code_z), 0xE9);
    }

    #[test]
    fn test_ebcdic_roundtrip() {
        // Test A-Z
        for ebcdic in [
            0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5,
            0xD6, 0xD7, 0xD8, 0xD9, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9,
        ] {
            let hollerith = ebcdic_to_hollerith(ebcdic);
            let result = hollerith_to_ebcdic(&hollerith);
            assert_eq!(result, ebcdic, "Roundtrip failed for 0x{:02X}", ebcdic);
        }

        // Test 0-9
        for ebcdic in 0xF0..=0xF9 {
            let hollerith = ebcdic_to_hollerith(ebcdic);
            let result = hollerith_to_ebcdic(&hollerith);
            assert_eq!(result, ebcdic, "Roundtrip failed for 0x{:02X}", ebcdic);
        }
    }
}
