// Punch Card Module
//
// Data structures and operations for IBM punch cards

use crate::ebcdic::{ebcdic_to_hollerith, hollerith_to_ebcdic};
use crate::hollerith::{HollerithCode, char_to_hollerith, hollerith_to_char};
use serde::{Deserialize, Serialize};

/// Represents a single column on a punch card
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
    /// The punch pattern for this column
    pub punches: HollerithCode,
    /// The printed character at the top (IBM 029 feature)
    /// None for binary cards or blank columns
    pub printed_char: Option<char>,
}

impl Column {
    /// Create a new blank column
    pub fn new() -> Self {
        Column {
            punches: HollerithCode::empty(),
            printed_char: None,
        }
    }

    /// Create a column from a character (encodes and sets printed char)
    pub fn from_char(c: char) -> Self {
        let upper_c = c.to_ascii_uppercase();
        Column {
            punches: char_to_hollerith(upper_c).unwrap_or_else(HollerithCode::empty),
            printed_char: Some(upper_c),
        }
    }

    /// Create a column from a Hollerith code (binary mode, no printing)
    pub fn from_hollerith(code: HollerithCode) -> Self {
        Column {
            punches: code,
            printed_char: None,
        }
    }

    /// Get the character representation of this column
    pub fn to_char(&self) -> Option<char> {
        hollerith_to_char(&self.punches)
    }

    /// Check if this column is blank (no punches)
    pub fn is_blank(&self) -> bool {
        self.punches.rows.is_empty()
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of punch card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    /// Text card with character printing (IBM 029 mode)
    Text,
    /// Binary card without character printing (object deck)
    Binary,
}

/// Represents a complete 80-column punch card
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PunchCard {
    /// The 80 columns of the card
    columns: Vec<Column>,
    /// The type of card (text or binary)
    card_type: CardType,
}

impl PunchCard {
    /// Create a new blank punch card
    pub fn new(card_type: CardType) -> Self {
        PunchCard {
            columns: vec![Column::new(); 80],
            card_type,
        }
    }

    /// Create a text card from a string (max 80 characters)
    pub fn from_text(text: &str) -> Self {
        let mut card = PunchCard::new(CardType::Text);
        for (i, c) in text.chars().take(80).enumerate() {
            card.columns[i] = Column::from_char(c);
        }
        card
    }

    /// Create a card from raw bytes
    ///
    /// Supports two formats:
    /// - 108 bytes: IBM 1130 binary format (72 columns × 12 rows = 864 bits)
    ///   Columns 73-80 are left blank (not included in binary data)
    /// - 80 bytes: Legacy format, 1 byte per column (only 8 bits, lossy)
    ///
    /// Array layout: [12, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    pub fn from_binary(data: &[u8]) -> Self {
        let mut card = PunchCard::new(CardType::Binary);

        if data.len() == 108 {
            // IBM 1130 binary format: 108 bytes = 864 bits for columns 1-72
            // Unpack 108 bytes into 864 bits (72 columns × 12 rows each)
            let mut bit_idx = 0;
            for col_idx in 0..72 {
                let mut punch_array = [false; 12];
                for punch in &mut punch_array {
                    let byte_idx = bit_idx / 8;
                    let bit_in_byte = bit_idx % 8;
                    if byte_idx < data.len() {
                        *punch = (data[byte_idx] & (1 << bit_in_byte)) != 0;
                    }
                    bit_idx += 1;
                }
                card.columns[col_idx] =
                    Column::from_hollerith(HollerithCode::from_array(punch_array));
            }
            // Columns 73-80 remain blank (default Column::new())
        } else {
            // Legacy 80-byte format: 1 byte per column, only first 8 array positions (lossy)
            for (i, &byte) in data.iter().take(80).enumerate() {
                let mut punch_array = [false; 12];
                for (bit, punch) in punch_array.iter_mut().enumerate().take(8) {
                    *punch = (byte & (1 << bit)) != 0;
                }
                card.columns[i] = Column::from_hollerith(HollerithCode::from_array(punch_array));
            }
        }
        card
    }

    /// Convert the card to IBM 1130 binary format (108 bytes)
    ///
    /// IBM 1130 binary format:
    /// - Saves only columns 1-72 (72 columns × 12 rows = 864 bits = 108 bytes)
    /// - Columns 73-80 are not saved (typically used for sequence numbers on physical cards)
    /// - This matches the authentic IBM 1130 assembler object deck disk file format
    ///
    /// Array layout: [12, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(108);

