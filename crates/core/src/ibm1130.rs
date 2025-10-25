// IBM 1130 Format Module
//
// Specific format handling for IBM 1130 assembler source and object deck cards

use crate::punch_card::{CardType, PunchCard};

/// Generate an example IBM 1130 assembler source card
///
/// Format:
/// - Column 1-5: Label (optional)
/// - Column 6: Blank or continuation
/// - Column 7-10: Opcode
/// - Column 11-80: Operands and comments
pub fn generate_example_source() -> PunchCard {
    // Example: A simple IBM 1130 assembler instruction
    //          START DC   1
    // Or:      LOOP  LD   X
    PunchCard::from_text("START DC   0             IBM 1130 EXAMPLE PROGRAM")
}

/// Generate an example IBM 1130 object deck card
///
/// IBM 1130 binary format:
/// - Columns 1-72: Binary machine code (all 12 rows used for dense data encoding)
/// - Columns 73-80: Left blank (on physical cards these held sequence numbers)
/// - File format: 108 bytes (72 columns × 12 rows = 864 bits)
///
/// Binary cards show dense punch patterns across all rows, representing compiled
/// machine code that an assembler would produce when punching object decks.
pub fn generate_example_object() -> PunchCard {
    let mut example_data = Vec::with_capacity(108);

    // Create 72 columns of 12-bit punch patterns
    // Pack into 108 bytes (864 bits total)

    // Pattern inspired by actual binary object cards - varied punch patterns
    // using all 12 rows to create realistic machine code appearance
    let punch_patterns: Vec<u16> = vec![
        0x0E49, 0x0C31, 0x0842, 0x0421, 0x0E73, 0x0C52, 0x0946, 0x0735, 0x0E5A, 0x0C48, 0x08E3,
        0x0467, 0x0F21, 0x0D84, 0x0B42, 0x09C6, 0x0E87, 0x0C39, 0x0A51, 0x0763, 0x0E94, 0x0CB2,
        0x0856, 0x0429, 0x0F48, 0x0D31, 0x0B82, 0x0974, 0x0EC5, 0x0CA3, 0x0A61, 0x0847, 0x0E29,
        0x0C74, 0x08B5, 0x0493, 0x0F52, 0x0DB1, 0x0B73, 0x0965, 0x0E38, 0x0C91, 0x0A42, 0x0826,
        0x0F64, 0x0DC8, 0x0B51, 0x0937, 0x0EA7, 0x0C52, 0x0984, 0x0763, 0x0E41, 0x0CB5, 0x0A29,
        0x0876, 0x0F93, 0x0D42, 0x0BC6, 0x0948, 0x0E72, 0x0CA4, 0x0851, 0x0639, 0x0F28, 0x0DB7,
        0x0B94, 0x0962, 0x0E56, 0x0C83, 0x0A41, 0x0725,
    ];

    // Pack 72 12-bit patterns into 108 bytes
    let mut bit_buffer: Vec<bool> = Vec::with_capacity(864);
    for pattern in punch_patterns {
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
        example_data.push(byte_val);
    }

    PunchCard::from_binary(&example_data)
}

/// Validate IBM 1130 source card format
///
/// Checks if the card follows basic IBM 1130 assembler conventions
pub fn validate_source_format(card: &PunchCard) -> Result<(), String> {
    if card.card_type() != CardType::Text {
        return Err("Source cards must be text type".to_string());
    }

    // Additional validation could check:
    // - Label field (columns 1-5)
    // - Blank/continuation (column 6)
    // - Opcode field (columns 7-10)
    // For now, just check it's a text card

    Ok(())
}

/// Validate IBM 1130 object deck format
///
/// Checks if the card follows binary card conventions
pub fn validate_object_format(card: &PunchCard) -> Result<(), String> {
    if card.card_type() != CardType::Binary {
        return Err("Object cards must be binary type".to_string());
    }

    // Check that we have punches (not a blank card)
    if card.punched_count() == 0 {
        return Err("Object card cannot be blank".to_string());
    }

    Ok(())
}

/// Common IBM 1130 opcodes for reference
#[allow(dead_code)]
pub mod opcodes {
    pub const LD: &str = "LD"; // Load Accumulator
    pub const STO: &str = "STO"; // Store Accumulator
    pub const ADD: &str = "ADD"; // Add to Accumulator
    pub const SUB: &str = "SUB"; // Subtract from Accumulator
    pub const MPY: &str = "MPY"; // Multiply
    pub const DIV: &str = "DIV"; // Divide
    pub const BSC: &str = "BSC"; // Branch or Skip Conditional
    pub const DC: &str = "DC"; // Define Constant
    pub const DSA: &str = "DSA"; // Define Storage Area
    pub const END: &str = "END"; // End of Assembly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_example_source() {
        let card = generate_example_source();
        assert_eq!(card.card_type(), CardType::Text);
        assert!(card.punched_count() > 0);

        // Check that it contains expected content
        let text = card.to_text();
        assert!(text.contains("START"));
    }

    #[test]
    fn test_generate_example_object() {
        let card = generate_example_object();
        assert_eq!(card.card_type(), CardType::Binary);
        assert!(card.punched_count() > 0);
    }

    #[test]
    fn test_validate_source_format_valid() {
        let card = PunchCard::from_text("LOOP  LD   X");
        assert!(validate_source_format(&card).is_ok());
    }

    #[test]
    fn test_validate_source_format_invalid_type() {
        let card = PunchCard::from_binary(&[0x00]);
        assert!(validate_source_format(&card).is_err());
    }

    #[test]
    fn test_validate_object_format_valid() {
        let card = PunchCard::from_binary(&[0xC0, 0x00]);
        assert!(validate_object_format(&card).is_ok());
    }

    #[test]
    fn test_validate_object_format_invalid_type() {
        let card = PunchCard::from_text("HELLO");
        assert!(validate_object_format(&card).is_err());
    }

    #[test]
    fn test_validate_object_format_blank() {
        let card = PunchCard::new(CardType::Binary);
        assert!(validate_object_format(&card).is_err());
    }
}
