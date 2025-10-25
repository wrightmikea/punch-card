// IBM 1130 Punch Card Simulator - Core Library
//
// This library provides the core functionality for simulating IBM punch cards,
// including Hollerith encoding, punch card data structures, and IBM 1130 format support.

pub mod ebcdic;
pub mod hollerith;
pub mod ibm1130;
pub mod punch_card;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
