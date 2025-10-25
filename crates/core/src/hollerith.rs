// Hollerith Encoding Module
//
// Implements the Hollerith punch card encoding system used by IBM 029 keypunch.
// Supports 64 printable characters with zone (12, 11, 0) and numeric (1-9) punches.

use serde::{Deserialize, Serialize};

/// Represents a Hollerith punch pattern for one column of a punch card
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HollerithCode {
    /// The rows that are punched (12, 11, 0-9)
    /// Row 12 is represented as 12, row 11 as 11, rows 0-9 as their numeric value
    pub rows: Vec<u8>,
}

impl HollerithCode {
    /// Create a new HollerithCode with the specified punched rows
    pub fn new(rows: Vec<u8>) -> Self {
        let mut sorted_rows = rows;
        sorted_rows.sort();
        sorted_rows.dedup();
        HollerithCode { rows: sorted_rows }
    }

    /// Create an empty HollerithCode (no punches - represents space/blank)
    pub fn empty() -> Self {
        HollerithCode { rows: Vec::new() }
    }

    /// Check if a specific row is punched
    pub fn is_punched(&self, row: u8) -> bool {
        self.rows.contains(&row)
    }

    /// Get the punches as a 12-element boolean array (index 0=row 12, 1=row 11, 2=row 0, 3-11=rows 1-9)
    pub fn as_array(&self) -> [bool; 12] {
        let mut arr = [false; 12];
        for &row in &self.rows {
            let idx = match row {
                12 => 0,
                11 => 1,
                0 => 2,
                1..=9 => (row + 2) as usize,
                _ => continue,
            };
            arr[idx] = true;
        }
        arr
    }

    /// Create a HollerithCode from a 12-element boolean array
    pub fn from_array(arr: [bool; 12]) -> Self {
        let mut rows = Vec::new();
        for (idx, &punched) in arr.iter().enumerate() {
            if punched {
                let row = match idx {
                    0 => 12,
                    1 => 11,
                    2 => 0,
                    3..=11 => (idx - 2) as u8,
                    _ => continue,
                };
                rows.push(row);
            }
        }
        HollerithCode::new(rows)
    }
}

/// Convert a character to its Hollerith encoding
///
/// Based on IBM 029 keypunch encoding table
/// Returns None for unsupported characters
pub fn char_to_hollerith(c: char) -> Option<HollerithCode> {
    let code = match c {
        // Digits (numeric punch only)
        '0' => vec![0],
        '1' => vec![1],
        '2' => vec![2],
        '3' => vec![3],
        '4' => vec![4],
        '5' => vec![5],
        '6' => vec![6],
        '7' => vec![7],
        '8' => vec![8],
        '9' => vec![9],

        // Letters A-I (12 zone + numeric)
        'A' => vec![12, 1],
        'B' => vec![12, 2],
        'C' => vec![12, 3],
        'D' => vec![12, 4],
        'E' => vec![12, 5],
        'F' => vec![12, 6],
        'G' => vec![12, 7],
        'H' => vec![12, 8],
        'I' => vec![12, 9],

        // Letters J-R (11 zone + numeric)
        'J' => vec![11, 1],
        'K' => vec![11, 2],
        'L' => vec![11, 3],
        'M' => vec![11, 4],
        'N' => vec![11, 5],
        'O' => vec![11, 6],
        'P' => vec![11, 7],
        'Q' => vec![11, 8],
        'R' => vec![11, 9],

        // Letters S-Z (0 zone + numeric)
        'S' => vec![0, 2],
        'T' => vec![0, 3],
        'U' => vec![0, 4],
        'V' => vec![0, 5],
        'W' => vec![0, 6],
        'X' => vec![0, 7],
        'Y' => vec![0, 8],
        'Z' => vec![0, 9],

        // Special characters
        ' ' => vec![],     // blank (no punches)
        '&' => vec![12],   // ampersand
        '-' => vec![11],   // minus/hyphen
        '/' => vec![0, 1], // slash

        // Special characters with 8 punch
        '.' => vec![12, 3, 8], // period
        '<' => vec![12, 4, 8], // less than
        '(' => vec![12, 5, 8], // left paren
        '+' => vec![12, 6, 8], // plus
        '|' => vec![12, 7, 8], // vertical bar

        '!' => vec![11, 2, 8], // exclamation
        '$' => vec![11, 3, 8], // dollar
        '*' => vec![11, 4, 8], // asterisk
        ')' => vec![11, 5, 8], // right paren
        ';' => vec![11, 6, 8], // semicolon
        '¬' => vec![11, 7, 8], // logical not

        ',' => vec![0, 3, 8], // comma
        '%' => vec![0, 4, 8], // percent
        '_' => vec![0, 5, 8], // underscore
        '>' => vec![0, 6, 8], // greater than
        '?' => vec![0, 7, 8], // question mark

        ':' => vec![2, 8],  // colon
        '#' => vec![3, 8],  // hash/pound
        '@' => vec![4, 8],  // at sign
        '\'' => vec![5, 8], // apostrophe
        '=' => vec![6, 8],  // equals
        '"' => vec![7, 8],  // quote

        _ => return None,
    };

    Some(HollerithCode::new(code))
}