        // Pack 72 columns × 12 rows = 864 bits into 108 bytes
        let mut bit_buffer: Vec<bool> = Vec::with_capacity(864);
        for i in 0..72 {
            let punches = self.columns[i].punches.as_array();
            for &is_punched in punches.iter() {
                bit_buffer.push(is_punched);
            }
        }

        // Convert bits to bytes (8 bits per byte)
        for byte_idx in 0..108 {
            let mut byte_val: u8 = 0;
            for bit_in_byte in 0..8 {
                let bit_idx = byte_idx * 8 + bit_in_byte;
                if bit_idx < bit_buffer.len() && bit_buffer[bit_idx] {
                    byte_val |= 1 << bit_in_byte;
                }
            }
            data.push(byte_val);
        }

        data
    }

    /// Convert the card to EBCDIC format (80 bytes = 1 byte per column)
    /// Standard format for IBM punch card data interchange
    ///
    /// Each column's Hollerith punch pattern is converted to its EBCDIC character code
    pub fn to_ebcdic(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(80);

        for column in &self.columns {
            let ebcdic_byte = hollerith_to_ebcdic(&column.punches);
            data.push(ebcdic_byte);
        }

        data
    }

    /// Create a card from EBCDIC format (80 bytes = 1 byte per column)
    ///
    /// Each byte is an EBCDIC character code that is converted to its Hollerith punch pattern
    pub fn from_ebcdic(data: &[u8]) -> Self {
        let mut card = PunchCard::new(CardType::Text);

        for (i, &ebcdic_byte) in data.iter().take(80).enumerate() {
            let hollerith = ebcdic_to_hollerith(ebcdic_byte);
            // Determine the printed character from the EBCDIC code
            let printed_char = match ebcdic_byte {
                0x40 => Some(' '),
                0xF0..=0xF9 => Some((b'0' + (ebcdic_byte - 0xF0)) as char),
                0xC1..=0xC9 => Some((b'A' + (ebcdic_byte - 0xC1)) as char),
                0xD1..=0xD9 => Some((b'J' + (ebcdic_byte - 0xD1)) as char),
                0xE2..=0xE9 => Some((b'S' + (ebcdic_byte - 0xE2)) as char),
                _ => None,
            };
            card.columns[i] = Column {
                punches: hollerith,
                printed_char,
            };
        }

        card
    }

    /// Get the card type
    pub fn card_type(&self) -> CardType {
        self.card_type
    }

    /// Get a reference to a column
    pub fn get_column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Get a mutable reference to a column
    pub fn get_column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Set a column from a character (text mode)
    pub fn set_column_char(&mut self, index: usize, c: char) -> Result<(), &'static str> {
        if index >= 80 {
            return Err("Column index out of range");
        }
        self.columns[index] = Column::from_char(c);
        Ok(())
    }

    /// Set a column from a Hollerith code (binary mode)
    pub fn set_column_hollerith(
        &mut self,
        index: usize,
        code: HollerithCode,
    ) -> Result<(), &'static str> {
        if index >= 80 {
            return Err("Column index out of range");
        }
        self.columns[index] = Column::from_hollerith(code);
        Ok(())
    }

    /// Clear a column (make it blank)
    pub fn clear_column(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= 80 {
            return Err("Column index out of range");
        }
        self.columns[index] = Column::new();
        Ok(())
    }

    /// Clear the entire card
    pub fn clear(&mut self) {
        for col in &mut self.columns {
            *col = Column::new();
        }
    }

    /// Convert the card to a text string
    /// Returns the text representation of all columns
    pub fn to_text(&self) -> String {
        self.columns
            .iter()
            .map(|col| col.to_char().unwrap_or('?'))
            .collect()
    }

    /// Get the number of punched columns (non-blank)
    pub fn punched_count(&self) -> usize {
        self.columns.iter().filter(|col| !col.is_blank()).count()
    }

    /// Get all columns as a slice
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }
}

