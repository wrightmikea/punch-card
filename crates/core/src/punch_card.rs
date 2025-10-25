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

    /// Create a binary card from raw bytes (80 or 160 bytes)
    ///
    /// Supports two formats:
    /// - 80 bytes: Each byte encodes 8 punch rows (first 8 positions), rows 8-11 are lost (lossy)
    /// - 160 bytes: Two bytes per column encode all 12 punch rows (full fidelity)
    ///
    /// Array layout: [12, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    pub fn from_binary(data: &[u8]) -> Self {
        let mut card = PunchCard::new(CardType::Binary);

        if data.len() >= 160 {
            // 160-byte format: 2 bytes per column, all 12 rows
            for i in 0..80 {
                let idx = i * 2;
                let byte1 = data[idx];
                let byte2 = if idx + 1 < data.len() {
                    data[idx + 1]
                } else {
                    0
                };
                let word = ((byte2 as u16) << 8) | (byte1 as u16);

                // Convert bit pattern to array format
                // bit 0 -> index 0 (row 12), bit 1 -> index 1 (row 11), ..., bit 11 -> index 11 (row 9)
                let punch_array = (0..12)
                    .map(|bit| (word & (1 << bit)) != 0)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                card.columns[i] = Column::from_hollerith(HollerithCode::from_array(punch_array));
            }
        } else {
            // 80-byte format: 1 byte per column, only first 8 array positions (legacy/lossy)
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

    /// Convert the card to binary format (160 bytes = 2 bytes per column)
    /// Preserves all 12 punch rows with full fidelity
    ///
    /// Array layout: [12, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// Bit mapping: bit 0 = row 12, bit 1 = row 11, bit 2 = row 0, ..., bit 11 = row 9
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(160);

        for column in &self.columns {
            let punches = column.punches.as_array();
            let mut word: u16 = 0;

            // Encode all 12 array positions as bits 0-11
            for (bit, &is_punched) in punches.iter().enumerate() {
                if is_punched {
                    word |= 1 << bit;
                }
            }

            // Store as little-endian (byte1 = low byte, byte2 = high byte)
            data.push((word & 0xFF) as u8);
            data.push(((word >> 8) & 0xFF) as u8);
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
}