/// Convert a Hollerith encoding to its character representation
///
/// Returns None for invalid or unsupported punch patterns
pub fn hollerith_to_char(code: &HollerithCode) -> Option<char> {
    // Handle empty (space)
    if code.rows.is_empty() {
        return Some(' ');
    }

    // Match against known patterns
    let rows = &code.rows;

    // Single punches (digits and zone punches)
    if rows.len() == 1 {
        return match rows[0] {
            0 => Some('0'),
            1 => Some('1'),
            2 => Some('2'),
            3 => Some('3'),
            4 => Some('4'),
            5 => Some('5'),
            6 => Some('6'),
            7 => Some('7'),
            8 => Some('8'),
            9 => Some('9'),
            11 => Some('-'),
            12 => Some('&'),
            _ => None,
        };
    }

    // Two punches (rows are sorted)
    if rows.len() == 2 {
        return match (rows[0], rows[1]) {
            // 12 zone letters (A-I) - sorted order: (1..9, 12)
            (1, 12) => Some('A'),
            (2, 12) => Some('B'),
            (3, 12) => Some('C'),
            (4, 12) => Some('D'),
            (5, 12) => Some('E'),
            (6, 12) => Some('F'),
            (7, 12) => Some('G'),
            (8, 12) => Some('H'),
            (9, 12) => Some('I'),

            // 11 zone letters (J-R) - sorted order: (1..9, 11)
            (1, 11) => Some('J'),
            (2, 11) => Some('K'),
            (3, 11) => Some('L'),
            (4, 11) => Some('M'),
            (5, 11) => Some('N'),
            (6, 11) => Some('O'),
            (7, 11) => Some('P'),
            (8, 11) => Some('Q'),
            (9, 11) => Some('R'),

            // 0 zone letters (S-Z) - sorted order: (0, 2..9)
            (0, 2) => Some('S'),
            (0, 3) => Some('T'),
            (0, 4) => Some('U'),
            (0, 5) => Some('V'),
            (0, 6) => Some('W'),
            (0, 7) => Some('X'),
            (0, 8) => Some('Y'),
            (0, 9) => Some('Z'),

            // Special two-punch characters
            (0, 1) => Some('/'),
            (2, 8) => Some(':'),
            (3, 8) => Some('#'),
            (4, 8) => Some('@'),
            (5, 8) => Some('\''),
            (6, 8) => Some('='),
            (7, 8) => Some('"'),

            _ => None,
        };
    }

    // Three punches (special characters with 8) - rows are sorted
    if rows.len() == 3 {
        return match (rows[0], rows[1], rows[2]) {
            // 12 zone with 8 - sorted order: (3..7, 8, 12)
            (3, 8, 12) => Some('.'),
            (4, 8, 12) => Some('<'),
            (5, 8, 12) => Some('('),
            (6, 8, 12) => Some('+'),
            (7, 8, 12) => Some('|'),

            // 11 zone with 8 - sorted order: (2..7, 8, 11)
            (2, 8, 11) => Some('!'),
            (3, 8, 11) => Some('$'),
            (4, 8, 11) => Some('*'),
            (5, 8, 11) => Some(')'),
            (6, 8, 11) => Some(';'),
            (7, 8, 11) => Some('¬'),

            // 0 zone with 8 - sorted order: (0, 3..7, 8)
            (0, 3, 8) => Some(','),
            (0, 4, 8) => Some('%'),
            (0, 5, 8) => Some('_'),
            (0, 6, 8) => Some('>'),
            (0, 7, 8) => Some('?'),

            _ => None,
        };
    }

    // Unknown pattern
    None
}

/// Encode a string into Hollerith punch patterns
///
/// Returns a vector of HollerithCode for each character
/// Unsupported characters are replaced with a space (blank)
pub fn encode_string(s: &str) -> Vec<HollerithCode> {
    s.chars()
        .map(|c| char_to_hollerith(c.to_ascii_uppercase()).unwrap_or_else(HollerithCode::empty))
        .collect()
}