impl Default for PunchCard {
    fn default() -> Self {
        Self::new(CardType::Text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_new() {
        let col = Column::new();
        assert!(col.is_blank());
        assert_eq!(col.printed_char, None);
    }

    #[test]
    fn test_column_from_char() {
        let col = Column::from_char('A');
        assert!(!col.is_blank());
        assert_eq!(col.printed_char, Some('A'));
        assert_eq!(col.to_char(), Some('A'));
    }

    #[test]
    fn test_column_from_char_lowercase() {
        let col = Column::from_char('a');
        assert_eq!(col.printed_char, Some('A'));
        assert_eq!(col.to_char(), Some('A'));
    }

    #[test]
    fn test_column_from_hollerith() {
        let code = HollerithCode::new(vec![12, 1]);
        let col = Column::from_hollerith(code);
        assert_eq!(col.printed_char, None);
        assert_eq!(col.to_char(), Some('A'));
    }

    #[test]
    fn test_punch_card_new() {
        let card = PunchCard::new(CardType::Text);
        assert_eq!(card.card_type(), CardType::Text);
        assert_eq!(card.punched_count(), 0);
    }

    #[test]
    fn test_punch_card_from_text() {
        let card = PunchCard::from_text("HELLO");
        assert_eq!(card.card_type(), CardType::Text);
        assert_eq!(card.punched_count(), 5);
        assert_eq!(card.get_column(0).unwrap().to_char(), Some('H'));
        assert_eq!(card.get_column(4).unwrap().to_char(), Some('O'));
    }

    #[test]
    fn test_punch_card_from_text_max_80() {
        let long_text = "A".repeat(100);
        let card = PunchCard::from_text(&long_text);
        assert_eq!(card.punched_count(), 80);
    }

    #[test]
    fn test_punch_card_from_binary() {
        let data = vec![0b10101010, 0b01010101];
        let card = PunchCard::from_binary(&data);
        assert_eq!(card.card_type(), CardType::Binary);
        assert!(card.punched_count() > 0);

        // Check that first column has punches from the byte pattern
        let col = card.get_column(0).unwrap();
        assert!(!col.is_blank());
    }

    #[test]
    fn test_set_column_char() {
        let mut card = PunchCard::new(CardType::Text);
        card.set_column_char(0, 'A').unwrap();
        assert_eq!(card.get_column(0).unwrap().to_char(), Some('A'));
    }

    #[test]
    fn test_set_column_char_out_of_range() {
        let mut card = PunchCard::new(CardType::Text);
        assert!(card.set_column_char(80, 'A').is_err());
    }

    #[test]
    fn test_clear_column() {
        let mut card = PunchCard::from_text("HELLO");
        card.clear_column(0).unwrap();
        assert!(card.get_column(0).unwrap().is_blank());
        assert_eq!(card.punched_count(), 4);
    }

    #[test]
    fn test_clear_card() {
        let mut card = PunchCard::from_text("HELLO");
        card.clear();
        assert_eq!(card.punched_count(), 0);
    }

    #[test]
    fn test_clear_text_card_completely() {
        // Create a full 80-column text card
        let mut card = PunchCard::from_text(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGH",
        );
        assert_eq!(card.punched_count(), 80);

        // Clear it
        card.clear();

        // Verify all columns are blank
        assert_eq!(card.punched_count(), 0);
        for i in 0..80 {
            assert!(
                card.get_column(i).unwrap().is_blank(),
                "Column {} should be blank after clear",
                i
            );
        }
    }

    #[test]
    fn test_clear_binary_card_completely() {
        // Create a binary card with data in all 72 columns
        let mut binary_data = Vec::with_capacity(108);
        let mut bit_buffer: Vec<bool> = Vec::with_capacity(864);
        for _i in 0..72 {
            let pattern = 0x0FFF; // All 12 bits set
            for bit in 0..12 {
                bit_buffer.push((pattern & (1 << bit)) != 0);
            }
        }
        for byte_idx in 0..108 {
            let mut byte_val: u8 = 0;
            for bit_in_byte in 0..8 {
                let bit_idx = byte_idx * 8 + bit_in_byte;
                if bit_idx < bit_buffer.len() && bit_buffer[bit_idx] {
                    byte_val |= 1 << bit_in_byte;
                }
            }
            binary_data.push(byte_val);
        }

        let mut card = PunchCard::from_binary(&binary_data);
        assert_eq!(card.punched_count(), 72);

        // Clear it
        card.clear();

        // Verify all columns are blank
        assert_eq!(card.punched_count(), 0);
        for i in 0..80 {
            assert!(
                card.get_column(i).unwrap().is_blank(),
                "Column {} should be blank after clear",
                i
            );
        }
    }

    #[test]
    fn test_to_text() {
        let card = PunchCard::from_text("HELLO WORLD");
        let text = card.to_text();
        assert!(text.starts_with("HELLO WORLD"));
    }

    #[test]
    fn test_get_column_mut() {
        let mut card = PunchCard::new(CardType::Text);
        if let Some(col) = card.get_column_mut(0) {
            *col = Column::from_char('Z');
        }
        assert_eq!(card.get_column(0).unwrap().to_char(), Some('Z'));
    }

    #[test]
    fn test_text_card_save_load_roundtrip() {
        // Test A: 80-column text card round-trip
        // Create a text card with 80 columns of alphanumeric data
        let original_card = PunchCard::from_text(
            "HELLO WORLD TEST 1234567890 ABCDEFGHIJKLMNOPQRSTUVWXYZ MORE DATA TO FILL 80",
        );

        // Save to binary format (108 bytes for IBM 1130 format)
        // Note: Only columns 1-72 are saved, columns 73-80 are NOT saved
        let saved_data = original_card.to_binary();
        assert_eq!(saved_data.len(), 108);

        // Clear and load from binary format
        let loaded_card = PunchCard::from_binary(&saved_data);

        // Verify the card type
        assert_eq!(loaded_card.card_type(), CardType::Binary); // from_binary creates Binary type

        // Check column-by-column punch patterns match for columns 1-72
        for i in 0..72 {
            let orig_col = original_card.get_column(i).unwrap();
            let loaded_col = loaded_card.get_column(i).unwrap();
            assert_eq!(
                orig_col.punches, loaded_col.punches,
                "Column {} punch pattern mismatch",
                i
            );
        }

        // Columns 73-80 should be blank after reload (not saved in binary format)
        for i in 72..80 {
            let loaded_col = loaded_card.get_column(i).unwrap();
            assert!(
                loaded_col.is_blank(),
                "Column {} should be blank after load (not saved in 108-byte format)",
                i
            );
        }
    }

    #[test]
    fn test_binary_card_save_load_roundtrip() {
        // Test B: 72-column binary card round-trip
        // Create a binary card with only 72 columns of data (columns 73-80 blank)
        let mut binary_data = Vec::with_capacity(108);

        // Pack 72 columns × 12 bits each into 108 bytes
        let mut bit_buffer: Vec<bool> = Vec::with_capacity(864);
        for i in 0..72 {
            let pattern = 0x0E49 | (i as u16);
            for bit in 0..12 {
                bit_buffer.push((pattern & (1 << bit)) != 0);
            }
        }

        // Convert bits to bytes
        for byte_idx in 0..108 {
            let mut byte_val: u8 = 0;
            for bit_in_byte in 0..8 {
                let bit_idx = byte_idx * 8 + bit_in_byte;
                if bit_idx < bit_buffer.len() && bit_buffer[bit_idx] {
                    byte_val |= 1 << bit_in_byte;
                }
            }
            binary_data.push(byte_val);
        }

        let original_card = PunchCard::from_binary(&binary_data);

        // Save to binary format (108 bytes)
        let saved_data = original_card.to_binary();
        assert_eq!(saved_data.len(), 108);

        // Clear and load from binary format
        let loaded_card = PunchCard::from_binary(&saved_data);

        // Verify the cards are identical
        assert_eq!(loaded_card.card_type(), CardType::Binary);

        // Check column-by-column punch patterns match for all 80 columns
        // Columns 1-72 should have data, columns 73-80 should be blank
        for i in 0..80 {
            let orig_col = original_card.get_column(i).unwrap();
            let loaded_col = loaded_card.get_column(i).unwrap();
            assert_eq!(
                orig_col.punches, loaded_col.punches,
                "Column {} punch pattern mismatch",
                i
            );

            // Verify columns 73-80 are blank
            if i >= 72 {
                assert!(
                    orig_col.is_blank(),
                    "Column {} should be blank in original",
                    i
                );
                assert!(
                    loaded_col.is_blank(),
                    "Column {} should be blank after load",
                    i
                );
            }
        }
    }
}
