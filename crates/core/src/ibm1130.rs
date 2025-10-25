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
/// This would be a binary card representing compiled machine code
pub fn generate_example_object() -> PunchCard {
    // Create a binary card with example data
    // In reality, this would be actual machine code from the assembler
    let example_data = vec![
        0xC0, 0x00, 0x10, 0x00, // Example: Load instruction
        0x48, 0x00, 0x20, 0x00, // Example: Add instruction
        0x50, 0x00, 0x30, 0x00, // Example: Store instruction
        0x00, 0x00, 0x00, 0x00, // Padding
    ];

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