/// Decode Hollerith punch patterns into a string
///
/// Invalid patterns are replaced with '?' character
pub fn decode_string(codes: &[HollerithCode]) -> String {
    codes
        .iter()
        .map(|code| hollerith_to_char(code).unwrap_or('?'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hollerith_code_new() {
        let code = HollerithCode::new(vec![12, 1]);
        assert_eq!(code.rows, vec![1, 12]);
    }

    #[test]
    fn test_hollerith_code_dedup() {
        let code = HollerithCode::new(vec![12, 1, 12, 1]);
        assert_eq!(code.rows, vec![1, 12]);
    }

    #[test]
    fn test_hollerith_code_empty() {
        let code = HollerithCode::empty();
        assert!(code.rows.is_empty());
    }

    #[test]
    fn test_is_punched() {
        let code = HollerithCode::new(vec![12, 1]);
        assert!(code.is_punched(12));
        assert!(code.is_punched(1));
        assert!(!code.is_punched(2));
    }

    #[test]
    fn test_as_array() {
        let code = HollerithCode::new(vec![12, 1]);
        let arr = code.as_array();
        assert!(arr[0]); // row 12
        assert!(arr[3]); // row 1
        assert!(!arr[4]); // row 2
    }

    #[test]
    fn test_from_array() {
        let mut arr = [false; 12];
        arr[0] = true; // row 12
        arr[3] = true; // row 1
        let code = HollerithCode::from_array(arr);
        assert_eq!(code.rows, vec![1, 12]);
    }

    #[test]
    fn test_char_to_hollerith_digits() {
        assert_eq!(char_to_hollerith('0').unwrap().rows, vec![0]);
        assert_eq!(char_to_hollerith('1').unwrap().rows, vec![1]);
        assert_eq!(char_to_hollerith('9').unwrap().rows, vec![9]);
    }

    #[test]
    fn test_char_to_hollerith_letters_a_i() {
        assert_eq!(char_to_hollerith('A').unwrap().rows, vec![1, 12]);
        assert_eq!(char_to_hollerith('E').unwrap().rows, vec![5, 12]);
        assert_eq!(char_to_hollerith('I').unwrap().rows, vec![9, 12]);
    }

    #[test]
    fn test_char_to_hollerith_letters_j_r() {
        assert_eq!(char_to_hollerith('J').unwrap().rows, vec![1, 11]);
        assert_eq!(char_to_hollerith('M').unwrap().rows, vec![4, 11]);
        assert_eq!(char_to_hollerith('R').unwrap().rows, vec![9, 11]);
    }

    #[test]
    fn test_char_to_hollerith_letters_s_z() {
        assert_eq!(char_to_hollerith('S').unwrap().rows, vec![0, 2]);
        assert_eq!(char_to_hollerith('V').unwrap().rows, vec![0, 5]);
        assert_eq!(char_to_hollerith('Z').unwrap().rows, vec![0, 9]);
    }

    #[test]
    fn test_char_to_hollerith_special() {
        assert_eq!(char_to_hollerith(' ').unwrap().rows, vec![]);
        assert_eq!(char_to_hollerith('&').unwrap().rows, vec![12]);
        assert_eq!(char_to_hollerith('-').unwrap().rows, vec![11]);
        assert_eq!(char_to_hollerith('/').unwrap().rows, vec![0, 1]);
    }

    #[test]
    fn test_char_to_hollerith_with_8_punch() {
        assert_eq!(char_to_hollerith('.').unwrap().rows, vec![3, 8, 12]);
        assert_eq!(char_to_hollerith('(').unwrap().rows, vec![5, 8, 12]);
        assert_eq!(char_to_hollerith('*').unwrap().rows, vec![4, 8, 11]);
        assert_eq!(char_to_hollerith(',').unwrap().rows, vec![0, 3, 8]);
    }

    #[test]
    fn test_char_to_hollerith_unsupported() {
        assert!(char_to_hollerith('~').is_none());
        assert!(char_to_hollerith('£').is_none());
    }

    #[test]
    fn test_hollerith_to_char_digits() {
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![0])).unwrap(),
            '0'
        );
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![5])).unwrap(),
            '5'
        );
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![9])).unwrap(),
            '9'
        );
    }

    #[test]
    fn test_hollerith_to_char_letters() {
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![12, 1])).unwrap(),
            'A'
        );
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![11, 4])).unwrap(),
            'M'
        );
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![0, 9])).unwrap(),
            'Z'
        );
    }

    #[test]
    fn test_hollerith_to_char_space() {
        assert_eq!(hollerith_to_char(&HollerithCode::empty()).unwrap(), ' ');
    }

    #[test]
    fn test_hollerith_to_char_special() {
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![12, 3, 8])).unwrap(),
            '.'
        );
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![11, 4, 8])).unwrap(),
            '*'
        );
        assert_eq!(
            hollerith_to_char(&HollerithCode::new(vec![0, 3, 8])).unwrap(),
            ','
        );
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = "HELLO WORLD 123";
        let encoded = encode_string(original);
        let decoded = decode_string(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_encode_string_lowercase() {
        let encoded = encode_string("hello");
        let decoded = decode_string(&encoded);
        assert_eq!(decoded, "HELLO");
    }

    #[test]
    fn test_encode_string_unsupported_chars() {
        let encoded = encode_string("A~B");
        let decoded = decode_string(&encoded);
        assert_eq!(decoded, "A B"); // Unsupported char becomes space
    }

    #[test]
    fn test_decode_invalid_pattern() {
        let invalid_code = HollerithCode::new(vec![12, 11, 0]); // Invalid combination
        assert_eq!(hollerith_to_char(&invalid_code), None);
    }
}
