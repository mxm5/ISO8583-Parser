//! # String Manipulation Module
//!
//! This module provides utilities for string manipulation.
//!
//! ## Examples
//!
//! ```
//! use emv_parser::string_manipulation::StringManipulation;
//!
//! let mut s = String::from("48656C6C6F2C576F726C64"); // "Hello, World" in hex
//!
//! // Convert hex to ASCII
//! let ascii_result = s.hex_to_ascii();
//! assert_eq!(ascii_result.unwrap(), "Hello,World");
//! 
//! // Get a slice of the string until a specified length
//! let slice = s.get_slice_until(5);
//! assert_eq!(slice, "48656");
//! 
//! // Get another slice of the string until a specified length
//! let slice = s.get_slice_until(5);
//! assert_eq!(slice, "C6C6F");
//! 
//! // Process a field based on field number, length, and name
//! s.process_field(1, 12, "test");
//!
//! use emv_parser::string_manipulation::positions_of_set_bits;
//!
//! let bitmap: Vec<u32> = positions_of_set_bits(u64::from_str_radix("3038058020C19201", 16).unwrap());
//! assert_eq!(bitmap, vec![3, 4, 11, 12, 13, 22, 24, 25, 35, 41, 42, 48, 49, 52, 55, 64]);
//! ```

pub mod string_manipulation {
    use emv_tlv_parser::parse_tlv;

    /// Returns the positions of set bits in a binary number.
    pub fn positions_of_set_bits(n: u64) -> Vec<u32> {
        (0..64).filter(|&bit| 1 & (n >> (63 - bit)) != 0).map(|bit| bit + 1).collect()
    }
    
    /// Trait for string manipulation operations.
    pub trait StringManipulation {
        /// Get a slice of the string until a specified length.
        fn get_slice_until(&mut self, length: usize) -> String;

        /// Convert a hex string to ASCII.
        fn hex_to_ascii(&mut self) -> Result<String, hex::FromHexError>;

        /// Process a field based on field number, length, and name.
        fn process_field(&mut self, field_number: u32, length: u32, name: &str);
    }

    impl StringManipulation for String {
        /// Get a slice of the string until a specified length.
        fn get_slice_until(&mut self, length: usize) -> String {
            self.drain(..length).collect::<String>()
        }

        /// Convert a hex string to ASCII.
        fn hex_to_ascii(&mut self) -> Result<String, hex::FromHexError> {
            let hex_bytes = hex::decode(self)?;
            let ascii_chars: String = hex_bytes.iter().map(|&byte| byte as char).collect();
            Ok(ascii_chars)
        }

        /// Process a field based on field number, length, and name.
        fn process_field(&mut self, field_number: u32, length: u32, name: &str) {
            let mut field_value = if field_number == 35 {
                self.get_slice_until(38 as usize)
            } else {
                self.get_slice_until(length as usize)
            };

            let value_to_print = if matches!(field_number, 41 | 42 | 49 | 62) {
                field_value.hex_to_ascii().unwrap()
            } else {
                field_value.to_string()
            };

            println!("Field {:3} | {:12} | Length: {:3} | {}", field_number, name, length, value_to_print);
            
            if field_number == 55 {
                match parse_tlv(value_to_print) {
                    Ok(tags) => tags.iter().for_each(|tag| println!("{}", tag)),
                    Err(e) => eprintln!("Error parsing TLV: {}", e),
                }
            }
        }
    }
}